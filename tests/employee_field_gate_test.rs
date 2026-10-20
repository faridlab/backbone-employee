//! The field-level permission gate on Employee's confidential fields
//! (`base_salary`, `mobile_phone`, `birth_date` require `employee:read_confidential`
//! — ADR-0033 second amendment, declared in `schema/models/employee.model.yaml`).
//!
//! Proves, against the module's own guarded generic routes driven in-process:
//!
//! - a non-holder's read answers the three fields as `null` — the key stays, the
//!   value goes — both on the employee's own row and on the row an `?include=`
//!   from another entity of this module expands (the same seam a cross-module
//!   include goes through: the registry is keyed by table);
//! - a holder (or a `*:*` caller) sees the values;
//! - a non-holder's filter naming a gated field is refused `400 invalid_query`
//!   reason `field_permission` (a holder may ask);
//! - a non-holder's write naming a gated field is refused `422
//!   field_permission_required` naming the permission, while a write that does
//!   not name it is untouched;
//! - the declared people-search set (`employee_number, first_name, last_name,
//!   email`) cannot probe a gated field: `?search=` by the phone number finds
//!   nothing, by the given name finds the row.
//!
//! Each run uses a PRIVATE scratch database (`employee_gate_test_{suffix}`)
//! created from the server named by `DATABASE_URL` (the role must be allowed to
//! create databases) and dropped at the end; the module's migrations are applied
//! to it.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use tower::ServiceExt;
use uuid::Uuid;

use backbone_auth::middleware::AuthContext;
use backbone_employee::application::service::{EmployeeService, EmploymentService};
use backbone_employee::infrastructure::persistence::{EmployeeRepository, EmploymentRepository};
use backbone_employee::presentation::http::{create_employee_routes, create_employment_routes};

/// A caller holding exactly `permissions`.
fn caller(perms: &[&str]) -> AuthContext {
    let mut ctx = AuthContext::new("00000000-0000-0000-0000-00000000aa01".to_string());
    ctx.permissions = perms.iter().map(|p| p.to_string()).collect();
    ctx
}

