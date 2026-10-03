//! What a hire writes onto the people master: the `recruitment.hired` consumer numbers the new
//! employee from the company's sequence, starts the employment on the offer's proposed first day,
//! and gives the employee the id every other consumer of the event derives from the offer.
//!
//! Each test runs in a PRIVATE scratch database (`employee_hired_test_{suffix}`) created from the
//! server named by `DATABASE_URL` (the role must be allowed to create databases), with just the
//! tables the consumer touches. When `DATABASE_URL`'s server cannot be reached the test skips
//! with a note rather than failing.

use std::sync::Arc;

use async_trait::async_trait;
use backbone_employee::application::service::{
    hire_first_day, hired_employee_id, hired_employee_id_for, EmployeeNumberFormat,
    EmployeeNumberFormatSource, RecruitmentHiredHandler,
};
use backbone_messaging::{IntegrationEventEnvelope, IntegrationEventHandler};
use backbone_outbox::outbox;
use chrono::{NaiveDate, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

async fn connect(suffix: &str) -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://serpa:serpa_dev_password@127.0.0.1:5432/postgres".into());
    let (prefix, _) = url.trim_end_matches('/').rsplit_once('/')?;
    let admin = match PgPool::connect(&format!("{prefix}/postgres")).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("skip recruitment_hired_test: cannot reach `{prefix}/postgres` ({e})");
            return None;
        }
    };
    let scratch = format!("employee_hired_test_{suffix}");
    let _ = sqlx::query(&format!(r#"DROP DATABASE IF EXISTS "{scratch}" WITH (FORCE)"#))
        .execute(&admin)
        .await;
    sqlx::query(&format!(r#"CREATE DATABASE "{scratch}""#))
        .execute(&admin)
        .await
        .expect("create scratch database");
    admin.close().await;
    Some(
        PgPool::connect(&format!("{prefix}/{scratch}"))
            .await
            .expect("connect scratch database"),
    )
}

/// The employee tables as the consumer sees them: `org_unit_id` lands from the acting-unit
/// setting the consumer binds, as the composing service's tenancy decorator installs it.
async fn setup(pool: &PgPool) {
    for stmt in [
        "CREATE TYPE employment_status AS ENUM ('permanent','contract','probation','associate')",
        "CREATE SCHEMA employee",
        r#"CREATE TABLE employee.employees (
               id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
               org_unit_id UUID NOT NULL DEFAULT (NULLIF(current_setting('app.acting_unit_id', true), ''))::uuid,
               employee_number TEXT NOT NULL,
               first_name TEXT NOT NULL,
               last_name TEXT,
               email TEXT,
               candidate_id UUID,
               base_salary NUMERIC(18,2),
               metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
               UNIQUE (org_unit_id, employee_number)
           )"#,
        r#"CREATE TABLE employee.employments (
               id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
               org_unit_id UUID NOT NULL DEFAULT (NULLIF(current_setting('app.acting_unit_id', true), ''))::uuid,
               employee_id UUID NOT NULL,
               employment_status employment_status NOT NULL DEFAULT 'permanent',
               join_date DATE NOT NULL,
               department_id UUID,
               position_id UUID,
               metadata JSONB NOT NULL DEFAULT '{}'::jsonb
           )"#,
    ] {
        sqlx::query(stmt).execute(pool).await.expect("setup ddl");
    }
    outbox::migrate(pool, "employee").await.expect("employee inbox");
}

async fn seed_employee(pool: &PgPool, company: Uuid, number: &str, deleted: bool) {
    let metadata = if deleted {
        json!({ "deleted_at": "2026-01-01T00:00:00Z" })
    } else {
        json!({})
    };
    sqlx::query(
        "INSERT INTO employee.employees (org_unit_id, employee_number, first_name, metadata) \
         VALUES ($1, $2, 'Seeded', $3)",
    )
    .bind(company)
    .bind(number)
    .bind(metadata)
    .execute(pool)
    .await
    .expect("seed employee");
}

fn hired(company: Uuid, offer: Uuid, start_date: Option<&str>) -> IntegrationEventEnvelope {
    let mut payload = json!({
        "offer_id": offer,
        "company_id": company,
        "candidate_id": Uuid::new_v4(),
        "first_name": "Rina",
        "last_name": "Kusuma",
        "email": "rina@example.com",
        "employment_type": "probation",
        "proposed_salary": "9000000",
        "position_id": null,
        "department_id": null,
        "join_date": "2026-10-02",
    });
    if let Some(d) = start_date {
        payload["start_date"] = json!(d);
    }
    IntegrationEventEnvelope {
        id: Uuid::new_v4().to_string(),
        event_type: "recruitment.hired".into(),
        source_context: "JobOffer".into(),
        aggregate_id: offer.to_string(),
        occurred_at: Utc::now(),
        published_at: Utc::now(),
        version: 1,
        correlation_id: None,
        causation_id: None,
        payload,
    }
}

async fn hired_row(pool: &PgPool, offer: Uuid) -> (String, NaiveDate, String) {
    sqlx::query_as(
        r#"SELECT e.employee_number, m.join_date, m.employment_status::text
             FROM employee.employees e
             JOIN employee.employments m ON m.employee_id = e.id
            WHERE e.id = $1"#,
    )
    .bind(hired_employee_id(offer))
    .fetch_one(pool)
    .await
    .expect("the hire's employee + employment, under the id derived from the offer")
}

/// The hire takes the next number in the company's sequence — past soft-deleted holders, and
/// ignoring numbers in another shape (the offer-derived numbers earlier hires were given) — and
/// starts on the offer's proposed first day.
#[tokio::test]
async fn a_hire_is_numbered_from_the_sequence_and_starts_on_the_proposed_day() {
    let Some(pool) = connect("sequence").await else { return };
    setup(&pool).await;
    let company = Uuid::new_v4();
    for n in ["E000001", "E000002", "E000003"] {
        seed_employee(&pool, company, n, false).await;
    }
    seed_employee(&pool, company, "E000004", true).await;
    seed_employee(&pool, company, "REC-3bf7bf48-e855-434c-acc5-51bd4cb9d0ac", false).await;
    seed_employee(&pool, company, "E12-legacy", false).await;

    let offer = Uuid::new_v4();
    let envelope = hired(company, offer, Some("2026-11-02"));
    assert_eq!(hired_employee_id_for(&envelope), Some(hired_employee_id(offer)));
    let handler = RecruitmentHiredHandler::new(pool.clone());
    handler.handle(envelope.clone()).await.expect("hire applies");

    let (number, join_date, status) = hired_row(&pool, offer).await;
    assert_eq!(number, "E000005", "next after the highest issued, soft-deleted included");
    assert_eq!(join_date, NaiveDate::from_ymd_opt(2026, 11, 2).unwrap(), "the proposed first day");
    assert_eq!(status, "probation");

    // A redelivery of the same event is a no-op: no second employee, no number consumed.
    handler.handle(envelope).await.expect("replay is a no-op");
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM employee.employees")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 7);
}

/// Without a proposed start date the employment starts on the hire day the producer stamped.
#[tokio::test]
async fn a_hire_without_a_start_date_starts_on_the_hire_day() {
    let Some(pool) = connect("hireday").await else { return };
    setup(&pool).await;
    let company = Uuid::new_v4();
    let offer = Uuid::new_v4();
    RecruitmentHiredHandler::new(pool.clone())
        .handle(hired(company, offer, None))
        .await
        .expect("hire applies");
    let (number, join_date, _) = hired_row(&pool, offer).await;
    assert_eq!(number, "E000001", "the first number of an empty company");
    assert_eq!(join_date, NaiveDate::from_ymd_opt(2026, 10, 2).unwrap());

    assert_eq!(
        hire_first_day(&json!({ "start_date": null, "join_date": "2026-10-02" })),
        NaiveDate::from_ymd_opt(2026, 10, 2)
    );
    assert_eq!(hire_first_day(&json!({})), None);
}

struct Format(&'static str, usize);

#[async_trait]
impl EmployeeNumberFormatSource for Format {
    async fn employee_number_format(&self) -> EmployeeNumberFormat {
        EmployeeNumberFormat::new(Some(self.0), Some(self.1))
    }
}

/// The format comes from the composer's settings, and hires landing at the same moment each get
/// their own number.
#[tokio::test]
async fn concurrent_hires_get_distinct_numbers_in_the_configured_format() {
    let Some(pool) = connect("concurrent").await else { return };
    setup(&pool).await;
    let company = Uuid::new_v4();
    seed_employee(&pool, company, "HP-0007", false).await;
    seed_employee(&pool, company, "E000900", false).await;

    let handler = Arc::new(
        RecruitmentHiredHandler::new(pool.clone()).with_number_format(Arc::new(Format("HP-", 4))),
    );
    let offers: Vec<Uuid> = (0..4).map(|_| Uuid::new_v4()).collect();
    let runs = offers.iter().map(|offer| {
        let handler = handler.clone();
        let envelope = hired(company, *offer, Some("2026-11-02"));
        async move { handler.handle(envelope).await }
    });
    for outcome in futures_join(runs).await {
        outcome.expect("each concurrent hire applies");
    }

    let mut numbers = Vec::new();
    for offer in &offers {
        numbers.push(hired_row(&pool, *offer).await.0);
    }
    numbers.sort();
    assert_eq!(numbers, ["HP-0008", "HP-0009", "HP-0010", "HP-0011"]);
}

/// Run the futures concurrently on the runtime and collect their outcomes in order.
async fn futures_join<F, T>(futs: impl Iterator<Item = F>) -> Vec<T>
where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let handles: Vec<_> = futs.map(tokio::spawn).collect();
    let mut out = Vec::new();
    for h in handles {
        out.push(h.await.expect("task"));
    }
    out
}
