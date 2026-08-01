use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "employment_state", rename_all = "snake_case")]
pub enum EmploymentState {
    Active,
    Inactive,
}

impl std::fmt::Display for EmploymentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
        }
    }
}

impl FromStr for EmploymentState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err(format!("Unknown EmploymentState variant: {}", s)),
        }
    }
}

impl Default for EmploymentState {
    fn default() -> Self {
        Self::Active
    }
}
