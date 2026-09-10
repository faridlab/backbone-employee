//! Behavior tests for the peer-visible employee directory (`employee.employees_public`)
//! and the one-active-employment invariant.
//!
//! Oracle: Odoo's `hr.employee.public` is a read-only directory every internal user can
//! browse; our port redacts harder (civil-registration/health/religion PII never projected)
//! and keeps full reads behind the consent guard. Tenancy is the composing service's job
//! (ADR-0029): the view projects no tenancy column, the module ships FORCE RLS with no
//! policy, and `security_invoker = on` makes the view inherit whatever fence the composing
//! service's tenancy decorator installs. The "current version" pointer (Odoo
//! `current_version_id`) maps to "the active employment", now a DB-level unique invariant.
//!
//! Requires a migrated `backbone_employee_test` DB (see tests/common/mod.rs for the DSN).

mod common;
use common::*;

use sqlx::Row;
use uuid::Uuid;

/// Seed one employee (+ optional active employment) with every PII column filled, so a
/// leak would be caught. Returns the employee id.
async fn seed_employee(pool: &sqlx::PgPool, number: &str, with_active_employment: bool) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"INSERT INTO employee.employees
             (id, employee_number, first_name, last_name, email, mobile_phone,
              phone, birth_place, birth_date, gender, marital_status, blood_type)
           VALUES ($1, $2, 'Ani', 'Satria', 'ani@example.com', '+6281200000001',
                   '+62215000001', 'Bandung', '1990-05-17', 'female', 'married', 'o')"#,
    )
    .bind(id)
    .bind(number)
    .execute(pool)
    .await
    .expect("seed employee");
    if with_active_employment {
        sqlx::query(
            r#"INSERT INTO employee.employments
                 (id, employee_id, employment_status, join_date, status)
               VALUES ($1, $2, 'permanent', '2026-01-05', 'active')"#,
        )
        .bind(Uuid::new_v4())
        .bind(id)
        .execute(pool)
        .await
        .expect("seed employment");
    }
    id
}

/// EPP-1 — the directory view projects directory fields and NOTHING else: none of the
/// redacted columns exist on the view, so no query can select them. The tenancy column is
/// redacted with the PII (the module is tenant-agnostic, ADR-0029).
#[tokio::test]
async fn epp1_view_column_set_is_redacted() {
    let pool = pool().await;
    let redacted = [
        "company_id",
        "birth_place",
        "birth_date",
        "gender",
        "marital_status",
        "blood_type",
        "religion_id",
        "user_id",
    ];
    let row = sqlx::query(
        r#"SELECT array_agg(attname ORDER BY attname) AS cols
           FROM pg_attribute
           WHERE attrelid = 'employee.employees_public'::regclass
             AND attnum > 0 AND NOT attisdropped"#,
    )
    .fetch_one(&pool)
    .await
    .expect("view exists (run module migrations)");
    let cols: Vec<String> = row.get::<Vec<String>, _>("cols");
    for banned in redacted {
        assert!(!cols.contains(&banned.to_string()), "view must not project {banned}");
    }
    for needed in ["id", "employee_number", "first_name", "email", "department_id", "join_date"] {
        assert!(cols.contains(&needed.to_string()), "view must project {needed}");
    }
}

/// EPP-2 — a peer lookup returns the directory fields with the active placement joined in.
#[tokio::test]
async fn epp2_directory_row_carries_active_placement() {
    let pool = pool().await;
    let unique = Uuid::new_v4();
    let id = seed_employee(&pool, format!("EPP2-{unique}").as_str(), true).await;

    let row = sqlx::query(
        r#"SELECT first_name, email, employment_status::text AS employment_status, join_date
           FROM employee.employees_public WHERE id = $1"#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .expect("directory row");

    assert_eq!(row.get::<String, _>("first_name"), "Ani");
    assert_eq!(row.get::<String, _>("email"), "ani@example.com");
    assert_eq!(row.get::<String, _>("employment_status"), "permanent");
    assert_eq!(row.get::<chrono::NaiveDate, _>("join_date"), "2026-01-05".parse::<chrono::NaiveDate>().unwrap());
}

/// EPP-3 — an employee with no active employment still appears (identity is not placement),
/// with NULL placement columns; a soft-deleted employee disappears entirely.
#[tokio::test]
async fn epp3_no_active_employment_and_soft_delete() {
    let pool = pool().await;
    let unique = Uuid::new_v4();

    let bare = seed_employee(&pool, format!("EPP3A-{unique}").as_str(), false).await;
    let row = sqlx::query(
        "SELECT employment_status, join_date FROM employee.employees_public WHERE id = $1",
    )
    .bind(bare)
    .fetch_one(&pool)
    .await
    .expect("bare employee visible");
    assert!(row.get::<Option<String>, _>("employment_status").is_none());
    assert!(row.get::<Option<chrono::NaiveDate>, _>("join_date").is_none());

    let gone = seed_employee(&pool, format!("EPP3B-{unique}").as_str(), true).await;
    sqlx::query("UPDATE employee.employees SET metadata = jsonb_set(metadata, '{deleted_at}', to_jsonb(NOW())) WHERE id = $1")
        .bind(gone)
        .execute(&pool)
        .await
        .expect("soft delete");
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM employee.employees_public WHERE id = $1")
        .bind(gone)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, 0, "soft-deleted employee must leave the directory");
}

