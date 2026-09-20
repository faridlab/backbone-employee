use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::RecordChangeStatus;
use super::AuditMetadata;

/// Strongly-typed ID for RecordChangeRequest
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecordChangeRequestId(pub Uuid);

impl RecordChangeRequestId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for RecordChangeRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for RecordChangeRequestId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for RecordChangeRequestId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<RecordChangeRequestId> for Uuid {
    fn from(id: RecordChangeRequestId) -> Self { id.0 }
}

impl AsRef<Uuid> for RecordChangeRequestId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for RecordChangeRequestId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecordChangeRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub field_path: String,
    pub current_value: Option<String>,
    pub proposed_value: Option<String>,
    pub reason: Option<String>,
    pub status: RecordChangeStatus,
    pub approval_request_id: Option<Uuid>,
    pub decided_at: Option<DateTime<Utc>>,
    pub applied_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl RecordChangeRequest {
    /// Create a builder for RecordChangeRequest
    pub fn builder() -> RecordChangeRequestBuilder {
        <RecordChangeRequestBuilder as Default>::default()
    }

    /// Create a new RecordChangeRequest with required fields
    pub fn new(employee_id: Uuid, field_path: String, status: RecordChangeStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            field_path,
            current_value: None,
            proposed_value: None,
            reason: None,
            status,
            approval_request_id: None,
            decided_at: None,
            applied_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> RecordChangeRequestId {
        RecordChangeRequestId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &RecordChangeStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the current_value field (chainable)
    pub fn with_current_value(mut self, value: String) -> Self {
        self.current_value = Some(value);
        self
    }

    /// Set the proposed_value field (chainable)
    pub fn with_proposed_value(mut self, value: String) -> Self {
        self.proposed_value = Some(value);
        self
    }

    /// Set the reason field (chainable)
    pub fn with_reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the approval_request_id field (chainable)
    pub fn with_approval_request_id(mut self, value: Uuid) -> Self {
        self.approval_request_id = Some(value);
        self
    }

    /// Set the decided_at field (chainable)
    pub fn with_decided_at(mut self, value: DateTime<Utc>) -> Self {
        self.decided_at = Some(value);
        self
    }

    /// Set the applied_at field (chainable)
    pub fn with_applied_at(mut self, value: DateTime<Utc>) -> Self {
        self.applied_at = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.employee_id = v; }
                }
                "field_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.field_path = v; }
                }
                "current_value" => {
                    if let Ok(v) = serde_json::from_value(value) { self.current_value = v; }
                }
                "proposed_value" => {
                    if let Ok(v) = serde_json::from_value(value) { self.proposed_value = v; }
                }
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "approval_request_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.approval_request_id = v; }
                }
                "decided_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.decided_at = v; }
                }
                "applied_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.applied_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for RecordChangeRequest {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "RecordChangeRequest"
    }
}

impl backbone_core::PersistentEntity for RecordChangeRequest {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for RecordChangeRequest {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("approval_request_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "record_change_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["field_path"]
    }
}

/// Builder for RecordChangeRequest entity
///
/// Provides a fluent API for constructing RecordChangeRequest instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct RecordChangeRequestBuilder {
    employee_id: Option<Uuid>,
    field_path: Option<String>,
    current_value: Option<String>,
    proposed_value: Option<String>,
    reason: Option<String>,
    status: Option<RecordChangeStatus>,
    approval_request_id: Option<Uuid>,
    decided_at: Option<DateTime<Utc>>,
    applied_at: Option<DateTime<Utc>>,
}

impl RecordChangeRequestBuilder {
    /// Set the employee_id field (required)
    pub fn employee_id(mut self, value: Uuid) -> Self {
        self.employee_id = Some(value);
        self
    }

    /// Set the field_path field (required)
    pub fn field_path(mut self, value: String) -> Self {
        self.field_path = Some(value);
        self
    }

    /// Set the current_value field (optional)
    pub fn current_value(mut self, value: String) -> Self {
        self.current_value = Some(value);
        self
    }

    /// Set the proposed_value field (optional)
    pub fn proposed_value(mut self, value: String) -> Self {
        self.proposed_value = Some(value);
        self
    }

    /// Set the reason field (optional)
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the status field (default: `RecordChangeStatus::default()`)
    pub fn status(mut self, value: RecordChangeStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the approval_request_id field (optional)
    pub fn approval_request_id(mut self, value: Uuid) -> Self {
        self.approval_request_id = Some(value);
        self
    }

    /// Set the decided_at field (optional)
    pub fn decided_at(mut self, value: DateTime<Utc>) -> Self {
        self.decided_at = Some(value);
        self
    }

    /// Set the applied_at field (optional)
    pub fn applied_at(mut self, value: DateTime<Utc>) -> Self {
        self.applied_at = Some(value);
        self
    }

    /// Build the RecordChangeRequest entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<RecordChangeRequest, String> {
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let field_path = self.field_path.ok_or_else(|| "field_path is required".to_string())?;

        Ok(RecordChangeRequest {
            id: Uuid::new_v4(),
            employee_id,
            field_path,
            current_value: self.current_value,
            proposed_value: self.proposed_value,
            reason: self.reason,
            status: self.status.unwrap_or_default(),
            approval_request_id: self.approval_request_id,
            decided_at: self.decided_at,
            applied_at: self.applied_at,
            metadata: AuditMetadata::default(),
        })
    }
}
