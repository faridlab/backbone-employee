use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::EmploymentAction;
use super::AuditMetadata;

/// Strongly-typed ID for EmploymentHistory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmploymentHistoryId(pub Uuid);

impl EmploymentHistoryId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmploymentHistoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmploymentHistoryId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmploymentHistoryId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmploymentHistoryId> for Uuid {
    fn from(id: EmploymentHistoryId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmploymentHistoryId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmploymentHistoryId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmploymentHistory {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub effective_date: NaiveDate,
    pub action: EmploymentAction,
    pub position_id_from: Option<Uuid>,
    pub position_id_to: Option<Uuid>,
    pub level_id_from: Option<Uuid>,
    pub level_id_to: Option<Uuid>,
    pub department_id_from: Option<Uuid>,
    pub department_id_to: Option<Uuid>,
    pub reference_id: Option<Uuid>,
    pub note: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmploymentHistory {
    /// Create a builder for EmploymentHistory
    pub fn builder() -> EmploymentHistoryBuilder {
        <EmploymentHistoryBuilder as Default>::default()
    }

    /// Create a new EmploymentHistory with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, effective_date: NaiveDate, action: EmploymentAction) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            effective_date,
            action,
            position_id_from: None,
            position_id_to: None,
            level_id_from: None,
            level_id_to: None,
            department_id_from: None,
            department_id_to: None,
            reference_id: None,
            note: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmploymentHistoryId {
        EmploymentHistoryId(self.id)
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


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the position_id_from field (chainable)
    pub fn with_position_id_from(mut self, value: Uuid) -> Self {
        self.position_id_from = Some(value);
        self
    }

    /// Set the position_id_to field (chainable)
    pub fn with_position_id_to(mut self, value: Uuid) -> Self {
        self.position_id_to = Some(value);
        self
    }

    /// Set the level_id_from field (chainable)
    pub fn with_level_id_from(mut self, value: Uuid) -> Self {
        self.level_id_from = Some(value);
        self
    }

    /// Set the level_id_to field (chainable)
    pub fn with_level_id_to(mut self, value: Uuid) -> Self {
        self.level_id_to = Some(value);
        self
    }

    /// Set the department_id_from field (chainable)
    pub fn with_department_id_from(mut self, value: Uuid) -> Self {
        self.department_id_from = Some(value);
        self
    }

    /// Set the department_id_to field (chainable)
    pub fn with_department_id_to(mut self, value: Uuid) -> Self {
        self.department_id_to = Some(value);
        self
    }

    /// Set the reference_id field (chainable)
    pub fn with_reference_id(mut self, value: Uuid) -> Self {
        self.reference_id = Some(value);
        self
    }

    /// Set the note field (chainable)
    pub fn with_note(mut self, value: String) -> Self {
        self.note = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "company_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_id = v; }
                }
                "employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.employee_id = v; }
                }
                "effective_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.effective_date = v; }
                }
                "action" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action = v; }
                }
                "position_id_from" => {
                    if let Ok(v) = serde_json::from_value(value) { self.position_id_from = v; }
                }
                "position_id_to" => {
                    if let Ok(v) = serde_json::from_value(value) { self.position_id_to = v; }
                }
                "level_id_from" => {
                    if let Ok(v) = serde_json::from_value(value) { self.level_id_from = v; }
                }
                "level_id_to" => {
                    if let Ok(v) = serde_json::from_value(value) { self.level_id_to = v; }
                }
                "department_id_from" => {
                    if let Ok(v) = serde_json::from_value(value) { self.department_id_from = v; }
                }
                "department_id_to" => {
                    if let Ok(v) = serde_json::from_value(value) { self.department_id_to = v; }
                }
                "reference_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reference_id = v; }
                }
                "note" => {
                    if let Ok(v) = serde_json::from_value(value) { self.note = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmploymentHistory {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmploymentHistory"
    }
}

impl backbone_core::PersistentEntity for EmploymentHistory {
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

impl backbone_orm::EntityRepoMeta for EmploymentHistory {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("reference_id".to_string(), "uuid".to_string());
        m.insert("action".to_string(), "employment_action".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for EmploymentHistory entity
///
/// Provides a fluent API for constructing EmploymentHistory instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmploymentHistoryBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    effective_date: Option<NaiveDate>,
    action: Option<EmploymentAction>,
    position_id_from: Option<Uuid>,
    position_id_to: Option<Uuid>,
    level_id_from: Option<Uuid>,
    level_id_to: Option<Uuid>,
    department_id_from: Option<Uuid>,
    department_id_to: Option<Uuid>,
    reference_id: Option<Uuid>,
    note: Option<String>,
}

impl EmploymentHistoryBuilder {
    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the employee_id field (required)
    pub fn employee_id(mut self, value: Uuid) -> Self {
        self.employee_id = Some(value);
        self
    }

    /// Set the effective_date field (required)
    pub fn effective_date(mut self, value: NaiveDate) -> Self {
        self.effective_date = Some(value);
        self
    }

    /// Set the action field (required)
    pub fn action(mut self, value: EmploymentAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Set the position_id_from field (optional)
    pub fn position_id_from(mut self, value: Uuid) -> Self {
        self.position_id_from = Some(value);
        self
    }

    /// Set the position_id_to field (optional)
    pub fn position_id_to(mut self, value: Uuid) -> Self {
        self.position_id_to = Some(value);
        self
    }

    /// Set the level_id_from field (optional)
    pub fn level_id_from(mut self, value: Uuid) -> Self {
        self.level_id_from = Some(value);
        self
    }

    /// Set the level_id_to field (optional)
    pub fn level_id_to(mut self, value: Uuid) -> Self {
        self.level_id_to = Some(value);
        self
    }

    /// Set the department_id_from field (optional)
    pub fn department_id_from(mut self, value: Uuid) -> Self {
        self.department_id_from = Some(value);
        self
    }

    /// Set the department_id_to field (optional)
    pub fn department_id_to(mut self, value: Uuid) -> Self {
        self.department_id_to = Some(value);
        self
    }

    /// Set the reference_id field (optional)
    pub fn reference_id(mut self, value: Uuid) -> Self {
        self.reference_id = Some(value);
        self
    }

    /// Set the note field (optional)
    pub fn note(mut self, value: String) -> Self {
        self.note = Some(value);
        self
    }

    /// Build the EmploymentHistory entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmploymentHistory, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let effective_date = self.effective_date.ok_or_else(|| "effective_date is required".to_string())?;
        let action = self.action.ok_or_else(|| "action is required".to_string())?;

        Ok(EmploymentHistory {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            effective_date,
            action,
            position_id_from: self.position_id_from,
            position_id_to: self.position_id_to,
            level_id_from: self.level_id_from,
            level_id_to: self.level_id_to,
            department_id_from: self.department_id_from,
            department_id_to: self.department_id_to,
            reference_id: self.reference_id,
            note: self.note,
            metadata: AuditMetadata::default(),
        })
    }
}
