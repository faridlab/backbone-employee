use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for EmployeeBpjs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeBpjsId(pub Uuid);

impl EmployeeBpjsId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmployeeBpjsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeBpjsId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeBpjsId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmployeeBpjsId> for Uuid {
    fn from(id: EmployeeBpjsId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmployeeBpjsId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmployeeBpjsId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmployeeBpjs {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub bpjs_ketenagakerjaan_number: Option<String>,
    pub npp_bpjs_ketenagakerjaan: Option<String>,
    pub bpjs_ketenagakerjaan_date: Option<NaiveDate>,
    pub bpjs_kesehatan_number: Option<String>,
    pub bpjs_kesehatan_family: Option<i32>,
    pub bpjs_kesehatan_date: Option<NaiveDate>,
    pub jaminan_pensiun_date: Option<NaiveDate>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmployeeBpjs {
    /// Create a builder for EmployeeBpjs
    pub fn builder() -> EmployeeBpjsBuilder {
        <EmployeeBpjsBuilder as Default>::default()
    }

    /// Create a new EmployeeBpjs with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            bpjs_ketenagakerjaan_number: None,
            npp_bpjs_ketenagakerjaan: None,
            bpjs_ketenagakerjaan_date: None,
            bpjs_kesehatan_number: None,
            bpjs_kesehatan_family: None,
            bpjs_kesehatan_date: None,
            jaminan_pensiun_date: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeBpjsId {
        EmployeeBpjsId(self.id)
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

    /// Set the bpjs_ketenagakerjaan_number field (chainable)
    pub fn with_bpjs_ketenagakerjaan_number(mut self, value: String) -> Self {
        self.bpjs_ketenagakerjaan_number = Some(value);
        self
    }

    /// Set the npp_bpjs_ketenagakerjaan field (chainable)
    pub fn with_npp_bpjs_ketenagakerjaan(mut self, value: String) -> Self {
        self.npp_bpjs_ketenagakerjaan = Some(value);
        self
    }

    /// Set the bpjs_ketenagakerjaan_date field (chainable)
    pub fn with_bpjs_ketenagakerjaan_date(mut self, value: NaiveDate) -> Self {
        self.bpjs_ketenagakerjaan_date = Some(value);
        self
    }

    /// Set the bpjs_kesehatan_number field (chainable)
    pub fn with_bpjs_kesehatan_number(mut self, value: String) -> Self {
        self.bpjs_kesehatan_number = Some(value);
        self
    }

    /// Set the bpjs_kesehatan_family field (chainable)
    pub fn with_bpjs_kesehatan_family(mut self, value: i32) -> Self {
        self.bpjs_kesehatan_family = Some(value);
        self
    }

    /// Set the bpjs_kesehatan_date field (chainable)
    pub fn with_bpjs_kesehatan_date(mut self, value: NaiveDate) -> Self {
        self.bpjs_kesehatan_date = Some(value);
        self
    }

    /// Set the jaminan_pensiun_date field (chainable)
    pub fn with_jaminan_pensiun_date(mut self, value: NaiveDate) -> Self {
        self.jaminan_pensiun_date = Some(value);
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
                "bpjs_ketenagakerjaan_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bpjs_ketenagakerjaan_number = v; }
                }
                "npp_bpjs_ketenagakerjaan" => {
                    if let Ok(v) = serde_json::from_value(value) { self.npp_bpjs_ketenagakerjaan = v; }
                }
                "bpjs_ketenagakerjaan_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bpjs_ketenagakerjaan_date = v; }
                }
                "bpjs_kesehatan_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bpjs_kesehatan_number = v; }
                }
                "bpjs_kesehatan_family" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bpjs_kesehatan_family = v; }
                }
                "bpjs_kesehatan_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bpjs_kesehatan_date = v; }
                }
                "jaminan_pensiun_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.jaminan_pensiun_date = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmployeeBpjs {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmployeeBpjs"
    }
}

impl backbone_core::PersistentEntity for EmployeeBpjs {
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

impl backbone_orm::EntityRepoMeta for EmployeeBpjs {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for EmployeeBpjs entity
///
/// Provides a fluent API for constructing EmployeeBpjs instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeBpjsBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    bpjs_ketenagakerjaan_number: Option<String>,
    npp_bpjs_ketenagakerjaan: Option<String>,
    bpjs_ketenagakerjaan_date: Option<NaiveDate>,
    bpjs_kesehatan_number: Option<String>,
    bpjs_kesehatan_family: Option<i32>,
    bpjs_kesehatan_date: Option<NaiveDate>,
    jaminan_pensiun_date: Option<NaiveDate>,
}

impl EmployeeBpjsBuilder {
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

    /// Set the bpjs_ketenagakerjaan_number field (optional)
    pub fn bpjs_ketenagakerjaan_number(mut self, value: String) -> Self {
        self.bpjs_ketenagakerjaan_number = Some(value);
        self
    }

    /// Set the npp_bpjs_ketenagakerjaan field (optional)
    pub fn npp_bpjs_ketenagakerjaan(mut self, value: String) -> Self {
        self.npp_bpjs_ketenagakerjaan = Some(value);
        self
    }

    /// Set the bpjs_ketenagakerjaan_date field (optional)
    pub fn bpjs_ketenagakerjaan_date(mut self, value: NaiveDate) -> Self {
        self.bpjs_ketenagakerjaan_date = Some(value);
        self
    }

    /// Set the bpjs_kesehatan_number field (optional)
    pub fn bpjs_kesehatan_number(mut self, value: String) -> Self {
        self.bpjs_kesehatan_number = Some(value);
        self
    }

    /// Set the bpjs_kesehatan_family field (default: `0`)
    pub fn bpjs_kesehatan_family(mut self, value: i32) -> Self {
        self.bpjs_kesehatan_family = Some(value);
        self
    }

    /// Set the bpjs_kesehatan_date field (optional)
    pub fn bpjs_kesehatan_date(mut self, value: NaiveDate) -> Self {
        self.bpjs_kesehatan_date = Some(value);
        self
    }

    /// Set the jaminan_pensiun_date field (optional)
    pub fn jaminan_pensiun_date(mut self, value: NaiveDate) -> Self {
        self.jaminan_pensiun_date = Some(value);
        self
    }

    /// Build the EmployeeBpjs entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmployeeBpjs, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;

        Ok(EmployeeBpjs {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            bpjs_ketenagakerjaan_number: self.bpjs_ketenagakerjaan_number,
            npp_bpjs_ketenagakerjaan: self.npp_bpjs_ketenagakerjaan,
            bpjs_ketenagakerjaan_date: self.bpjs_ketenagakerjaan_date,
            bpjs_kesehatan_number: self.bpjs_kesehatan_number,
            bpjs_kesehatan_family: self.bpjs_kesehatan_family,
            bpjs_kesehatan_date: self.bpjs_kesehatan_date,
            jaminan_pensiun_date: self.jaminan_pensiun_date,
            metadata: AuditMetadata::default(),
        })
    }
}
