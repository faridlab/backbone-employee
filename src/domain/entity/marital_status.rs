use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "marital_status", rename_all = "snake_case")]
pub enum MaritalStatus {
    Single,
    Married,
    Widow,
    Widower,
}

impl std::fmt::Display for MaritalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Married => write!(f, "married"),
            Self::Widow => write!(f, "widow"),
            Self::Widower => write!(f, "widower"),
        }
    }
}

impl FromStr for MaritalStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "single" => Ok(Self::Single),
            "married" => Ok(Self::Married),
            "widow" => Ok(Self::Widow),
            "widower" => Ok(Self::Widower),
            _ => Err(format!("Unknown MaritalStatus variant: {}", s)),
        }
    }
}

impl Default for MaritalStatus {
    fn default() -> Self {
        Self::Single
    }
}
