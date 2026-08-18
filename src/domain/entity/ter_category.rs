use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "ter_category", rename_all = "snake_case")]
pub enum TerCategory {
    TerA,
    TerB,
    TerC,
}

impl std::fmt::Display for TerCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TerA => write!(f, "ter_a"),
            Self::TerB => write!(f, "ter_b"),
            Self::TerC => write!(f, "ter_c"),
        }
    }
}

impl FromStr for TerCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ter_a" => Ok(Self::TerA),
            "ter_b" => Ok(Self::TerB),
            "ter_c" => Ok(Self::TerC),
            _ => Err(format!("Unknown TerCategory variant: {}", s)),
        }
    }
}

impl Default for TerCategory {
    fn default() -> Self {
        Self::TerA
    }
}