/// Drive `router` with `ctx` as the request's caller; answer `(status, body)`.
async fn send(router: &axum::Router, method: &str, path: &str, body: Option<Value>, ctx: &AuthContext) -> (StatusCode, Value) {
    let builder = Request::builder().method(method).uri(path).extension(ctx.clone());
    let req = match body {
        Some(b) => builder.header("content-type", "application/json").body(Body::from(b.to_string())).unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    let resp = router.clone().oneshot(req).await.expect("router answers");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.expect("body");
    let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

/// The first violation of a refused answer, as `(code, params)`.
fn first_violation(body: &Value) -> (String, Value) {
    let v = body["violations"][0].as_object().expect("a violations array");
    (
        v["code"].as_str().unwrap_or_default().to_string(),
        v["params"].clone(),
    )
}

#[tokio::test]
async fn the_confidential_fields_are_permission_gated() {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://serpa:serpa_dev_password@127.0.0.1:5432/postgres".into());
    let (prefix, _) = url.trim_end_matches('/').rsplit_once('/').expect("a database in DATABASE_URL");
    let dbname = format!("employee_gate_test_{}", Uuid::new_v4().simple());
    let admin = sqlx::PgPool::connect(&format!("{prefix}/postgres")).await.expect("reach the server named by DATABASE_URL");
    sqlx::query(&format!("CREATE DATABASE {dbname}")).execute(&admin).await.expect("create the scratch database");

    let pool = sqlx::PgPool::connect(&format!("{prefix}/{dbname}")).await.expect("connect the scratch database");
    // The module's audit triggers call the composed backbone-auditlog module's shared capture
    // function — its migrations are the composer's, not this module's. Stub the function as a
    // no-op so the module's own migration set runs on a module-only scratch database (the gate
    // under test lives in the HTTP seam, not in the audit trail; the seed migrations skip
    // themselves with a notice when the identity tables are absent).
    sqlx::query(r#"CREATE SCHEMA IF NOT EXISTS auditlog"#).execute(&pool).await.expect("auditlog schema");
    sqlx::query(
        r#"CREATE FUNCTION auditlog.capture_data_change() RETURNS trigger AS $fn$
             BEGIN RETURN NULL; END
           $fn$ LANGUAGE plpgsql"#,
    )
    .execute(&pool)
    .await
    .expect("auditlog capture stub");
    sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
        .await
        .expect("read the module's migrations")
        .run(&pool)
        .await
        .expect("apply the module's migrations");

    // One employee with every gated column set, so a leak would be caught.
    let id = Uuid::new_v4();
    sqlx::query(
        r#"INSERT INTO employee.employees
             (id, employee_number, first_name, last_name, email, base_salary, mobile_phone,
              phone, birth_place, birth_date, gender, marital_status, blood_type)
           VALUES ($1, $2, 'Ani', 'Satria', 'ani@example.com', 1234500.00, '+6281234567890',
                   '+62215000001', 'Bandung', '1990-05-17', 'female', 'married', 'o')"#,
    )
    .bind(id)
    .bind(format!("GATE-{}", Uuid::new_v4().simple()))
    .execute(&pool)
    .await
    .expect("seed employee");
    sqlx::query(
        r#"INSERT INTO employee.employments
             (id, employee_id, employment_status, join_date, status)
           VALUES ($1, $2, 'permanent', '2026-01-05', 'active')"#,
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .execute(&pool)
    .await
    .expect("seed employment");

    // Both guarded routers: building the employee one also registers its gated
    // fields under its table, which is what the include expansion redacts against.
    let employees = create_employee_routes(std::sync::Arc::new(EmployeeService::with_repository(
        std::sync::Arc::new(EmployeeRepository::new(pool.clone())),
    )));
    let employments = create_employment_routes(std::sync::Arc::new(EmploymentService::with_repository(
        std::sync::Arc::new(EmploymentRepository::new(pool.clone())),
    )));

    let non_holder = caller(&["employee:read"]);
    let holder = caller(&["employee:read", "employee:read_confidential"]);
    let wildcard = caller(&["*:*"]);
    let row_path = format!("/employees/{id}");

    // ── Reads: the non-holder's own row answers nulls; the key stays. ──
    let (status, body) = send(&employees, "GET", &row_path, None, &non_holder).await;
    assert_eq!(status, StatusCode::OK, "non-holder reads the row: {body}");
    assert_eq!(body["data"]["firstName"], json!("Ani"), "ungated fields stay readable: {body}");
    for key in ["baseSalary", "mobilePhone", "birthDate"] {
        assert!(body["data"].as_object().unwrap().contains_key(key), "the gated key {key} stays: {body}");
        assert!(body["data"][key].is_null(), "the gated {key} is null for a non-holder: {body}");
    }

    // A holder sees the values.
    let (status, body) = send(&employees, "GET", &row_path, None, &holder).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"]["baseSalary"].as_str().unwrap_or("").contains("1234500"), "holder sees the salary: {body}");
    assert_eq!(body["data"]["mobilePhone"], json!("+6281234567890"), "holder sees the phone: {body}");
    assert_eq!(body["data"]["birthDate"], json!("1990-05-17"), "holder sees the birth date: {body}");

    // So does a `*:*` caller.
    let (status, body) = send(&employees, "GET", &row_path, None, &wildcard).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["mobilePhone"], json!("+6281234567890"), "*:* holds every permission: {body}");

    // ── Includes: the expanded employee row is redacted against the caller. ──
    // The caller holds the employment list's permission and NOT employee:read —
    // exactly the cross-module shape (a salary through another entity's list).
    let include_caller = caller(&["employment:read"]);
    let (status, body) = send(
        &employments,
        "GET",
        &format!("/employments?include=employee&employeeId={id}"),
        None,
        &include_caller,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "include list answers: {body}");
    let expanded = &body["data"][0]["employee"];
    assert!(expanded.is_object(), "the include expands the employee row: {body}");
    for key in ["baseSalary", "mobilePhone", "birthDate"] {
        assert!(expanded[key].is_null(), "the include redacts {key} for a non-holder: {body}");
    }
    let (status, body) = send(
        &employments,
        "GET",
        &format!("/employments?include=employee&employeeId={id}"),
        None,
        &caller(&["employment:read", "employee:read_confidential"]),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body["data"][0]["employee"]["mobilePhone"], json!("+6281234567890"),
        "the include serves a holder of the field's permission: {body}"
    );

    // ── Questions: a non-holder may not name a gated field as a column. ──
    let (status, body) = send(&employees, "GET", "/employees?baseSalary=1234500.00", None, &non_holder).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "a gated filter is refused: {body}");
    let (code, params) = first_violation(&body);
    assert_eq!(code, "invalid_query", "{body}");
    assert_eq!(params["reason"], json!("field_permission"), "{body}");
    assert_eq!(params["permission"], json!("employee:read_confidential"), "{body}");

    let (status, body) = send(&employees, "GET", "/employees?baseSalary=1234500.00", None, &holder).await;
    assert_eq!(status, StatusCode::OK, "a holder may ask the same question: {body}");

    // ── Writes: naming a gated field without the permission is refused, naming
    // nothing gated is untouched. ──
    let writer = caller(&["employee:read", "employee:update"]);
    let (status, body) = send(&employees, "PATCH", &row_path, Some(json!({"baseSalary": "999.00"})), &writer).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "a gated write is refused: {body}");
    let (code, params) = first_violation(&body);
    assert_eq!(code, "field_permission_required", "{body}");
    assert_eq!(params["permission"], json!("employee:read_confidential"), "{body}");

    let (status, body) = send(&employees, "PATCH", &row_path, Some(json!({"firstName": "Ani"})), &writer).await;
    assert_eq!(status, StatusCode::OK, "a write naming no gated field is untouched: {body}");

    let conf_writer = caller(&["employee:read", "employee:update", "employee:read_confidential"]);
    let (status, body) = send(&employees, "PATCH", &row_path, Some(json!({"mobilePhone": "+628111000111"})), &conf_writer).await;
    assert_eq!(status, StatusCode::OK, "a holder may write the gated field: {body}");

    // ── Search: the declared set cannot probe a gated field. ──
    let (status, body) = send(&employees, "GET", "/employees?search=%2B6281234567890", None, &non_holder).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"].as_array().map(Vec::len), Some(0), "the phone number is not a search column: {body}");
    let (status, body) = send(&employees, "GET", "/employees?search=Ani", None, &non_holder).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "the given name is a search column: {body}");

    // The refused write really wrote nothing (the phone changed only via the holder).
    let stored: (Option<String>,) = sqlx::query_as("SELECT mobile_phone FROM employee.employees WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("read back");
    assert_eq!(stored.0.as_deref(), Some("+628111000111"), "only the holder's write landed");

    pool.close().await;
    sqlx::query(&format!("DROP DATABASE {dbname}")).execute(&admin).await.expect("drop the scratch database");
    admin.close().await;
}
