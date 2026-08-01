use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;
use super::AuditMetadata;

/// Strongly-typed ID for EmployeeEducation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeEducationId(pub Uuid);

impl EmployeeEducationId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmployeeEducationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeEducationId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeEducationId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmployeeEducationId> for Uuid {
    fn from(id: EmployeeEducationId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmployeeEducationId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmployeeEducationId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmployeeEducation {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub institution_name: String,
    pub major: Option<String>,
    pub field: Option<String>,
    pub score: Option<Decimal>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmployeeEducation {
    /// Create a builder for EmployeeEducation
    pub fn builder() -> EmployeeEducationBuilder {
        EmployeeEducationBuilder::default()
    }

    /// Create a new EmployeeEducation with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, institution_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            institution_name,
            major: None,
            field: None,
            score: None,
            start_year: None,
            end_year: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeEducationId {
        EmployeeEducationId(self.id)
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

    /// Set the major field (chainable)
    pub fn with_major(mut self, value: String) -> Self {
        self.major = Some(value);
        self
    }

    /// Set the field field (chainable)
    pub fn with_field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    /// Set the score field (chainable)
    pub fn with_score(mut self, value: Decimal) -> Self {
        self.score = Some(value);
        self
    }

    /// Set the start_year field (chainable)
    pub fn with_start_year(mut self, value: i32) -> Self {
        self.start_year = Some(value);
        self
    }

    /// Set the end_year field (chainable)
    pub fn with_end_year(mut self, value: i32) -> Self {
        self.end_year = Some(value);
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
                "institution_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.institution_name = v; }
                }
                "major" => {
                    if let Ok(v) = serde_json::from_value(value) { self.major = v; }
                }
                "field" => {
                    if let Ok(v) = serde_json::from_value(value) { self.field = v; }
                }
                "score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.score = v; }
                }
                "start_year" => {
                    if let Ok(v) = serde_json::from_value(value) { self.start_year = v; }
                }
                "end_year" => {
                    if let Ok(v) = serde_json::from_value(value) { self.end_year = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmployeeEducation {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmployeeEducation"
    }
}

impl backbone_core::PersistentEntity for EmployeeEducation {
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

impl backbone_orm::EntityRepoMeta for EmployeeEducation {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["institution_name"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for EmployeeEducation entity
///
/// Provides a fluent API for constructing EmployeeEducation instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeEducationBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    institution_name: Option<String>,
    major: Option<String>,
    field: Option<String>,
    score: Option<Decimal>,
    start_year: Option<i32>,
    end_year: Option<i32>,
}

impl EmployeeEducationBuilder {
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

    /// Set the institution_name field (required)
    pub fn institution_name(mut self, value: String) -> Self {
        self.institution_name = Some(value);
        self
    }

    /// Set the major field (optional)
    pub fn major(mut self, value: String) -> Self {
        self.major = Some(value);
        self
    }

    /// Set the field field (optional)
    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    /// Set the score field (optional)
    pub fn score(mut self, value: Decimal) -> Self {
        self.score = Some(value);
        self
    }

    /// Set the start_year field (optional)
    pub fn start_year(mut self, value: i32) -> Self {
        self.start_year = Some(value);
        self
    }

    /// Set the end_year field (optional)
    pub fn end_year(mut self, value: i32) -> Self {
        self.end_year = Some(value);
        self
    }

    /// Build the EmployeeEducation entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmployeeEducation, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let institution_name = self.institution_name.ok_or_else(|| "institution_name is required".to_string())?;

        Ok(EmployeeEducation {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            institution_name,
            major: self.major,
            field: self.field,
            score: self.score,
            start_year: self.start_year,
            end_year: self.end_year,
            metadata: AuditMetadata::default(),
        })
    }
}
