use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "male"),
            Self::Female => write!(f, "female"),
        }
    }
}

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            _ => Err(format!("Unknown Gender variant: {}", s)),
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Self::Male
    }
}
