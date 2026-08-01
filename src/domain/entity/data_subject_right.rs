use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "data_subject_right", rename_all = "snake_case")]
pub enum DataSubjectRight {
    Access,
    Rectify,
    Erase,
    Export,
    Object,
    Restrict,
}

impl std::fmt::Display for DataSubjectRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Access => write!(f, "access"),
            Self::Rectify => write!(f, "rectify"),
            Self::Erase => write!(f, "erase"),
            Self::Export => write!(f, "export"),
            Self::Object => write!(f, "object"),
            Self::Restrict => write!(f, "restrict"),
        }
    }
}

impl FromStr for DataSubjectRight {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "access" => Ok(Self::Access),
            "rectify" => Ok(Self::Rectify),
            "erase" => Ok(Self::Erase),
            "export" => Ok(Self::Export),
            "object" => Ok(Self::Object),
            "restrict" => Ok(Self::Restrict),
            _ => Err(format!("Unknown DataSubjectRight variant: {}", s)),
        }
    }
}

impl Default for DataSubjectRight {
    fn default() -> Self {
        Self::Access
    }
}
