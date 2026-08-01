//! Consumer for the `onboarding.completed` compound event (ADR-005).
//!
//! The employee module owns the APPLY side of the onboarding-completion: on each
//! `onboarding.completed` envelope it flips the joiner's `employments.status` to `active`,
//! **idempotently**. Registered on the integration bus in backbone-hr-app's `main.rs`.
//!
//! ## Idempotency
//!
//! The relay is at-least-once, so this handler MUST be idempotent — and a status UPDATE is naturally
//! idempotent (re-applying `status='active'` is a no-op). It additionally wraps the UPDATE in
//! [`backbone_outbox::inbox::once`]: the `(consumer, event_id)` claim and the UPDATE run in ONE
//! transaction and commit together, so a redelivery is a pure no-op (the inbox returns `false` and
//! the UPDATE is skipped). Defense-in-depth: even a bug that bypassed the inbox would only re-write
//! the same `active` value.
//!
//! Payroll enrollment (salary structure + BPJS) is a separate, complex target and is intentionally
//! NOT wired here — see ADR-005 TODO. The producer emits the event regardless.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use async_trait::async_trait;
use backbone_messaging::{EventError, IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::inbox;
use sqlx::PgPool;
use uuid::Uuid;

/// The consumer name stamped into the employee inbox. The ADR-005 idempotency key for this target is
/// `("onboarding.active", onboarding_id)`; the `onboarding_id` arrives as the envelope id (preserved
/// from the outbox row id).
const CONSUMER: &str = "onboarding.active";

/// Integration-event handler that activates the joiner's employment on `onboarding.completed`,
/// idempotently. Holds only the pool — the apply is one UPDATE inside an `inbox`-guarded transaction.
pub struct OnboardingCompletedHandler {
    pool: PgPool,
}

impl OnboardingCompletedHandler {
    /// Create a new handler bound to the given pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntegrationEventHandler for OnboardingCompletedHandler {
    async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
        // The envelope id IS the outbox row's id (the relay preserves it) → the dedup key.
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| handler_err(format!("bad envelope id '{}': {e}", envelope.id)))?;

        let p = &envelope.payload;
        let employee_id: Uuid = json_field(p, "employee_id")?;

        let mut tx = self.pool.begin().await.map_err(map_db)?;

        // Claim the event in-tx with the effect: the inbox row + the status UPDATE commit together
        // (or roll back together).
        let first_time = inbox::once(&mut *tx, "employee", CONSUMER, event_id)
            .await
            .map_err(|e| handler_err(format!("inbox claim: {e}")))?;

        if first_time {
            // Flip the joiner's placement to active. `status` is the employment_state enum
            // ('active'/'inactive'); the cast on the Postgres side keeps this text→enum safe. The
            // UPDATE is itself idempotent (re-writing 'active' is a no-op) — the inbox claim is the
            // mandatory backstop that suppresses even the redundant UPDATE on redelivery.
            sqlx::query(
                r#"UPDATE employee.employments
                      SET status = 'active'
                    WHERE employee_id = $1"#,
            )
            .bind(employee_id)
            .execute(&mut *tx)
            .await
            .map_err(map_db)?;
        }

        tx.commit().await.map_err(map_db)?;
        Ok(())
    }

    fn event_patterns(&self) -> Vec<&'static str> {
        vec!["onboarding.completed"]
    }

    fn name(&self) -> &'static str {
        "OnboardingCompletedHandler"
    }
}

fn json_field<T>(p: &serde_json::Value, field: &str) -> Result<T, EventError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(p[field].clone())
        .map_err(|e| handler_err(format!("payload.{field}: {e}")))
}

fn map_db(e: sqlx::Error) -> EventError {
    handler_err(format!("db: {e}"))
}

fn handler_err(message: String) -> EventError {
    EventError::handler(CONSUMER, message)
}
