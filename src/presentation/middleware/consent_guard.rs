//! UU PDP consent-guard middleware — intercepts PII entity POST (create) requests,
//! extracts `employee_id` from the JSON body, checks a valid DataConsent exists,
//! and rejects (403) if not. Non-PII routes + non-POST methods pass through.
//!
//! Mount on the employee module's PII entity routes in the composer, like `company_auth`:
//!   .route_layer(from_fn(consent_guard_middleware))

use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;

use crate::domain::entity::DataCategory;
use crate::application::service::consent_guard;

/// Map a request path to a PII DataCategory, or None if it's not a PII route.
fn path_to_category(path: &str) -> Option<DataCategory> {
    // The generated routes are /employee_identities, /employee_taxes, etc.
    // (collection names from the schema.)
    match path.trim_end_matches('/') {
        "/employee_identities" | "/employee_identities/" => Some(DataCategory::Identity),
        "/employee_taxes" | "/employee_taxes/" => Some(DataCategory::Financial),
        "/employee_bpjs" | "/employee_bpjs/" => Some(DataCategory::Financial),
        "/employee_families" | "/employee_families/" => Some(DataCategory::Family),
        "/employee_bank_accounts" | "/employee_bank_accounts/" => Some(DataCategory::Financial),
        "/employee_contacts" | "/employee_contacts/" => Some(DataCategory::Contact),
        _ => None,
    }
}

/// The consent-guard middleware. Mount on PII entity routes.
///
/// - Non-POST methods → pass through (reads/GET are not consent-gated).
/// - Non-PII paths → pass through.
/// - POST to a PII path → buffer body, extract `employee_id`, check consent.
///   - Valid consent → pass through.
///   - No consent → 403 Forbidden with the UU PDP message.
///   - No `employee_id` in body → pass through (let the handler validate the input).
pub async fn consent_guard_middleware(
    Extension(pool): Extension<sqlx::PgPool>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Only gate POST (create/update) requests.
    if request.method() != Method::POST {
        return next.run(request).await;
    }

    // Only gate PII entity routes.
    let path = request.uri().path().to_string();
    let category = match path_to_category(&path) {
        Some(c) => c,
        None => return next.run(request).await,
    };

    // Buffer the body to extract employee_id, then reconstruct the request.
    let (parts, body) = request.into_parts();
    let bytes = match to_bytes(body, 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => return next.run(Request::from_parts(parts, Body::empty())).await,
    };

    // Parse employee_id from the JSON body (best-effort — no employee_id → passthrough).
    let employee_id: Option<uuid::Uuid> = serde_json::from_slice::<serde_json::Value>(&bytes)
        .ok()
        .and_then(|v| {
            v.get("employee_id")
                .and_then(|e| e.as_str())
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
        });

    let request = Request::from_parts(parts, Body::from(bytes));

    match employee_id {
        Some(emp_id) => {
            if consent_guard::has_valid_consent(&pool, emp_id, &category).await {
                next.run(request).await
            } else {
                Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .header("Content-Type", "application/json")
                    .body(Body::from(format!(
                        r#"{{"error":"UU_PDP_CONSENT_REQUIRED","message":"No valid DataConsent for category '{}' (employee {}). Capture consent before writing this PII."}}"#,
                        category, emp_id
                    )))
                    .unwrap_or_else(|_| {
                        // Static fallback: an empty body with the status set directly cannot fail.
                        let mut fallback = Response::new(Body::empty());
                        *fallback.status_mut() = StatusCode::FORBIDDEN;
                        fallback
                    })
            }
        }
        None => next.run(request).await,
    }
}
