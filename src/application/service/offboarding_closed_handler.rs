//! Consumer for the `offboarding.closed` compound event — role side (ADR-005).
//!
//! The employee module owns the APPLY side of the offboarding-close: on each `offboarding.closed`
//! envelope it flips the leaver's `employments.status` to `inactive`, **idempotently**. Registered on
//! the integration bus in backbone-hr-app's `main.rs` alongside the payroll `OffboardingSettlementHandler`.
//!
//! ## Idempotency
//!
//! The relay is at-least-once, so this handler MUST be idempotent — and a status UPDATE is naturally
//! idempotent. It additionally wraps the UPDATE in [`backbone_outbox::inbox::once`]: the
//! `(consumer, event_id)` claim and the UPDATE run in ONE transaction and commit together, so a
//! redelivery is a pure no-op (the inbox returns `false` and the UPDATE is skipped).
//!
//! `date_of_exit` is stamped from the payload's `last_working_day` (the offboarding producer
//! carries it from the Offboarding row), so the placement records both the deactivation and
//! when it took effect.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use async_trait::async_trait;
use backbone_messaging::{EventError, IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::inbox;
use sqlx::PgPool;
use uuid::Uuid;

/// The consumer name stamped into the employee inbox. The ADR-005 idempotency key for this target is
/// `("offboarding.role", offboarding_id)`; the `offboarding_id` arrives as the envelope id (preserved
/// from the outbox row id).
const CONSUMER: &str = "offboarding.role";

/// Integration-event handler that deactivates the leaver's employment on `offboarding.closed`,
/// idempotently. Holds only the pool — the apply is one UPDATE inside an `inbox`-guarded transaction.
pub struct OffboardingClosedHandler {
    pool: PgPool,
}

impl OffboardingClosedHandler {
    /// Create a new handler bound to the given pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntegrationEventHandler for OffboardingClosedHandler {
    async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
        // The envelope id IS the outbox row's id (the relay preserves it) → the dedup key.
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| handler_err(format!("bad envelope id '{}': {e}", envelope.id)))?;

        let p = &envelope.payload;
        let company_id: Uuid = json_field(p, "company_id")?;
        let employee_id: Uuid = json_field(p, "employee_id")?;
        let last_working_day: Option<chrono::NaiveDate> = p
            .get("last_working_day")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

        let mut tx = self.pool.begin().await.map_err(map_db)?;

        // The relay's connection crosses tenants only on the outbox tables — every domain table
        // sits behind the strict company fence. Bind the event's company (from the payload) before
        // any statement so the UPDATE reaches the leaver's row instead of silently matching zero.
        backbone_orm::company_scope::bind_company_on(&mut tx, company_id)
            .await
            .map_err(|e| handler_err(format!("company bind: {e}")))?;

        // Claim the event in-tx with the effect: the inbox row + the status UPDATE commit together.
        let first_time = inbox::once(&mut *tx, "employee", CONSUMER, event_id)
            .await
            .map_err(|e| handler_err(format!("inbox claim: {e}")))?;

        if first_time {
            // Deactivate the leaver's placement + stamp date_of_exit from the payload's
            // last_working_day (the offboarding producer carries it from the Offboarding row).
            sqlx::query(
                r#"UPDATE employee.employments
                      SET status = 'inactive', date_of_exit = $3
                    WHERE company_id = $1 AND employee_id = $2"#,
            )
            .bind(company_id)
            .bind(employee_id)
            .bind(last_working_day)
            .execute(&mut *tx)
            .await
            .map_err(map_db)?;
        }

        tx.commit().await.map_err(map_db)?;
        Ok(())
    }

    fn event_patterns(&self) -> Vec<&'static str> {
        vec!["offboarding.closed"]
    }

    fn name(&self) -> &'static str {
        "OffboardingClosedHandler"
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
