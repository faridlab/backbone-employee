use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::Gender;
use super::MaritalStatus;
use super::BloodType;
use super::AuditMetadata;

/// Strongly-typed ID for Employee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeId(pub Uuid);

impl EmployeeId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmployeeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmployeeId> for Uuid {
    fn from(id: EmployeeId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmployeeId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmployeeId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_number: String,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub mobile_phone: Option<String>,
    pub phone: Option<String>,
    pub birth_place: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub marital_status: Option<MaritalStatus>,
    pub blood_type: Option<BloodType>,
    pub religion_id: Option<Uuid>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Employee {
    /// Create a builder for Employee
    pub fn builder() -> EmployeeBuilder {
        EmployeeBuilder::default()
    }

    /// Create a new Employee with required fields
    pub fn new(company_id: Uuid, employee_number: String, first_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_number,
            user_id: None,
            first_name,
            last_name: None,
            email: None,
            mobile_phone: None,
            phone: None,
            birth_place: None,
            birth_date: None,
            gender: None,
            marital_status: None,
            blood_type: None,
            religion_id: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeId {
        EmployeeId(self.id)
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

    /// Set the user_id field (chainable)
    pub fn with_user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the last_name field (chainable)
    pub fn with_last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    /// Set the email field (chainable)
    pub fn with_email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the mobile_phone field (chainable)
    pub fn with_mobile_phone(mut self, value: String) -> Self {
        self.mobile_phone = Some(value);
        self
    }

    /// Set the phone field (chainable)
    pub fn with_phone(mut self, value: String) -> Self {
        self.phone = Some(value);
        self
    }

    /// Set the birth_place field (chainable)
    pub fn with_birth_place(mut self, value: String) -> Self {
        self.birth_place = Some(value);
        self
    }

    /// Set the birth_date field (chainable)
    pub fn with_birth_date(mut self, value: NaiveDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    /// Set the gender field (chainable)
    pub fn with_gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Set the marital_status field (chainable)
    pub fn with_marital_status(mut self, value: MaritalStatus) -> Self {
        self.marital_status = Some(value);
        self
    }

    /// Set the blood_type field (chainable)
    pub fn with_blood_type(mut self, value: BloodType) -> Self {
        self.blood_type = Some(value);
        self
    }

    /// Set the religion_id field (chainable)
    pub fn with_religion_id(mut self, value: Uuid) -> Self {
        self.religion_id = Some(value);
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
                "employee_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.employee_number = v; }
                }
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "first_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.first_name = v; }
                }
                "last_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_name = v; }
                }
                "email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email = v; }
                }
                "mobile_phone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mobile_phone = v; }
                }
                "phone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.phone = v; }
                }
                "birth_place" => {
                    if let Ok(v) = serde_json::from_value(value) { self.birth_place = v; }
                }
                "birth_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.birth_date = v; }
                }
                "gender" => {
                    if let Ok(v) = serde_json::from_value(value) { self.gender = v; }
                }
                "marital_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.marital_status = v; }
                }
                "blood_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.blood_type = v; }
                }
                "religion_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.religion_id = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Employee {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Employee"
    }
}

impl backbone_core::PersistentEntity for Employee {
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

impl backbone_orm::EntityRepoMeta for Employee {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("religion_id".to_string(), "uuid".to_string());
        m.insert("gender".to_string(), "gender".to_string());
        m.insert("marital_status".to_string(), "marital_status".to_string());
        m.insert("blood_type".to_string(), "blood_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["employee_number", "first_name"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for Employee entity
///
/// Provides a fluent API for constructing Employee instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeBuilder {
    company_id: Option<Uuid>,
    employee_number: Option<String>,
    user_id: Option<Uuid>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    mobile_phone: Option<String>,
    phone: Option<String>,
    birth_place: Option<String>,
    birth_date: Option<NaiveDate>,
    gender: Option<Gender>,
    marital_status: Option<MaritalStatus>,
    blood_type: Option<BloodType>,
    religion_id: Option<Uuid>,
}

impl EmployeeBuilder {
    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the employee_number field (required)
    pub fn employee_number(mut self, value: String) -> Self {
        self.employee_number = Some(value);
        self
    }

    /// Set the user_id field (optional)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the first_name field (required)
    pub fn first_name(mut self, value: String) -> Self {
        self.first_name = Some(value);
        self
    }

    /// Set the last_name field (optional)
    pub fn last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    /// Set the email field (optional)
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the mobile_phone field (optional)
    pub fn mobile_phone(mut self, value: String) -> Self {
        self.mobile_phone = Some(value);
        self
    }

    /// Set the phone field (optional)
    pub fn phone(mut self, value: String) -> Self {
        self.phone = Some(value);
        self
    }

    /// Set the birth_place field (optional)
    pub fn birth_place(mut self, value: String) -> Self {
        self.birth_place = Some(value);
        self
    }

    /// Set the birth_date field (optional)
    pub fn birth_date(mut self, value: NaiveDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    /// Set the gender field (optional)
    pub fn gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Set the marital_status field (optional)
    pub fn marital_status(mut self, value: MaritalStatus) -> Self {
        self.marital_status = Some(value);
        self
    }

    /// Set the blood_type field (optional)
    pub fn blood_type(mut self, value: BloodType) -> Self {
        self.blood_type = Some(value);
        self
    }

    /// Set the religion_id field (optional)
    pub fn religion_id(mut self, value: Uuid) -> Self {
        self.religion_id = Some(value);
        self
    }

    /// Build the Employee entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Employee, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_number = self.employee_number.ok_or_else(|| "employee_number is required".to_string())?;
        let first_name = self.first_name.ok_or_else(|| "first_name is required".to_string())?;

        Ok(Employee {
            id: Uuid::new_v4(),
            company_id,
            employee_number,
            user_id: self.user_id,
            first_name,
            last_name: self.last_name,
            email: self.email,
            mobile_phone: self.mobile_phone,
            phone: self.phone,
            birth_place: self.birth_place,
            birth_date: self.birth_date,
            gender: self.gender,
            marital_status: self.marital_status,
            blood_type: self.blood_type,
            religion_id: self.religion_id,
            metadata: AuditMetadata::default(),
        })
    }
}
