use crate::error::{AndroidError, AppleError};
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

/// Supported strategies for building Android application.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AndroidStrategy {
    /// Generate .apk with Gradle. Default strategy.
    #[default]
    #[serde(rename = "gradle-apk")]
    GradleApk,
    /// Generate native .aab without Gradle. This strategy currently doesn't support
    /// Crossbow plugins.
    #[serde(rename = "native-apk")]
    NativeApk,
    /// Generate native .apk without Gradle. This strategy currently doesn't support
    /// Crossbow plugins.
    #[serde(rename = "native-aab")]
    NativeAab,
}

impl std::str::FromStr for AndroidStrategy {
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "gradle-apk" => Ok(Self::GradleApk),
            "native-apk" => Ok(Self::NativeApk),
            "native-aab" => Ok(Self::NativeAab),
            _ => Err(AndroidError::InvalidBuildStrategy(s.to_owned())),
        }
    }
}
