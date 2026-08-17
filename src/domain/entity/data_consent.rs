use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DataCategory;
use super::LawfulBasis;
use super::AuditMetadata;

/// Strongly-typed ID for DataConsent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataConsentId(pub Uuid);

impl DataConsentId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for DataConsentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DataConsentId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for DataConsentId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<DataConsentId> for Uuid {
    fn from(id: DataConsentId) -> Self { id.0 }
}

impl AsRef<Uuid> for DataConsentId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for DataConsentId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataConsent {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub data_category: DataCategory,
    pub lawful_basis: LawfulBasis,
    pub consent_given_at: Option<DateTime<Utc>>,
    pub consent_method: Option<String>,
    pub privacy_notice_version: Option<String>,
    pub retention_until: Option<NaiveDate>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl DataConsent {
    /// Create a builder for DataConsent
    pub fn builder() -> DataConsentBuilder {
        <DataConsentBuilder as Default>::default()
    }

    /// Create a new DataConsent with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, data_category: DataCategory, lawful_basis: LawfulBasis) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            data_category,
            lawful_basis,
            consent_given_at: None,
            consent_method: None,
            privacy_notice_version: None,
            retention_until: None,
            withdrawn_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> DataConsentId {
        DataConsentId(self.id)
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

    /// Set the consent_given_at field (chainable)
    pub fn with_consent_given_at(mut self, value: DateTime<Utc>) -> Self {
        self.consent_given_at = Some(value);
        self
    }

    /// Set the consent_method field (chainable)
    pub fn with_consent_method(mut self, value: String) -> Self {
        self.consent_method = Some(value);
        self
    }

    /// Set the privacy_notice_version field (chainable)
    pub fn with_privacy_notice_version(mut self, value: String) -> Self {
        self.privacy_notice_version = Some(value);
        self
    }

    /// Set the retention_until field (chainable)
    pub fn with_retention_until(mut self, value: NaiveDate) -> Self {
        self.retention_until = Some(value);
        self
    }

    /// Set the withdrawn_at field (chainable)
    pub fn with_withdrawn_at(mut self, value: DateTime<Utc>) -> Self {
        self.withdrawn_at = Some(value);
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
                "data_category" => {
                    if let Ok(v) = serde_json::from_value(value) { self.data_category = v; }
                }
                "lawful_basis" => {
                    if let Ok(v) = serde_json::from_value(value) { self.lawful_basis = v; }
                }
                "consent_given_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.consent_given_at = v; }
                }
                "consent_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.consent_method = v; }
                }
                "privacy_notice_version" => {
                    if let Ok(v) = serde_json::from_value(value) { self.privacy_notice_version = v; }
                }
                "retention_until" => {
                    if let Ok(v) = serde_json::from_value(value) { self.retention_until = v; }
                }
                "withdrawn_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.withdrawn_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for DataConsent {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "DataConsent"
    }
}

impl backbone_core::PersistentEntity for DataConsent {
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

impl backbone_orm::EntityRepoMeta for DataConsent {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("data_category".to_string(), "data_category".to_string());
        m.insert("lawful_basis".to_string(), "lawful_basis".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for DataConsent entity
///
/// Provides a fluent API for constructing DataConsent instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct DataConsentBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    data_category: Option<DataCategory>,
    lawful_basis: Option<LawfulBasis>,
    consent_given_at: Option<DateTime<Utc>>,
    consent_method: Option<String>,
    privacy_notice_version: Option<String>,
    retention_until: Option<NaiveDate>,
    withdrawn_at: Option<DateTime<Utc>>,
}

impl DataConsentBuilder {
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

    /// Set the data_category field (required)
    pub fn data_category(mut self, value: DataCategory) -> Self {
        self.data_category = Some(value);
        self
    }

    /// Set the lawful_basis field (required)
    pub fn lawful_basis(mut self, value: LawfulBasis) -> Self {
        self.lawful_basis = Some(value);
        self
    }

    /// Set the consent_given_at field (optional)
    pub fn consent_given_at(mut self, value: DateTime<Utc>) -> Self {
        self.consent_given_at = Some(value);
        self
    }

    /// Set the consent_method field (optional)
    pub fn consent_method(mut self, value: String) -> Self {
        self.consent_method = Some(value);
        self
    }

    /// Set the privacy_notice_version field (optional)
    pub fn privacy_notice_version(mut self, value: String) -> Self {
        self.privacy_notice_version = Some(value);
        self
    }

    /// Set the retention_until field (optional)
    pub fn retention_until(mut self, value: NaiveDate) -> Self {
        self.retention_until = Some(value);
        self
    }

    /// Set the withdrawn_at field (optional)
    pub fn withdrawn_at(mut self, value: DateTime<Utc>) -> Self {
        self.withdrawn_at = Some(value);
        self
    }

    /// Build the DataConsent entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<DataConsent, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let data_category = self.data_category.ok_or_else(|| "data_category is required".to_string())?;
        let lawful_basis = self.lawful_basis.ok_or_else(|| "lawful_basis is required".to_string())?;

        Ok(DataConsent {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            data_category,
            lawful_basis,
            consent_given_at: self.consent_given_at,
            consent_method: self.consent_method,
            privacy_notice_version: self.privacy_notice_version,
            retention_until: self.retention_until,
            withdrawn_at: self.withdrawn_at,
            metadata: AuditMetadata::default(),
        })
    }
}
