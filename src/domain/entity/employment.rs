use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::EmploymentStatus;
use super::EmploymentState;
use super::AuditMetadata;

/// Strongly-typed ID for Employment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmploymentId(pub Uuid);

impl EmploymentId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmploymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmploymentId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmploymentId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmploymentId> for Uuid {
    fn from(id: EmploymentId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmploymentId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmploymentId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Employment {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub employment_status: EmploymentStatus,
    pub join_date: NaiveDate,
    pub end_join_date: Option<NaiveDate>,
    pub department_id: Option<Uuid>,
    pub level_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub direct_manager_id: Option<Uuid>,
    pub status: EmploymentState,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Employment {
    /// Create a builder for Employment
    pub fn builder() -> EmploymentBuilder {
        <EmploymentBuilder as Default>::default()
    }

    /// Create a new Employment with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, employment_status: EmploymentStatus, join_date: NaiveDate, status: EmploymentState) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            employment_status,
            join_date,
            end_join_date: None,
            department_id: None,
            level_id: None,
            position_id: None,
            direct_manager_id: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmploymentId {
        EmploymentId(self.id)
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
    pub fn status(&self) -> &EmploymentState {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the end_join_date field (chainable)
    pub fn with_end_join_date(mut self, value: NaiveDate) -> Self {
        self.end_join_date = Some(value);
        self
    }

    /// Set the department_id field (chainable)
    pub fn with_department_id(mut self, value: Uuid) -> Self {
        self.department_id = Some(value);
        self
    }

    /// Set the level_id field (chainable)
    pub fn with_level_id(mut self, value: Uuid) -> Self {
        self.level_id = Some(value);
        self
    }

    /// Set the position_id field (chainable)
    pub fn with_position_id(mut self, value: Uuid) -> Self {
        self.position_id = Some(value);
        self
    }

    /// Set the direct_manager_id field (chainable)
    pub fn with_direct_manager_id(mut self, value: Uuid) -> Self {
        self.direct_manager_id = Some(value);
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
                "employment_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.employment_status = v; }
                }
                "join_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.join_date = v; }
                }
                "end_join_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.end_join_date = v; }
                }
                "department_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.department_id = v; }
                }
                "level_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.level_id = v; }
                }
                "position_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.position_id = v; }
                }
                "direct_manager_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.direct_manager_id = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Employment {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Employment"
    }
}

impl backbone_core::PersistentEntity for Employment {
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

impl backbone_orm::EntityRepoMeta for Employment {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("department_id".to_string(), "uuid".to_string());
        m.insert("level_id".to_string(), "uuid".to_string());
        m.insert("position_id".to_string(), "uuid".to_string());
        m.insert("direct_manager_id".to_string(), "uuid".to_string());
        m.insert("employment_status".to_string(), "employment_status".to_string());
        m.insert("status".to_string(), "employment_state".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for Employment entity
///
/// Provides a fluent API for constructing Employment instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmploymentBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    employment_status: Option<EmploymentStatus>,
    join_date: Option<NaiveDate>,
    end_join_date: Option<NaiveDate>,
    department_id: Option<Uuid>,
    level_id: Option<Uuid>,
    position_id: Option<Uuid>,
    direct_manager_id: Option<Uuid>,
    status: Option<EmploymentState>,
}

impl EmploymentBuilder {
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

    /// Set the employment_status field (default: `EmploymentStatus::default()`)
    pub fn employment_status(mut self, value: EmploymentStatus) -> Self {
        self.employment_status = Some(value);
        self
    }

    /// Set the join_date field (required)
    pub fn join_date(mut self, value: NaiveDate) -> Self {
        self.join_date = Some(value);
        self
    }

    /// Set the end_join_date field (optional)
    pub fn end_join_date(mut self, value: NaiveDate) -> Self {
        self.end_join_date = Some(value);
        self
    }

    /// Set the department_id field (optional)
    pub fn department_id(mut self, value: Uuid) -> Self {
        self.department_id = Some(value);
        self
    }

    /// Set the level_id field (optional)
    pub fn level_id(mut self, value: Uuid) -> Self {
        self.level_id = Some(value);
        self
    }

    /// Set the position_id field (optional)
    pub fn position_id(mut self, value: Uuid) -> Self {
        self.position_id = Some(value);
        self
    }

    /// Set the direct_manager_id field (optional)
    pub fn direct_manager_id(mut self, value: Uuid) -> Self {
        self.direct_manager_id = Some(value);
        self
    }

    /// Set the status field (default: `EmploymentState::default()`)
    pub fn status(mut self, value: EmploymentState) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the Employment entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Employment, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let join_date = self.join_date.ok_or_else(|| "join_date is required".to_string())?;

        Ok(Employment {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            employment_status: self.employment_status.unwrap_or_default(),
            join_date,
            end_join_date: self.end_join_date,
            department_id: self.department_id,
            level_id: self.level_id,
            position_id: self.position_id,
            direct_manager_id: self.direct_manager_id,
            status: self.status.unwrap_or_default(),
            metadata: AuditMetadata::default(),
        })
    }
}
