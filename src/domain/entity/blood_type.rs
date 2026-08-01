use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "blood_type", rename_all = "snake_case")]
pub enum BloodType {
    A,
    B,
    Ab,
    O,
}

impl std::fmt::Display for BloodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "a"),
            Self::B => write!(f, "b"),
            Self::Ab => write!(f, "ab"),
            Self::O => write!(f, "o"),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "ab" => Ok(Self::Ab),
            "o" => Ok(Self::O),
            _ => Err(format!("Unknown BloodType variant: {}", s)),
        }
    }
}

impl Default for BloodType {
    fn default() -> Self {
        Self::A
    }
}
