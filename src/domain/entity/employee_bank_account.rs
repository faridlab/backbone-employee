use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for EmployeeBankAccount
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeBankAccountId(pub Uuid);

impl EmployeeBankAccountId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmployeeBankAccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeBankAccountId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeBankAccountId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmployeeBankAccountId> for Uuid {
    fn from(id: EmployeeBankAccountId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmployeeBankAccountId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmployeeBankAccountId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmployeeBankAccount {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub bank_id: Uuid,
    pub account_number: String,
    pub account_name: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmployeeBankAccount {
    /// Create a builder for EmployeeBankAccount
    pub fn builder() -> EmployeeBankAccountBuilder {
        <EmployeeBankAccountBuilder as Default>::default()
    }

    /// Create a new EmployeeBankAccount with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, bank_id: Uuid, account_number: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            bank_id,
            account_number,
            account_name: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeBankAccountId {
        EmployeeBankAccountId(self.id)
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

    /// Set the account_name field (chainable)
    pub fn with_account_name(mut self, value: String) -> Self {
        self.account_name = Some(value);
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
                "bank_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bank_id = v; }
                }
                "account_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.account_number = v; }
                }
                "account_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.account_name = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmployeeBankAccount {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmployeeBankAccount"
    }
}

impl backbone_core::PersistentEntity for EmployeeBankAccount {
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

impl backbone_orm::EntityRepoMeta for EmployeeBankAccount {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("bank_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["account_number"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for EmployeeBankAccount entity
///
/// Provides a fluent API for constructing EmployeeBankAccount instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeBankAccountBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    bank_id: Option<Uuid>,
    account_number: Option<String>,
    account_name: Option<String>,
}

impl EmployeeBankAccountBuilder {
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

    /// Set the bank_id field (required)
    pub fn bank_id(mut self, value: Uuid) -> Self {
        self.bank_id = Some(value);
        self
    }

    /// Set the account_number field (required)
    pub fn account_number(mut self, value: String) -> Self {
        self.account_number = Some(value);
        self
    }

    /// Set the account_name field (optional)
    pub fn account_name(mut self, value: String) -> Self {
        self.account_name = Some(value);
        self
    }

    /// Build the EmployeeBankAccount entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmployeeBankAccount, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let bank_id = self.bank_id.ok_or_else(|| "bank_id is required".to_string())?;
        let account_number = self.account_number.ok_or_else(|| "account_number is required".to_string())?;

        Ok(EmployeeBankAccount {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            bank_id,
            account_number,
            account_name: self.account_name,
            metadata: AuditMetadata::default(),
        })
    }
}
