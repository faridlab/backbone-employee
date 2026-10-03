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
//! As defense-in-depth, the employee's **id** is derived deterministically from the `offer_id`
//! ([`hired_employee_id`]), so even a bug that bypassed the inbox would collide on the primary key
//! rather than silently duplicate. The same derivation is how the other consumers of the event
//! (the composing service's onboarding and initial-compensation legs) find the employee this
//! handler created, without the event having to carry it.
//!
//! ## What the hire writes
//!
//! - `employee_number`: the next number in the company's sequence ([`allocate_employee_number`]),
//!   in the format the composer's [`EmployeeNumberFormatSource`] answers (default `E000001`).
//! - `employments.join_date`: the hire's first day ([`hire_first_day`]) — the offer's proposed
//!   start date when the event carries one, the day of the hire otherwise.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use std::sync::Arc;

use async_trait::async_trait;
use backbone_messaging::{EventError, IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::inbox;
use chrono::NaiveDate;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::employee_numbering::{
    allocate_employee_number, DefaultEmployeeNumberFormat, EmployeeNumberFormatSource,
};

/// The consumer name stamped into the employee inbox. Scoped so multiple employee consumers (future)
/// each process the same event exactly once.
const CONSUMER: &str = "recruitment.hired";

/// The UUIDv5 namespace a hired employee's id is derived in (see [`hired_employee_id`]). Fixed
/// forever: changing it would make the event's consumers disagree on which employee a hire made.
const HIRED_EMPLOYEE_NAMESPACE: Uuid = Uuid::from_u128(0x6a1d_3c0e_8f4b_5e21_9b7a_2d4c_6e8f_0a13);

/// The id of the employee a hire creates, derived from the offer it was hired from.
///
/// Every consumer of `recruitment.hired` computes the same id from the same offer, so the
/// onboarding and initial-compensation legs reach the new employee without a lookup by any
/// mutable field, and a replay that slipped past the inbox collides on the primary key.
pub fn hired_employee_id(offer_id: Uuid) -> Uuid {
    Uuid::new_v5(&HIRED_EMPLOYEE_NAMESPACE, offer_id.as_bytes())
}

/// [`hired_employee_id`] for a `recruitment.hired` envelope: keyed on the payload's `offer_id`,
/// or on the envelope id for a hire that names no offer. `None` only for a malformed envelope.
pub fn hired_employee_id_for(envelope: &IntegrationEventEnvelope) -> Option<Uuid> {
    serde_json::from_value::<Option<Uuid>>(envelope.payload["offer_id"].clone())
        .ok()
        .flatten()
        .or_else(|| Uuid::parse_str(&envelope.id).ok())
        .map(hired_employee_id)
}

/// The hire's first day: the offer's proposed `start_date` when the event carries one, else the
/// `join_date` the producer stamps (the day of the hire). `None` when neither parses.
pub fn hire_first_day(payload: &serde_json::Value) -> Option<NaiveDate> {
    let date = |key: &str| {
        serde_json::from_value::<Option<NaiveDate>>(payload[key].clone())
            .ok()
            .flatten()
    };
    date("start_date").or_else(|| date("join_date"))
}

/// Integration-event handler that turns a `recruitment.hired` envelope into an `Employee` +
/// `Employment`, idempotently. Holds the pool and the employee-number format port — the apply is
/// plain SQL inside an `inbox`-guarded transaction, so it needs no service-layer wiring (and ties
/// the dedup + the inserts atomically, which a GenericCrudService `.create()` on its own
/// connection could not).
pub struct RecruitmentHiredHandler {
    pool: PgPool,
    numbering: Arc<dyn EmployeeNumberFormatSource>,
}

impl RecruitmentHiredHandler {
    /// The database this consumer writes on: the relay binds the tenant's
    /// pool as the request pool for the whole consumer call (ADR-0029 pool
    /// law); the composed pool is the fallback.
    fn rpool(&self) -> sqlx::PgPool {
        crate::request_pool::current().unwrap_or_else(|| self.pool.clone())
    }

    /// Create a new handler bound to the given pool, numbering hires in the default format.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            numbering: Arc::new(DefaultEmployeeNumberFormat),
        }
    }

    /// Take the employee-number format from `source` (the composer's settings) instead of the
    /// default.
    pub fn with_number_format(mut self, source: Arc<dyn EmployeeNumberFormatSource>) -> Self {
        self.numbering = source;
        self
    }
}

