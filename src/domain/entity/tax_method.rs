use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "tax_method", rename_all = "snake_case")]
pub enum TaxMethod {
    Gross,
    GrossUp,
    Netto,
}

impl std::fmt::Display for TaxMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gross => write!(f, "gross"),
            Self::GrossUp => write!(f, "gross_up"),
            Self::Netto => write!(f, "netto"),
        }
    }
}

impl FromStr for TaxMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gross" => Ok(Self::Gross),
            "gross_up" => Ok(Self::GrossUp),
            "netto" => Ok(Self::Netto),
            _ => Err(format!("Unknown TaxMethod variant: {}", s)),
        }
    }
}

impl Default for TaxMethod {
    fn default() -> Self {
        Self::Gross
    }
}
