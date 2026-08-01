use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "identity_type", rename_all = "snake_case")]
pub enum IdentityType {
    Id,
    Passport,
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Passport => write!(f, "passport"),
        }
    }
}

impl FromStr for IdentityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "id" => Ok(Self::Id),
            "passport" => Ok(Self::Passport),
            _ => Err(format!("Unknown IdentityType variant: {}", s)),
        }
    }
}

impl Default for IdentityType {
    fn default() -> Self {
        Self::Id
    }
}
