use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "record_change_status", rename_all = "snake_case")]
pub enum RecordChangeStatus {
    Pending,
    Ready,
    Applied,
    Rejected,
    Cancelled,
}

impl std::fmt::Display for RecordChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Ready => write!(f, "ready"),
            Self::Applied => write!(f, "applied"),
            Self::Rejected => write!(f, "rejected"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl FromStr for RecordChangeStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "ready" => Ok(Self::Ready),
            "applied" => Ok(Self::Applied),
            "rejected" => Ok(Self::Rejected),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown RecordChangeStatus variant: {}", s)),
        }
    }
}

impl Default for RecordChangeStatus {
    fn default() -> Self {
        Self::Pending
    }
}
