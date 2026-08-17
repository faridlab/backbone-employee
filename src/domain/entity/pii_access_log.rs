use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DataCategory;

/// Strongly-typed ID for PiiAccessLog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PiiAccessLogId(pub Uuid);

impl PiiAccessLogId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PiiAccessLogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PiiAccessLogId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PiiAccessLogId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PiiAccessLogId> for Uuid {
    fn from(id: PiiAccessLogId) -> Self { id.0 }
}

impl AsRef<Uuid> for PiiAccessLogId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PiiAccessLogId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PiiAccessLog {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub accessed_by: Uuid,
    pub data_category: DataCategory,
    pub purpose: Option<String>,
    pub accessed_at: DateTime<Utc>,
}

impl PiiAccessLog {
    /// Create a builder for PiiAccessLog
    pub fn builder() -> PiiAccessLogBuilder {
        <PiiAccessLogBuilder as Default>::default()
    }

    /// Create a new PiiAccessLog with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, accessed_by: Uuid, data_category: DataCategory, accessed_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            accessed_by,
            data_category,
            purpose: None,
            accessed_at,
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PiiAccessLogId {
        PiiAccessLogId(self.id)
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the purpose field (chainable)
    pub fn with_purpose(mut self, value: String) -> Self {
        self.purpose = Some(value);
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
                "accessed_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.accessed_by = v; }
                }
                "data_category" => {
                    if let Ok(v) = serde_json::from_value(value) { self.data_category = v; }
                }
                "purpose" => {
                    if let Ok(v) = serde_json::from_value(value) { self.purpose = v; }
                }
                "accessed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.accessed_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PiiAccessLog {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PiiAccessLog"
    }
}

impl backbone_core::PersistentEntity for PiiAccessLog {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        let _ = ts;
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        let _ = ts;
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        let _ = ts;
    }
}

impl backbone_orm::EntityRepoMeta for PiiAccessLog {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("data_category".to_string(), "data_category".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for PiiAccessLog entity
///
/// Provides a fluent API for constructing PiiAccessLog instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PiiAccessLogBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    accessed_by: Option<Uuid>,
    data_category: Option<DataCategory>,
    purpose: Option<String>,
    accessed_at: Option<DateTime<Utc>>,
}

impl PiiAccessLogBuilder {
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

    /// Set the accessed_by field (required)
    pub fn accessed_by(mut self, value: Uuid) -> Self {
        self.accessed_by = Some(value);
        self
    }

    /// Set the data_category field (required)
    pub fn data_category(mut self, value: DataCategory) -> Self {
        self.data_category = Some(value);
        self
    }

    /// Set the purpose field (optional)
    pub fn purpose(mut self, value: String) -> Self {
        self.purpose = Some(value);
        self
    }

    /// Set the accessed_at field (default: `Utc::now()`)
    pub fn accessed_at(mut self, value: DateTime<Utc>) -> Self {
        self.accessed_at = Some(value);
        self
    }

    /// Build the PiiAccessLog entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PiiAccessLog, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let accessed_by = self.accessed_by.ok_or_else(|| "accessed_by is required".to_string())?;
        let data_category = self.data_category.ok_or_else(|| "data_category is required".to_string())?;

        Ok(PiiAccessLog {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            accessed_by,
            data_category,
            purpose: self.purpose,
            accessed_at: self.accessed_at.unwrap_or(Utc::now()),
        })
    }
}