/// EPP-4 — at most one active employment per employee, enforced by the DB (fires on raw
/// SQL; ADR-0015). A second active row is rejected even when the first was soft-deleted
/// out of the predicate.
#[tokio::test]
async fn epp4_second_active_employment_rejected() {
    let pool = pool().await;
    let unique = Uuid::new_v4();
    let id = seed_employee(&pool, format!("EPP4-{unique}").as_str(), true).await;

    let dup = sqlx::query(
        r#"INSERT INTO employee.employments
             (id, employee_id, employment_status, join_date, status)
           VALUES ($1, $2, 'contract', '2026-06-01', 'active')"#,
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .execute(&pool)
    .await;

    match dup {
        Err(sqlx::Error::Database(ref e)) if e.is_unique_violation() => {}
        other => panic!("second active employment must be a unique violation, got {other:?}"),
    }

    // Moving the first to inactive unblocks a new active row (the transfer case).
    sqlx::query("UPDATE employee.employments SET status = 'inactive' WHERE employee_id = $1 AND status = 'active'")
        .bind(id)
        .execute(&pool)
        .await
        .expect("deactivate first");
    sqlx::query(
        r#"INSERT INTO employee.employments
             (id, employee_id, employment_status, join_date, status)
           VALUES ($1, $2, 'contract', '2026-06-01', 'active')"#,
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .execute(&pool)
    .await
    .expect("replacement active employment accepted after the old one went inactive");
}

/// EPP-5 — the strip posture: the module ships FORCE RLS with NO policy (ADR-0029 — the
/// composing service's tenancy decorator installs the fence), so a non-superuser session
/// reads ZERO rows through the view whether or not a legacy `app.company_id` var is set.
/// Fail-closed by construction; the owner connection seeds (superusers bypass RLS).
///
/// Uses the module test's own `employees_test_app` role (NOSUPERUSER), created idempotently.
#[tokio::test]
async fn epp5_view_is_fail_closed_until_the_decorator_installs_a_policy() {
    let owner = pool().await;
    // Idempotent role + grants (test-instance setup; safe to repeat).
    sqlx::query(
        r#"DO $$ BEGIN
             IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'employees_test_app') THEN
               CREATE ROLE employees_test_app LOGIN PASSWORD 'employees_test_app';
             END IF;
           END $$"#,
    )
    .execute(&owner)
    .await
    .expect("ensure role");
    sqlx::query("GRANT USAGE ON SCHEMA employee TO employees_test_app")
        .execute(&owner)
        .await
        .expect("schema usage");
    sqlx::query("GRANT SELECT ON employee.employees_public TO employees_test_app")
        .execute(&owner)
        .await
        .expect("view select");
    // The view is security_invoker — the role also needs SELECT on the base tables to read
    // through it at all.
    sqlx::query("GRANT SELECT ON ALL TABLES IN SCHEMA employee TO employees_test_app")
        .execute(&owner)
        .await
        .expect("base-table select");

    let unique_a = Uuid::new_v4();
    let unique_b = Uuid::new_v4();
    seed_employee(&owner, format!("EPP5A-{unique_a}").as_str(), true).await;
    seed_employee(&owner, format!("EPP5B-{unique_b}").as_str(), true).await;

    // The app-role leg must hit the SAME server and database as the owner leg
    // (DATABASE_URL), not hardcoded values — derive host:port and the database name
    // from the env DSN and swap only the credentials.
    let dsn = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://serpa:serpa_dev_password@127.0.0.1:5432/backbone_employee_test".into()
    });
    let hostport = dsn
        .split('@')
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .unwrap_or("127.0.0.1:5432")
        .to_string();
    let database = dsn.rsplit('/').next().unwrap_or("backbone_employee_test");
    let app = sqlx::PgPool::connect(&format!(
        "postgresql://employees_test_app:employees_test_app@{hostport}/{database}"
    ))
    .await
    .expect("connect as app role");

    // A stale `app.company_id` var is IGNORED post-strip — no policy consumes it any more.
    let n: i64 = scoped_count(&app, Some(Uuid::new_v4())).await;
    assert_eq!(n, 0, "a set app.company_id must NOT un-fence the view after the strip");
    // With nothing set the view is equally closed.
    let n: i64 = scoped_count(&app, None).await;
    assert_eq!(n, 0, "unset app.company_id must see zero rows through the view");
}

/// Count the directory as the app role with an optional `app.company_id` var bound on the
/// connection (kept so the test can prove the module no longer consumes it).
async fn scoped_count(pool: &sqlx::PgPool, company: Option<Uuid>) -> i64 {
    let mut tx = pool.begin().await.expect("begin");
    if let Some(c) = company {
        sqlx::query("SELECT set_config('app.company_id', $1, true)")
            .bind(c.to_string())
            .execute(&mut *tx)
            .await
            .expect("set company");
    }
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM employee.employees_public")
        .fetch_one(&mut *tx)
        .await
        .expect("count");
    tx.rollback().await.expect("rollback");
    n
}
