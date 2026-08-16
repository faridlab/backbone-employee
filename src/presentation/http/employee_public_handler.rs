//! Read-only HTTP surface for the `employee.employees_public` directory view — the
//! hr.employee.public port (Wave 1 P1 / H-1).
//!
//! User-owned (declared under `user_owned` in `metaphor.codegen.yaml`). GET-only by
//! construction: no write verb exists for a view, so the Odoo `create="0" write="0"`
//! directory posture is enforced by the route table itself.
//!
//! Route shape follows the generated read-route family (`/employees/...`) so composing
//! apps mount it beside `create_readonly_employee_routes` under the same guard layers.
//! Company scoping is NOT done here — the composition root's guard layers bind the
//! request-scoped company (RLS fence), and the view is `security_invoker = on`.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::infrastructure::persistence::employee_public_repository::EmployeePublicRepository;

/// Shared state for the directory routes — the pool the repo reads through.
#[derive(Clone)]
pub struct EmployeePublicState {
    pool: sqlx::PgPool,
}

impl EmployeePublicState {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

/// `GET /employees/public` — page through the company directory.
///
/// Query params: `limit` (default 50, max 200) and `offset` (default 0).
#[derive(serde::Deserialize)]
pub struct ListPublicParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_public(
    State(state): State<EmployeePublicState>,
    axum::extract::Query(params): axum::extract::Query<ListPublicParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(50).clamp(1, 200);
    let offset = params.offset.unwrap_or(0).max(0);
    match EmployeePublicRepository::new().list_public(&state.pool, limit, offset).await {
        Ok(rows) => (StatusCode::OK, Json(rows)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("directory listing failed: {e}"),
        )
            .into_response(),
    }
}

/// `GET /employees/public/{id}` — one directory row.
pub async fn get_public(
    State(state): State<EmployeePublicState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match EmployeePublicRepository::new().find_public(&state.pool, id).await {
        Ok(Some(row)) => (StatusCode::OK, Json(row)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "employee not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("directory lookup failed: {e}"),
        )
            .into_response(),
    }
}

/// Read-only directory routes (`GET /employees/public`, `GET /employees/public/:id`).
pub fn create_employee_public_read_routes(pool: sqlx::PgPool) -> Router<()> {
    let state = EmployeePublicState::new(pool);
    Router::new()
        .route("/employees/public", get(list_public))
        .route("/employees/public/:id", get(get_public))
        .with_state(state)
}
