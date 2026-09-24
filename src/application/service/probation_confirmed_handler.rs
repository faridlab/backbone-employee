//! Consumer for the `lifecycle.probation_confirmed` compound event (ADR-005).
//!
//! The employee module owns the APPLY side of probation confirmation: on each
//! `lifecycle.probation_confirmed` envelope it appends an `employment_histories` row
//! (`action='confirmation'`) and flips the joiner's `employments.employment_status` from
//! `probation` to `permanent`, **idempotently**. Registered on the integration bus where the
//! outbox relay drains `lifecycle.outbox_events`.
//!
//! ## Idempotency
//!
//! The relay is at-least-once, so this handler MUST be idempotent. It uses the framework's
//! [`backbone_outbox::inbox::once`]: the `(consumer, event_id)` claim, the history append, and the
//! status CAS run in ONE transaction and commit together. A redelivery re-runs `inbox::once`,
//! which returns `false`, so both effects are skipped.
//!
//! The status flip is a compare-and-swap (`probation` → `permanent`): an employment that is
//! already `permanent` — or was never on probation — matches zero rows, which is a legitimate
//! no-op (logged), not an error. The confirmation is still recorded in the history timeline,
//! because the event fired either way.
//!
//! ## Tenancy
//!
//! The module owns no scoping column (ADR-0029) — a composing service's tenancy decorator
//! installs org scoping for these tables. The handler relays the AMBIENT org scope onto its
//! transaction when the caller bound one; an undecorated deployment has none and skips this.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use async_trait::async_trait;
use backbone_messaging::{EventError, IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::inbox;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

/// The consumer name stamped into the employee inbox. Scoped so multiple employee consumers each
/// process the same event exactly once.
const CONSUMER: &str = "lifecycle.probation_confirmed";

/// Integration-event handler that records a probation confirmation and makes the joiner permanent,
/// idempotently. Holds only the pool — the apply is plain SQL inside an `inbox`-guarded transaction.
pub struct ProbationConfirmedHandler {
    pool: PgPool,
}

impl ProbationConfirmedHandler {
    /// Create a new handler bound to the given pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntegrationEventHandler for ProbationConfirmedHandler {
    async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
        // The envelope id IS the outbox row's id (the relay preserves it) → the dedup key.
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| handler_err(format!("bad envelope id '{}': {e}", envelope.id)))?;

        let p = &envelope.payload;
        let employee_id: Uuid = json_field(p, "employee_id")?;
        let onboarding_id: Option<Uuid> = serde_json::from_value(p["onboarding_id"].clone()).ok();
        let confirmation_date: NaiveDate = json_field(p, "confirmation_date")?;

        let mut tx = self.pool.begin().await.map_err(map_db)?;

        // Tenancy posture (ADR-0029): the module owns no scoping column — the composing
        // service's tenancy decorator does. Relay the AMBIENT org scope onto this transaction
        // when the caller bound one, so the decorator's org-unit fill (and any policy it
        // installed) sees this transaction's statements. An undecorated deployment has no
        // ambient scope and skips this entirely.
        // Relay the ambient request scope when the caller bound one; a
        // RELAY delivery has none, and the composing decorator's org-unit
        // fill reads a scope — fall back to the event's owning company leg
        // (every lifecycle emit carries it) so the writes land fenced.
        let payload_company: Option<Uuid> =
            serde_json::from_value(p["company_id"].clone()).ok();
        let scope = backbone_orm::org_scope::current_org_scope().or_else(|| {
            payload_company.map(backbone_orm::org_scope::OrgScope::for_company_unit)
        });
        if let Some(scope) = scope {
            backbone_orm::org_scope::bind_org_scope_on(&mut tx, &scope)
                .await
                .map_err(|e| handler_err(format!("org scope bind: {e}")))?;
        }

        // Claim the event in-tx with the effect: the inbox row, the history append, and the
        // status CAS commit together (or roll back together).
        let first_time = inbox::once(&mut *tx, "employee", CONSUMER, event_id)
            .await
            .map_err(|e| handler_err(format!("inbox claim: {e}")))?;

        if first_time {
            // 1. Record the milestone in the history timeline. The literal 'confirmation' coerces
            //    to the employment_action enum on the Postgres side; reference_id ties the row
            //    back to the onboarding that produced the event.
            sqlx::query(
                r#"INSERT INTO employee.employment_histories
                       (employee_id, effective_date, action, reference_id, note)
                   VALUES ($1, $2, 'confirmation', $3, $4)"#,
            )
            .bind(employee_id)
            .bind(confirmation_date)
            .bind(onboarding_id)
            .bind("probation confirmed")
            .execute(&mut *tx)
            .await
            .map_err(map_db)?;

            // 2. CAS probation → permanent. Zero affected rows is a logged no-op (already
            //    permanent, or hired straight to permanent without probation).
            let flipped = sqlx::query(
                r#"UPDATE employee.employments
                      SET employment_status = 'permanent'
                    WHERE employee_id = $1 AND employment_status = 'probation'"#,
            )
            .bind(employee_id)
            .execute(&mut *tx)
            .await
            .map_err(map_db)?
            .rows_affected();
            if flipped == 0 {
                tracing::info!(
                    employee = %employee_id,
                    "probation confirmed but employment was not on probation; status left unchanged"
                );
            }
        }

        tx.commit().await.map_err(map_db)?;
        Ok(())
    }

    fn event_patterns(&self) -> Vec<&'static str> {
        // Exact-match the producer's event type (the outbox row's event_type).
        vec!["lifecycle.probation_confirmed"]
    }

    fn name(&self) -> &'static str {
        "ProbationConfirmedHandler"
    }
}

/// Decode a required payload field, mapping any failure to a handler error (so the bus reports a
/// precise malformed-payload message rather than a generic serde blob).
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
