use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "employment_action", rename_all = "snake_case")]
pub enum EmploymentAction {
    Hire,
    Transfer,
    Promotion,
    Demotion,
    RoleChange,
    Reinstatement,
}

impl std::fmt::Display for EmploymentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hire => write!(f, "hire"),
            Self::Transfer => write!(f, "transfer"),
            Self::Promotion => write!(f, "promotion"),
            Self::Demotion => write!(f, "demotion"),
            Self::RoleChange => write!(f, "role_change"),
            Self::Reinstatement => write!(f, "reinstatement"),
        }
    }
}

impl FromStr for EmploymentAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hire" => Ok(Self::Hire),
            "transfer" => Ok(Self::Transfer),
            "promotion" => Ok(Self::Promotion),
            "demotion" => Ok(Self::Demotion),
            "role_change" => Ok(Self::RoleChange),
            "reinstatement" => Ok(Self::Reinstatement),
            _ => Err(format!("Unknown EmploymentAction variant: {}", s)),
        }
    }
}

impl Default for EmploymentAction {
    fn default() -> Self {
        Self::Hire
    }
}
