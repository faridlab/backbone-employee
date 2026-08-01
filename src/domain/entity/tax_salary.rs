use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "tax_salary", rename_all = "snake_case")]
pub enum TaxSalary {
    Taxable,
    NonTaxable,
}

impl std::fmt::Display for TaxSalary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Taxable => write!(f, "taxable"),
            Self::NonTaxable => write!(f, "non_taxable"),
        }
    }
}

impl FromStr for TaxSalary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "taxable" => Ok(Self::Taxable),
            "non_taxable" => Ok(Self::NonTaxable),
            _ => Err(format!("Unknown TaxSalary variant: {}", s)),
        }
    }
}

impl Default for TaxSalary {
    fn default() -> Self {
        Self::Taxable
    }
}
