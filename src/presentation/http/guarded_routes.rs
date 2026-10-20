//! Guarded route composition for the Employee module (user-owned).
//!
//! Also carries the self-service record-change verbs (submit / apply /
//! refuse / cancel) over the lifecycle service — mounted by the composing
//! service via [`crate::EmployeeModule::record_change_verb_routes`].
//!
//! `EmployeeModule::routes()` (the deprecated alias) and `all_crud_routes()`
//! mount UNVALIDATED generic CRUD on every entity; the base read-only router
//! comes from the generated [`EmployeeModule::readonly_routes`]. This file
//! adds what the generator cannot know:
//!
//! - [`EmployeeModule::readonly_routes_with_public`] — the read-only base plus
//!   the `hr.employee.public` PII-redacted peer directory
//!   ([`crate::presentation::http::create_employee_public_read_routes`]).
//!
//! Validated writes (the PII write service's consent-gated surface) merge onto
//! this base when their HTTP layer lands.

use axum::Router;

impl crate::EmployeeModule {
    /// [`Self::readonly_routes`] plus the `hr.employee.public` peer directory —
    /// the guarded composition a consumer mounts.
    pub fn readonly_routes_with_public(&self) -> Router {
        self.readonly_routes().merge(
            crate::presentation::http::create_employee_public_read_routes(self.db_pool.clone()),
        )
    }
}

/// The record-change verbs. Mount under the host's org-guarded,
/// tenant-routed tree (reads ride the module's generic read surface).
pub fn record_change_verb_routes(
    svc: std::sync::Arc<crate::application::service::record_change_service::RecordChangeService>,
) -> Router {
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::{IntoResponse, Response},
        routing::post,
        Json,
    };
    use backbone_auth::org::OrgContext;
    use serde::Deserialize;
    use serde_json::json;
    use uuid::Uuid;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SubmitBody {
        employee_id: Uuid,
        field_path: String,
        #[serde(default)]
        current_value: Option<String>,
        proposed_value: String,
        #[serde(default)]
        reason: Option<String>,
    }

    fn err(e: crate::application::service::record_change_service::RecordChangeError) -> Response {
        use crate::application::service::record_change_service::RecordChangeError as E;
        let (status, code) = match &e {
            E::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            E::Invalid(_) => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_request"),
            E::Verdict(_) => (StatusCode::CONFLICT, "verdict_not_satisfied"),
            E::Seam(_) => (StatusCode::INTERNAL_SERVER_ERROR, "approvals_seam_error"),
            E::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
        };
        (status, Json(json!({ "error": code, "message": e.to_string() }))).into_response()
    }

    async fn submit(
        _org: OrgContext,
        State(svc): State<std::sync::Arc<crate::application::service::record_change_service::RecordChangeService>>,
        Json(b): Json<SubmitBody>,
    ) -> Response {
        match svc
            .submit(b.employee_id, b.field_path, b.current_value, Some(b.proposed_value), b.reason)
            .await
        {
            Ok(id) => (StatusCode::CREATED, Json(json!({ "id": id }))).into_response(),
            Err(e) => err(e),
        }
    }

    async fn apply(
        _org: OrgContext,
        State(svc): State<std::sync::Arc<crate::application::service::record_change_service::RecordChangeService>>,
        Path(request_id): Path<Uuid>,
    ) -> Response {
        match svc.apply(request_id).await {
            Ok(()) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => err(e),
        }
    }

    async fn refuse(
        _org: OrgContext,
        State(svc): State<std::sync::Arc<crate::application::service::record_change_service::RecordChangeService>>,
        Path(request_id): Path<Uuid>,
    ) -> Response {
        match svc.refuse(request_id).await {
            Ok(()) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => err(e),
        }
    }

    async fn cancel(
        _org: OrgContext,
        State(svc): State<std::sync::Arc<crate::application::service::record_change_service::RecordChangeService>>,
        Path(request_id): Path<Uuid>,
    ) -> Response {
        match svc.cancel(request_id).await {
            Ok(()) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => err(e),
        }
    }

    Router::new()
        // unguarded: the self-service verbs mount under the host's org-guarded, tenant-routed tree (OrgContext fences the org); the module adds no permission of its own by design — see the header note.
        .route("/record-change-requests/submit", post(submit))
        // unguarded: same host-guarded tree; the approvals seam owns who may apply.
        .route("/record-change-requests/:request_id/apply", post(apply))
        // unguarded: same host-guarded tree; the approvals seam owns who may refuse.
        .route("/record-change-requests/:request_id/refuse", post(refuse))
        // unguarded: same host-guarded tree; a caller may cancel its own request.
        .route("/record-change-requests/:request_id/cancel", post(cancel))
        // Pool law (ADR-0029): bind the composer-installed request pool for
        // the whole handler call — the write services resolve their database
        // through the module's task-local, and without this binding a tenant
        // verb falls back to the composed boot pool.
        .layer(axum::middleware::from_fn(crate::request_pool::bind_request_pool))
        .with_state(svc)
}
