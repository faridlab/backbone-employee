//! Consumer for the `promotion.effective` compound event — role side (ADR-005).
//!
//! The employee module owns the APPLY side of the promotion's role change: on each
//! `promotion.effective` envelope it appends an `employment_histories` row capturing the
//! position/level/department move, **idempotently**. This handler is registered on the integration
//! bus in backbone-hr-app's `main.rs` alongside the payroll `PromotionSalaryHandler` (both subscribe
//! to `promotion.effective`; each dedups independently via its own inbox consumer name).
//!
//! ## Idempotency
//!
//! The relay is at-least-once, so this handler MUST be idempotent. It uses the framework's
//! [`backbone_outbox::inbox::once`]: the `(consumer, event_id)` claim and the employment_history
//! INSERT run in ONE transaction and commit together. The `event_id` is the bus envelope id, which
//! the relay preserves from the outbox row's id — so dedup keys end-to-end. A redelivery re-runs
//! `inbox::once`, which returns `false`, so the insert is skipped.
//!
//! As defense-in-depth, `employment_histories.reference_id` is set to the `promotion_id` (the
//! idempotency link per ADR-005), so even a bug that bypassed the inbox would leave an audit trail
//! tying the row back to the source workflow.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use async_trait::async_trait;
use backbone_messaging::{EventError, IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::inbox;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

/// The consumer name stamped into the employee inbox. The ADR-005 idempotency key for this target is
/// `("promotion.role", promotion_id)`; the `promotion_id` arrives as the envelope id (preserved from
/// the outbox row id), so the consumer name carries the first half.
const CONSUMER: &str = "promotion.role";

/// Integration-event handler that turns a `promotion.effective` envelope into an `employment_histories`
/// row, idempotently. Holds only the pool — the apply is plain SQL inside an `inbox`-guarded
/// transaction.
pub struct PromotionEffectiveHandler {
    pool: PgPool,
}

impl PromotionEffectiveHandler {
    /// Create a new handler bound to the given pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntegrationEventHandler for PromotionEffectiveHandler {
    async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
        // The envelope id IS the outbox row's id (the relay preserves it) → the dedup key.
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| handler_err(format!("bad envelope id '{}': {e}", envelope.id)))?;

        let p = &envelope.payload;
        let company_id: Uuid = json_field(p, "company_id")?;
        let employee_id: Uuid = json_field(p, "employee_id")?;
        let promotion_id: Option<Uuid> = serde_json::from_value(p["promotion_id"].clone()).ok();
        let position_id_from: Option<Uuid> = serde_json::from_value(p["position_id_from"].clone()).ok();
        let position_id_to: Option<Uuid> = serde_json::from_value(p["position_id_to"].clone()).ok();
        let level_id_from: Option<Uuid> = serde_json::from_value(p["level_id_from"].clone()).ok();
        let level_id_to: Option<Uuid> = serde_json::from_value(p["level_id_to"].clone()).ok();
        let department_id_from: Option<Uuid> = serde_json::from_value(p["department_id_from"].clone()).ok();
        let department_id_to: Option<Uuid> = serde_json::from_value(p["department_id_to"].clone()).ok();
        let effective_date: NaiveDate = json_field(p, "effective_date")?;

        let mut tx = self.pool.begin().await.map_err(map_db)?;

        // The relay's connection crosses tenants only on the outbox tables — every domain table
        // sits behind the strict company fence. Bind the event's company (from the payload) before
        // any statement so the insert passes the fence's WITH CHECK.
        backbone_orm::company_scope::bind_company_on(&mut tx, company_id)
            .await
            .map_err(|e| handler_err(format!("company bind: {e}")))?;

        // Claim the event in-tx with the effect: the inbox row + the history insert commit together
        // (or roll back together). A failed apply re-claims on the next delivery; a successful apply
        // never re-applies — exactly-once effect over at-least-once delivery.
        let first_time = inbox::once(&mut *tx, "employee", CONSUMER, event_id)
            .await
            .map_err(|e| handler_err(format!("inbox claim: {e}")))?;

        if first_time {
            // employment_histories.action is a Postgres enum; the producer's promotion_type maps onto
            // the 'promotion'/'transfer'/'demotion' variants, defaulting to 'promotion' for the
            // generic 'lateral' case (no exact variant). reference_id = promotion_id is the
            // non-null idempotency link back to the source workflow.
            let action = match serde_json::from_value::<String>(p["promotion_type"].clone())
                .ok()
                .as_deref()
            {
                Some("transfer") => "transfer",
                Some("demotion") => "demotion",
                _ => "promotion",
            };

            sqlx::query(
                r#"INSERT INTO employee.employment_histories
                       (company_id, employee_id, effective_date, action, position_id_from,
                        position_id_to, level_id_from, level_id_to, department_id_from,
                        department_id_to, reference_id)
                   VALUES ($1, $2, $3, $4::employment_action, $5, $6, $7, $8, $9, $10, $11)"#,
            )
            .bind(company_id)
            .bind(employee_id)
            .bind(effective_date)
            .bind(action)
            .bind(position_id_from)
            .bind(position_id_to)
            .bind(level_id_from)
            .bind(level_id_to)
            .bind(department_id_from)
            .bind(department_id_to)
            .bind(promotion_id)
            .execute(&mut *tx)
            .await
            .map_err(map_db)?;
        }

        tx.commit().await.map_err(map_db)?;
        Ok(())
    }

    fn event_patterns(&self) -> Vec<&'static str> {
        // Exact-match the producer's PROMOTION_EFFECTIVE_EVENT_TYPE. The relay builds the envelope
        // with event_type = the outbox row's event_type ("promotion.effective").
        vec!["promotion.effective"]
    }

    fn name(&self) -> &'static str {
        "PromotionEffectiveHandler"
    }
}

/// Decode a required payload field, mapping any failure to a handler error.
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
