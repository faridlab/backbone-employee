//! The approvals seam for self-service record changes: the port the engine
//! files through, mirroring the timeoff module's posture (ADR-0004: no crate
//! edge — the composing service injects an adapter over backbone-approvals;
//! the default here is unwired, and a wired port that fails fails the submit
//! rather than creating a request the engine never saw).

use async_trait::async_trait;
use uuid::Uuid;

/// What a filing carries: enough of the proposed change for an approver to
/// render a verdict row without another read.
#[derive(Debug, Clone)]
pub struct RecordChangeFiling {
    /// The record_change_requests row's id (the correlation id).
    pub request_id: Uuid,
    pub employee_id: Uuid,
    pub field_path: String,
    pub current_value: Option<String>,
    pub proposed_value: Option<String>,
    pub reason: Option<String>,
}

/// The engine's verdict, as far as a record change cares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordChangeVerdict {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, thiserror::Error)]
pub enum RecordChangeSeamError {
    /// This deployment doesn't track approvals: the request carries no link
    /// (module behaves exactly as before the seam existed).
    #[error("approvals seam is not wired")]
    Unwired,
    #[error("no approval request {0} is known to the engine")]
    UnknownApprovalRequest(Uuid),
    #[error("approvals transport: {0}")]
    Transport(String),
}

#[async_trait]
pub trait RecordChangeFilingPort: Send + Sync {
    async fn file(&self, filing: &RecordChangeFiling) -> Result<Uuid, RecordChangeSeamError>;
    async fn status(&self, approval_request_id: Uuid) -> Result<RecordChangeVerdict, RecordChangeSeamError>;
}

/// The unwired default.
pub struct UnwiredRecordApprovals;

#[async_trait]
impl RecordChangeFilingPort for UnwiredRecordApprovals {
    async fn file(&self, _filing: &RecordChangeFiling) -> Result<Uuid, RecordChangeSeamError> {
        Err(RecordChangeSeamError::Unwired)
    }
    async fn status(&self, id: Uuid) -> Result<RecordChangeVerdict, RecordChangeSeamError> {
        Err(RecordChangeSeamError::UnknownApprovalRequest(id))
    }
}
