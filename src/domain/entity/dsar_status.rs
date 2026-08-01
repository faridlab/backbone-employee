use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "dsar_status", rename_all = "snake_case")]
pub enum DsarStatus {
    Pending,
    InProgress,
    Fulfilled,
    Rejected,
}

impl std::fmt::Display for DsarStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Fulfilled => write!(f, "fulfilled"),
            Self::Rejected => write!(f, "rejected"),
        }
    }
}

impl FromStr for DsarStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "in_progress" => Ok(Self::InProgress),
            "fulfilled" => Ok(Self::Fulfilled),
            "rejected" => Ok(Self::Rejected),
            _ => Err(format!("Unknown DsarStatus variant: {}", s)),
        }
    }
}

impl Default for DsarStatus {
    fn default() -> Self {
        Self::Pending
    }
}
