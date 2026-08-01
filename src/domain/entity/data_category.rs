use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "data_category", rename_all = "snake_case")]
pub enum DataCategory {
    Identity,
    Financial,
    Family,
    Contact,
    Employment,
    Health,
    Biometric,
}

impl std::fmt::Display for DataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identity => write!(f, "identity"),
            Self::Financial => write!(f, "financial"),
            Self::Family => write!(f, "family"),
            Self::Contact => write!(f, "contact"),
            Self::Employment => write!(f, "employment"),
            Self::Health => write!(f, "health"),
            Self::Biometric => write!(f, "biometric"),
        }
    }
}

impl FromStr for DataCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "identity" => Ok(Self::Identity),
            "financial" => Ok(Self::Financial),
            "family" => Ok(Self::Family),
            "contact" => Ok(Self::Contact),
            "employment" => Ok(Self::Employment),
            "health" => Ok(Self::Health),
            "biometric" => Ok(Self::Biometric),
            _ => Err(format!("Unknown DataCategory variant: {}", s)),
        }
    }
}

impl Default for DataCategory {
    fn default() -> Self {
        Self::Identity
    }
}
