use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "family_relationship", rename_all = "snake_case")]
pub enum FamilyRelationship {
    Spouse,
    Child,
    Parent,
    Sibling,
    Other,
}

impl std::fmt::Display for FamilyRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spouse => write!(f, "spouse"),
            Self::Child => write!(f, "child"),
            Self::Parent => write!(f, "parent"),
            Self::Sibling => write!(f, "sibling"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl FromStr for FamilyRelationship {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "spouse" => Ok(Self::Spouse),
            "child" => Ok(Self::Child),
            "parent" => Ok(Self::Parent),
            "sibling" => Ok(Self::Sibling),
            "other" => Ok(Self::Other),
            _ => Err(format!("Unknown FamilyRelationship variant: {}", s)),
        }
    }
}

impl Default for FamilyRelationship {
    fn default() -> Self {
        Self::Spouse
    }
}
