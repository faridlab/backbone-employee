//! Consumer for the `recruitment.hired` compound event (ADR-005).
//!
//! The employee module owns the APPLY side of the first compound event: on each `recruitment.hired`
//! envelope it creates the `Employee` (people master) + `Employment` (placement) rows from the offer
//! payload, **idempotently**. This handler is registered on the integration bus in backbone-hr-app's
//! `main.rs` (where the outbox relay drains `recruitment.outbox_events` onto the bus).
//!
//! ## Idempotency
//!
//! The relay is at-least-once, so this handler MUST be idempotent. It uses the framework's
//! [`backbone_outbox::inbox::once`]: the `(consumer, event_id)` claim and the Employee/Employment
//! inserts run in ONE transaction and commit together. The `event_id` is the bus envelope id, which
//! the relay preserves from the outbox row's id — so dedup keys end-to-end. A redelivery re-runs
//! `inbox::once`, which returns `false`, so the inserts are skipped and the handler returns `Ok(())`.
//!
//! As defense-in-depth, `employee_number` is derived deterministically from the `offer_id`
//! (`REC-{offer_id}`), so even a bug that bypassed the inbox would collide on the per-company unique
//! index `idx_employees_company_id_employee_number` rather than silently duplicate.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use async_trait::async_trait;
use backbone_messaging::{EventError, IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::inbox;
use chrono::NaiveDate;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// The consumer name stamped into the employee inbox. Scoped so multiple employee consumers (future)
/// each process the same event exactly once.
const CONSUMER: &str = "recruitment.hired";

/// Integration-event handler that turns a `recruitment.hired` envelope into an `Employee` +
/// `Employment`, idempotently. Holds only the pool — the apply is plain SQL inside an `inbox`-guarded
/// transaction, so it needs no service-layer wiring (and ties the dedup + the inserts atomically,
/// which a GenericCrudService `.create()` on its own connection could not).
pub struct RecruitmentHiredHandler {
    pool: PgPool,
}

impl RecruitmentHiredHandler {
    /// Create a new handler bound to the given pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntegrationEventHandler for RecruitmentHiredHandler {
    async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
        // The envelope id IS the outbox row's id (the relay preserves it) → the dedup key.
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| handler_err(format!("bad envelope id '{}': {e}", envelope.id)))?;

        let p = &envelope.payload;
        let company_id: Uuid = json_field(p, "company_id")?;
        let first_name: String = json_field(p, "first_name")?;
        let last_name: Option<String> = serde_json::from_value(p["last_name"].clone()).ok();
        let email: Option<String> = serde_json::from_value(p["email"].clone()).ok();
        let employment_type: Option<String> = serde_json::from_value(p["employment_type"].clone()).ok();
        let position_id: Option<Uuid> = serde_json::from_value(p["position_id"].clone()).ok();
        let department_id: Option<Uuid> = serde_json::from_value(p["department_id"].clone()).ok();
        let offer_id: Option<Uuid> = serde_json::from_value(p["offer_id"].clone()).ok();
        // `join_date` is carried as an ISO date string; NaiveDate deserializes straight off it.
        let join_date: NaiveDate = json_field(p, "join_date")?;

        // Deterministic employee_number from the offer → a replay yields the SAME number, so even
        // without the inbox the per-company unique index would fence a duplicate (defense in depth).
        // 40-char budget: "REC-" + 36-char uuid = 40.
        let employee_number = match offer_id {
            Some(id) => format!("REC-{id}"),
            None => format!("REC-{event_id}"),
        };
        debug_assert!(
            employee_number.len() <= 40,
            "employee_number derived from a uuid fits the 40-char column budget"
        );

        let mut tx = self.pool.begin().await.map_err(map_db)?;

        // The relay's connection crosses tenants only on the outbox tables — every domain table
        // sits behind the strict company fence. Bind the event's company (from the payload) before
        // any statement so the INSERTs pass the fence's WITH CHECK.
        backbone_orm::company_scope::bind_company_on(&mut tx, company_id)
            .await
            .map_err(|e| handler_err(format!("company bind: {e}")))?;

        // Claim the event in-tx with the effect: the inbox row + the employee/employment inserts commit
        // together (or roll back together). A failed apply thus re-claims on the next delivery and a
        // successful apply never re-applies — exactly-once effect over at-least-once delivery.
        let first_time = inbox::once(&mut *tx, "employee", CONSUMER, event_id)
            .await
            .map_err(|e| handler_err(format!("inbox claim: {e}")))?;

        if first_time {
            // Map the free-text employment_type onto the employment_status enum (default: permanent).
            let employment_status = match employment_type.as_deref() {
                Some("contract") => "contract",
                Some("probation") => "probation",
                Some("associate") => "associate",
                _ => "permanent",
            };

            // Employee (people master). metadata + id are left to column defaults; the audit trigger
            // (in the real schema) stamps created_at/updated_at.
            let employee_id: Uuid = sqlx::query(
                r#"INSERT INTO employee.employees
                       (company_id, employee_number, first_name, last_name, email)
                   VALUES ($1, $2, $3, $4, $5)
                   RETURNING id"#,
            )
            .bind(company_id)
            .bind(&employee_number)
            .bind(&first_name)
            .bind(last_name.as_deref())
            .bind(email.as_deref())
            .fetch_one(&mut *tx)
            .await
            .map(|r| r.get::<Uuid, _>("id"))
            .map_err(map_db)?;

            // Employment (placement) — the offer's position/department become the employee's initial
            // assignment. Cast text → employment_status enum on the Postgres side.
            sqlx::query(
                r#"INSERT INTO employee.employments
                       (company_id, employee_id, employment_status, join_date, position_id, department_id)
                   VALUES ($1, $2, $3::employment_status, $4, $5, $6)"#,
            )
            .bind(company_id)
            .bind(employee_id)
            .bind(employment_status)
            .bind(join_date)
            .bind(position_id)
            .bind(department_id)
            .execute(&mut *tx)
            .await
            .map_err(map_db)?;
        }

        tx.commit().await.map_err(map_db)?;
        Ok(())
    }

    fn event_patterns(&self) -> Vec<&'static str> {
        // Exact-match the producer's HIRED_EVENT_TYPE. The relay builds the envelope with
        // event_type = the outbox row's event_type ("recruitment.hired").
        vec!["recruitment.hired"]
    }

    fn name(&self) -> &'static str {
        "RecruitmentHiredHandler"
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
