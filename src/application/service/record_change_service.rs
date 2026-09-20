//! The self-service record-change lifecycle: an employee ASKS, an approver
//! DECIDES on the approvals spine, and only then does anything touch the
//! record itself.
//!
//! The trail is the point (#485): "the data is out of date" used to be a
//! direct write or a conversation, and neither left a reviewable record of
//! who asked for what. Now every ask is a row with its before, after, and
//! reason; the verdict rides the same engine every other approval does; and
//! the write happens in ONE place, AFTER approval, for a WHITELIST of
//! base-profile contact fields the employee owns in practice (mobile_phone,
//! phone, email). Anything else the apply verb refuses typed — the request
//! is still reviewable, and HR executes it through their normal audited
//! edit screens rather than this lane inventing writes for fields with
//! consent gates (see consent_guard) or satellite shapes.

use chrono::Utc;
use sqlx::PgPool;
use std::sync::RwLock;
use uuid::Uuid;

use super::record_change_approvals_port::{
    RecordChangeFiling, RecordChangeFilingPort, RecordChangeSeamError, RecordChangeVerdict,
    UnwiredRecordApprovals,
};

#[derive(Debug, thiserror::Error)]
pub enum RecordChangeError {
    #[error("record change request not found")]
    NotFound,
    #[error("{0}")]
    Invalid(&'static str),
    /// The engine's verdict is not what this transition needs.
    #[error("{0}")]
    Verdict(&'static str),
    #[error("approvals seam: {0}")]
    Seam(#[from] RecordChangeSeamError),
    #[error("db: {0}")]
    Db(#[from] sqlx::Error),
}

/// The base-profile fields an employee may change through this lane once
/// approved. Everything else refuses typed at apply time.
const APPLIABLE_FIELDS: &[&str] = &["mobile_phone", "phone", "email"];

pub struct RecordChangeService {
    pool: PgPool,
    approvals: RwLock<std::sync::Arc<dyn RecordChangeFilingPort>>,
}

impl RecordChangeService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            approvals: RwLock::new(std::sync::Arc::new(UnwiredRecordApprovals)),
        }
    }

    /// Wire the approvals port (the composing service's adapter).
    pub fn set_approvals(&self, port: std::sync::Arc<dyn RecordChangeFilingPort>) {
        *self.approvals.write().expect("record approvals lock poisoned") = port;
    }

    fn approvals(&self) -> std::sync::Arc<dyn RecordChangeFilingPort> {
        self.approvals.read().expect("record approvals lock poisoned").clone()
    }

    /// File a change request: the row lands pending with its before/after,
    /// and the approvals engine gets the filing (a WIRED port that fails
    /// fails the submit — no untracked requests).
    pub async fn submit(
        &self,
        employee_id: Uuid,
        field_path: String,
        current_value: Option<String>,
        proposed_value: Option<String>,
        reason: Option<String>,
    ) -> Result<Uuid, RecordChangeError> {
        let field_path = field_path.trim().to_string();
        if field_path.is_empty() || field_path.len() > 120 {
            return Err(RecordChangeError::Invalid("field_path must be 1..=120 characters"));
        }
        if proposed_value.as_deref().map(str::trim).unwrap_or("").is_empty() {
            return Err(RecordChangeError::Invalid("proposed_value is required"));
        }
        // File first (the correlation id rides the filing), then insert with
        // the link — the same ordering discipline as the leave lifecycle.
        let request_id = Uuid::new_v4();
        let approval_request_id = match self
            .approvals()
            .file(&RecordChangeFiling {
                request_id,
                employee_id,
                field_path: field_path.clone(),
                current_value: current_value.clone(),
                proposed_value: proposed_value.clone(),
                reason: reason.clone(),
            })
            .await
        {
            Ok(id) => Some(id),
            Err(RecordChangeSeamError::Unwired) => None,
            Err(e) => return Err(e.into()),
        };

        let mut tx = self.pool.begin().await?;
        if let Some(scope) = backbone_orm::org_scope::current_org_scope() {
            backbone_orm::org_scope::bind_org_scope_on(&mut tx, &scope).await?;
        }
        let inserted = sqlx::query_scalar::<_, Uuid>(
            r#"INSERT INTO employee.record_change_requests
                 (id, employee_id, field_path, current_value, proposed_value, reason,
                  status, approval_request_id, metadata)
               VALUES ($1, $2, $3, $4, $5, $6, 'pending', $7, '{}'::jsonb)
               RETURNING id"#,
        )
        .bind(request_id)
        .bind(employee_id)
        .bind(&field_path)
        .bind(current_value)
        .bind(proposed_value)
        .bind(reason)
        .bind(approval_request_id)
        // Scoped ride: under the composing service's fence the insert lands
        // on the request-dedicated connection (org key filled by trigger
        // from the ambient acting unit).
        // Scoped ride: under the composing service's fence the insert lands
        // on the request-dedicated connection (org key filled by trigger
        // from the ambient acting unit); unfenced deployments behave as
        // before.
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(RecordChangeError::NotFound)?;
        tx.commit().await?;
        Ok(inserted)
    }

    /// The approved change, applied: writes the whitelisted field, stamps
    /// applied. Requires the engine's APPROVED verdict — approval and the
    /// write are two acts, deliberately (the engine vouches; this lane
    /// executes).
    pub async fn apply(&self, request_id: Uuid) -> Result<(), RecordChangeError> {
        let row = self.load(request_id).await?;
        if row.status != "pending" && row.status != "ready" {
            return Err(RecordChangeError::Invalid("only a pending or ready request can be applied"));
        }
        let Some(approval) = row.approval_request_id else {
            return Err(RecordChangeError::Verdict(
                "the request carries no approval link (approvals seam unwired at submit time)",
            ));
        };
        match self.approvals().status(approval).await? {
            RecordChangeVerdict::Approved => {}
            RecordChangeVerdict::Pending => {
                return Err(RecordChangeError::Verdict("the approval is still pending"));
            }
            RecordChangeVerdict::Rejected => {
                return Err(RecordChangeError::Verdict("the approval was rejected"));
            }
        }
        if !APPLIABLE_FIELDS.contains(&row.field_path.as_str()) {
            return Err(RecordChangeError::Invalid(
                "this field is not appliable through the self-service lane — route it through HR's audited edit",
            ));
        }

        let mut tx = self.pool.begin().await?;
        if let Some(scope) = backbone_orm::org_scope::current_org_scope() {
            backbone_orm::org_scope::bind_org_scope_on(&mut tx, &scope).await?;
        }
        let column = row.field_path.as_str();
        // field_path is whitelist-checked above, so the interpolation names
        // one of three known columns — never client input by the time it
        // reaches here.
        let sql = format!(
            "UPDATE employee.employees SET {column} = $2 WHERE id = $1 AND (metadata->>'deleted_at') IS NULL"
        );
        let updated = sqlx::query(&sql)
            .bind(row.employee_id)
            .bind(&row.proposed_value)
            .execute(&mut *tx)
            .await?
            .rows_affected();
        if updated != 1 {
            return Err(RecordChangeError::NotFound);
        }
        sqlx::query(
            r#"UPDATE employee.record_change_requests
                  SET status = 'applied', applied_at = now(), decided_at = COALESCE(decided_at, now())
                WHERE id = $1"#,
        )
        .bind(request_id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Record the engine's rejection on the request row.
    pub async fn refuse(&self, request_id: Uuid) -> Result<(), RecordChangeError> {
        let row = self.load(request_id).await?;
        if row.status != "pending" {
            return Err(RecordChangeError::Invalid("only a pending request can be refused"));
        }
        let Some(approval) = row.approval_request_id else {
            return Err(RecordChangeError::Verdict("the request carries no approval link"));
        };
        match self.approvals().status(approval).await? {
            RecordChangeVerdict::Rejected => {}
            RecordChangeVerdict::Approved => {
                return Err(RecordChangeError::Verdict("the approval was approved — apply it instead"));
            }
            RecordChangeVerdict::Pending => {
                return Err(RecordChangeError::Verdict("the approval is still pending"));
            }
        }
        let mut tx = self.pool.begin().await?;
        if let Some(scope) = backbone_orm::org_scope::current_org_scope() {
            backbone_orm::org_scope::bind_org_scope_on(&mut tx, &scope).await?;
        }
        sqlx::query(
            r#"UPDATE employee.record_change_requests
                  SET status = 'rejected', decided_at = now()
                WHERE id = $1"#,
        )
        .bind(request_id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// The requester withdraws the ask (before any verdict).
    pub async fn cancel(&self, request_id: Uuid) -> Result<(), RecordChangeError> {
        let row = self.load(request_id).await?;
        if row.status != "pending" {
            return Err(RecordChangeError::Invalid("only a pending request can be cancelled"));
        }
        let mut tx = self.pool.begin().await?;
        if let Some(scope) = backbone_orm::org_scope::current_org_scope() {
            backbone_orm::org_scope::bind_org_scope_on(&mut tx, &scope).await?;
        }
        sqlx::query(
            r#"UPDATE employee.record_change_requests
                  SET status = 'cancelled'
                WHERE id = $1"#,
        )
        .bind(request_id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn load(&self, request_id: Uuid) -> Result<Row, RecordChangeError> {
        backbone_orm::company_scope::fetch_optional_scoped(
            &self.pool,
            sqlx::query_as::<_, Row>(
                r#"SELECT employee_id, field_path, proposed_value, status::text AS status,
                          approval_request_id
                     FROM employee.record_change_requests
                    WHERE id = $1 AND (metadata->>'deleted_at') IS NULL"#,
            )
            .bind(request_id),
        )
        .await?
        .ok_or(RecordChangeError::NotFound)
    }
}

#[derive(sqlx::FromRow)]
struct Row {
    employee_id: Uuid,
    field_path: String,
    proposed_value: Option<String>,
    status: String,
    approval_request_id: Option<Uuid>,
}