#[async_trait]
impl IntegrationEventHandler for RecruitmentHiredHandler {
    async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
        // The envelope id IS the outbox row's id (the relay preserves it) → the dedup key.
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| handler_err(format!("bad envelope id '{}': {e}", envelope.id)))?;

        let p = &envelope.payload;
        let first_name: String = json_field(p, "first_name")?;
        let last_name: Option<String> = serde_json::from_value(p["last_name"].clone()).ok();
        let email: Option<String> = serde_json::from_value(p["email"].clone()).ok();
        let employment_type: Option<String> = serde_json::from_value(p["employment_type"].clone()).ok();
        let position_id: Option<Uuid> = serde_json::from_value(p["position_id"].clone()).ok();
        let department_id: Option<Uuid> = serde_json::from_value(p["department_id"].clone()).ok();
        let offer_id: Option<Uuid> = serde_json::from_value(p["offer_id"].clone()).ok();
        let candidate_id: Option<Uuid> = serde_json::from_value(p["candidate_id"].clone()).ok();
        // The offer's negotiated gross, carried as a decimal STRING (the
        // producer serializes Numeric through its Display) — parse to the
        // column's NUMERIC(18,2) bind.
        let proposed_salary: Option<rust_decimal::Decimal> =
            serde_json::from_value::<Option<String>>(p["proposed_salary"].clone())
                .ok()
                .flatten()
                .and_then(|s| s.parse::<rust_decimal::Decimal>().ok());
        // The first day: the offer's proposed start date when given, else the hire day the
        // producer stamps as `join_date` (both ISO date strings).
        let join_date: NaiveDate = match hire_first_day(p) {
            Some(d) => d,
            None => json_field(p, "join_date")?,
        };
        // The payload's owning company leg: relay deliveries carry no
        // ambient org scope, and the composing decorator's org-unit fill
        // reads one — without a scope bound here the employee INSERT dies
        // on the fill's kind guard. Bind the payload's unit before the tx.
        let payload_company: Option<Uuid> =
            serde_json::from_value(p["company_id"].clone()).ok();

        // The id is derived from the offer (see `hired_employee_id`); the number is allocated
        // inside the transaction below, once the claim says this is the first delivery.
        let employee_id = hired_employee_id(offer_id.unwrap_or(event_id));
        let number_format = self.numbering.employee_number_format().await;

        let mut tx = self.rpool().begin().await.map_err(map_db)?;

        // Tenancy posture (ADR-0029): the module owns no scoping column — the composing
        // service's tenancy decorator does. Relay the AMBIENT org scope onto this transaction
        // when the caller bound one; a RELAY delivery has none, so fall back to the
        // payload's owning company leg (the hire knows whose tenant it is — fail closed
        // when the event names neither).
        let scope = backbone_orm::org_scope::current_org_scope().or_else(|| {
            payload_company.map(backbone_orm::org_scope::OrgScope::for_company_unit)
        });
        let scope = scope.ok_or_else(|| {
            handler_err("no ambient org scope and no payload company_id — cannot place the hire".into())
        })?;
        backbone_orm::org_scope::bind_org_scope_on(&mut tx, &scope)
            .await
            .map_err(|e| handler_err(format!("org scope bind: {e}")))?;

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

            // The next number in the company's sequence, under the advisory lock this
            // transaction holds until the employee row commits.
            let employee_number = allocate_employee_number(&mut *tx, &number_format)
                .await
                .map_err(map_db)?;

            // Employee (people master). metadata is left to its column default; the audit
            // trigger (in the real schema) stamps created_at/updated_at. The offered salary rides
            // along as base_salary — the employee-master copy of the pay the hire was offered.
            let employee_id: Uuid = sqlx::query(
                r#"INSERT INTO employee.employees
                       (id, employee_number, first_name, last_name, email, candidate_id, base_salary)
                   VALUES ($1, $2, $3, $4, $5, $6, $7)
                   RETURNING id"#,
            )
            .bind(employee_id)
            .bind(&employee_number)
            .bind(&first_name)
            .bind(last_name.as_deref())
            .bind(email.as_deref())
            .bind(candidate_id)
            .bind(proposed_salary)
            .fetch_one(&mut *tx)
            .await
            .map(|r| r.get::<Uuid, _>("id"))
            .map_err(map_db)?;

            // Employment (placement) — the offer's position/department become the employee's initial
            // assignment. Cast text → employment_status enum on the Postgres side.
            sqlx::query(
                r#"INSERT INTO employee.employments
                       (employee_id, employment_status, join_date, position_id, department_id)
                   VALUES ($1, $2::employment_status, $3, $4, $5)"#,
            )
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
