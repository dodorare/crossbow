use crate::error::AndroidError;
use serde::{Deserialize, Serialize};

/// Stands for what application wrapper to use on build.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AppWrapper {
    #[default]
    #[serde(rename = "ndk-glue")]
    NdkGlue,
    #[serde(rename = "quad")]
    Quad,
}

impl std::str::FromStr for AppWrapper {
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ndk-glue" => Ok(Self::NdkGlue),
            "quad" => Ok(Self::Quad),
            _ => Err(AndroidError::InvalidAppWrapper(s.to_owned())),
        }
    }
}
