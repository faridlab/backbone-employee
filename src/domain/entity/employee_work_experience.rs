use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for EmployeeWorkExperience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeWorkExperienceId(pub Uuid);

impl EmployeeWorkExperienceId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmployeeWorkExperienceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeWorkExperienceId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeWorkExperienceId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmployeeWorkExperienceId> for Uuid {
    fn from(id: EmployeeWorkExperienceId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmployeeWorkExperienceId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmployeeWorkExperienceId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmployeeWorkExperience {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub company_name: String,
    pub job_position: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmployeeWorkExperience {
    /// Create a builder for EmployeeWorkExperience
    pub fn builder() -> EmployeeWorkExperienceBuilder {
        <EmployeeWorkExperienceBuilder as Default>::default()
    }

    /// Create a new EmployeeWorkExperience with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, company_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            company_name,
            job_position: None,
            start_date: None,
            end_date: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeWorkExperienceId {
        EmployeeWorkExperienceId(self.id)
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

    /// Set the job_position field (chainable)
    pub fn with_job_position(mut self, value: String) -> Self {
        self.job_position = Some(value);
        self
    }

    /// Set the start_date field (chainable)
    pub fn with_start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Set the end_date field (chainable)
    pub fn with_end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
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
                "company_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_name = v; }
                }
                "job_position" => {
                    if let Ok(v) = serde_json::from_value(value) { self.job_position = v; }
                }
                "start_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.start_date = v; }
                }
                "end_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.end_date = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmployeeWorkExperience {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmployeeWorkExperience"
    }
}

impl backbone_core::PersistentEntity for EmployeeWorkExperience {
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

impl backbone_orm::EntityRepoMeta for EmployeeWorkExperience {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["company_name"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for EmployeeWorkExperience entity
///
/// Provides a fluent API for constructing EmployeeWorkExperience instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeWorkExperienceBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    company_name: Option<String>,
    job_position: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl EmployeeWorkExperienceBuilder {
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

    /// Set the company_name field (required)
    pub fn company_name(mut self, value: String) -> Self {
        self.company_name = Some(value);
        self
    }

    /// Set the job_position field (optional)
    pub fn job_position(mut self, value: String) -> Self {
        self.job_position = Some(value);
        self
    }

    /// Set the start_date field (optional)
    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Set the end_date field (optional)
    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    /// Build the EmployeeWorkExperience entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmployeeWorkExperience, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let company_name = self.company_name.ok_or_else(|| "company_name is required".to_string())?;

        Ok(EmployeeWorkExperience {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            company_name,
            job_position: self.job_position,
            start_date: self.start_date,
            end_date: self.end_date,
            metadata: AuditMetadata::default(),
        })
    }
}
