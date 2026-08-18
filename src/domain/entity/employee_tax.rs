use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::PtkpTier;
use super::TaxMethod;
use super::TerCategory;
use super::TaxSalary;
use super::AuditMetadata;

/// Strongly-typed ID for EmployeeTax
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeTaxId(pub Uuid);

impl EmployeeTaxId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmployeeTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeTaxId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeTaxId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmployeeTaxId> for Uuid {
    fn from(id: EmployeeTaxId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmployeeTaxId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmployeeTaxId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmployeeTax {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub npwp_number: Option<String>,
    pub ptkp_override: Option<PtkpTier>,
    pub tax_method: TaxMethod,
    pub ter_category: Option<TerCategory>,
    pub tax_salary: TaxSalary,
    pub taxable_date: Option<NaiveDate>,
    pub beginning_netto: Option<Decimal>,
    pub pph21_paid: Option<Decimal>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmployeeTax {
    /// Create a builder for EmployeeTax
    pub fn builder() -> EmployeeTaxBuilder {
        <EmployeeTaxBuilder as Default>::default()
    }

    /// Create a new EmployeeTax with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, tax_method: TaxMethod, tax_salary: TaxSalary) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            npwp_number: None,
            ptkp_override: None,
            tax_method,
            ter_category: None,
            tax_salary,
            taxable_date: None,
            beginning_netto: None,
            pph21_paid: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeTaxId {
        EmployeeTaxId(self.id)
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

    /// Set the npwp_number field (chainable)
    pub fn with_npwp_number(mut self, value: String) -> Self {
        self.npwp_number = Some(value);
        self
    }

    /// Set the ptkp_override field (chainable)
    pub fn with_ptkp_override(mut self, value: PtkpTier) -> Self {
        self.ptkp_override = Some(value);
        self
    }

    /// Set the ter_category field (chainable)
    pub fn with_ter_category(mut self, value: TerCategory) -> Self {
        self.ter_category = Some(value);
        self
    }

    /// Set the taxable_date field (chainable)
    pub fn with_taxable_date(mut self, value: NaiveDate) -> Self {
        self.taxable_date = Some(value);
        self
    }

    /// Set the beginning_netto field (chainable)
    pub fn with_beginning_netto(mut self, value: Decimal) -> Self {
        self.beginning_netto = Some(value);
        self
    }

    /// Set the pph21_paid field (chainable)
    pub fn with_pph21_paid(mut self, value: Decimal) -> Self {
        self.pph21_paid = Some(value);
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
                "npwp_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.npwp_number = v; }
                }
                "ptkp_override" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ptkp_override = v; }
                }
                "tax_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.tax_method = v; }
                }
                "ter_category" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ter_category = v; }
                }
                "tax_salary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.tax_salary = v; }
                }
                "taxable_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.taxable_date = v; }
                }
                "beginning_netto" => {
                    if let Ok(v) = serde_json::from_value(value) { self.beginning_netto = v; }
                }
                "pph21_paid" => {
                    if let Ok(v) = serde_json::from_value(value) { self.pph21_paid = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmployeeTax {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmployeeTax"
    }
}

impl backbone_core::PersistentEntity for EmployeeTax {
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

impl backbone_orm::EntityRepoMeta for EmployeeTax {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("ptkp_override".to_string(), "ptkp_tier".to_string());
        m.insert("tax_method".to_string(), "tax_method".to_string());
        m.insert("ter_category".to_string(), "ter_category".to_string());
        m.insert("tax_salary".to_string(), "tax_salary".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for EmployeeTax entity
///
/// Provides a fluent API for constructing EmployeeTax instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeTaxBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    npwp_number: Option<String>,
    ptkp_override: Option<PtkpTier>,
    tax_method: Option<TaxMethod>,
    ter_category: Option<TerCategory>,
    tax_salary: Option<TaxSalary>,
    taxable_date: Option<NaiveDate>,
    beginning_netto: Option<Decimal>,
    pph21_paid: Option<Decimal>,
}

impl EmployeeTaxBuilder {
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

    /// Set the npwp_number field (optional)
    pub fn npwp_number(mut self, value: String) -> Self {
        self.npwp_number = Some(value);
        self
    }

    /// Set the ptkp_override field (optional)
    pub fn ptkp_override(mut self, value: PtkpTier) -> Self {
        self.ptkp_override = Some(value);
        self
    }

    /// Set the tax_method field (default: `TaxMethod::default()`)
    pub fn tax_method(mut self, value: TaxMethod) -> Self {
        self.tax_method = Some(value);
        self
    }

    /// Set the ter_category field (optional)
    pub fn ter_category(mut self, value: TerCategory) -> Self {
        self.ter_category = Some(value);
        self
    }

    /// Set the tax_salary field (default: `TaxSalary::default()`)
    pub fn tax_salary(mut self, value: TaxSalary) -> Self {
        self.tax_salary = Some(value);
        self
    }

    /// Set the taxable_date field (optional)
    pub fn taxable_date(mut self, value: NaiveDate) -> Self {
        self.taxable_date = Some(value);
        self
    }

    /// Set the beginning_netto field (optional)
    pub fn beginning_netto(mut self, value: Decimal) -> Self {
        self.beginning_netto = Some(value);
        self
    }

    /// Set the pph21_paid field (optional)
    pub fn pph21_paid(mut self, value: Decimal) -> Self {
        self.pph21_paid = Some(value);
        self
    }

    /// Build the EmployeeTax entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmployeeTax, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;

        Ok(EmployeeTax {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            npwp_number: self.npwp_number,
            ptkp_override: self.ptkp_override,
            tax_method: self.tax_method.unwrap_or_default(),
            ter_category: self.ter_category,
            tax_salary: self.tax_salary.unwrap_or_default(),
            taxable_date: self.taxable_date,
            beginning_netto: self.beginning_netto,
            pph21_paid: self.pph21_paid,
            metadata: AuditMetadata::default(),
        })
    }
}
