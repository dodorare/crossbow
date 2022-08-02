use crossbundle_tools::types::{apple_bundle::prelude::*, IosTarget};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// #[derive(Debug, Clone, Deserialize, Serialize, Default)]
// pub enum AppleStrategy {
//     #[default]
//     NativeIosApp,
// }

/// Full Apple configuration.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AppleConfig {
    // /// Supported strategies for building working application.
    // pub strategies: Option<Vec<AppleStrategy>>,
    /// `Info.plist` configuration.
    pub info_plist: Option<InfoPlist>,
    /// Path to `Info.plist` file.
    ///
    /// **Important:** If this field specified - `info_plist` property will be ignored.
    pub info_plist_path: Option<PathBuf>,
    /// Apple `resources` directory path relatively to project path.
    pub res: Option<PathBuf>,
    /// Custom Apple `assets` directory path relatively to project path.
    ///
    /// **Important:** This property has higher priority than global property.
    pub assets: Option<PathBuf>,
    /// Apple debug build targets.
    #[serde(default)]
    pub debug_build_targets: Vec<IosTarget>,
    /// Apple release build targets.
    #[serde(default)]
    pub release_build_targets: Vec<IosTarget>,
    // TODO: Add Apple plugins.
    // #[serde(flatten)]
    // pub plugins: ApplePlugins,
}
