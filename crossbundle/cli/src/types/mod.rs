#[cfg(feature = "android")]
pub mod android_config;
#[cfg(feature = "apple")]
pub mod apple_config;

#[cfg(feature = "android")]
pub use android_config::*;
#[cfg(feature = "apple")]
pub use apple_config::*;

use crossbow::Permission;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cross-platform configuration for Android and Apple for Crossbow.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CrossbowMetadata {
    /// Application name for Android and Apple.
    ///
    /// **Important:** This property has lower priority than Android or Apple `manifest`
    /// or `info_plist` property.
    pub app_name: Option<String>,
    /// Assets directory path relatively to project path.
    ///
    /// If specified more than one - all assets will be placed into one directory.
    ///
    /// **Important:** This property has lower priority than Android or Apple `assets`
    /// property.
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    /// Cross-platform permissions for Android and Apple.
    ///
    /// **Important:** This property has lower priority than AndroidManifest or Apple
    /// Info.plist properties.
    #[serde(default)]
    pub permissions: Vec<Permission>,
    /// Cross-platform icon for Android and Apple.
    ///
    /// All necessary icons will be automatically generated for Android and iOS.
    pub icon: Option<PathBuf>,
    #[cfg(feature = "android")]
    #[serde(default)]
    pub android: AndroidConfig,
    #[cfg(feature = "apple")]
    #[serde(default)]
    pub apple: AppleConfig,
}

impl CrossbowMetadata {
    #[cfg(feature = "android")]
    pub fn get_android_assets(&self) -> &[PathBuf] {
        if !self.android.assets.is_empty() {
            &self.android.assets
        } else {
            &self.assets
        }
    }

    #[cfg(feature = "apple")]
    pub fn get_apple_assets(&self) -> &[PathBuf] {
        if !self.apple.assets.is_empty() {
            &self.apple.assets
        } else {
            &self.assets
        }
    }

    #[cfg(feature = "android")]
    pub fn get_android_resources(&self) -> &[PathBuf] {
        &self.android.resources
    }

    #[cfg(feature = "apple")]
    pub fn get_apple_resources(&self) -> &[PathBuf] {
        &self.apple.resources
    }
}
