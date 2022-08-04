use crate::error::AppleError;
use serde::{Deserialize, Serialize};

/// Supported strategies for building application for Apple devices.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum IosStrategy {
    #[default]
    /// Generate .app and .ipa without XCode. Default strategy.
    #[serde(rename = "native-ipa")]
    NativeIpa,
}

impl std::str::FromStr for IosStrategy {
    type Err = AppleError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "native-ipa" => Ok(Self::NativeIpa),
            _ => Err(AppleError::InvalidBuildStrategy(s.to_owned())),
        }
    }
}
