use crossbundle_tools::types::{apple_bundle::prelude::*, IosTarget};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Full Apple configuration.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AppleConfig {
    /// `Info.plist` configuration.
    pub info_plist: Option<InfoPlist>,
    /// Path to `Info.plist` file.
    ///
    /// **Important:** If this field specified - `info_plist` property will be ignored.
    pub info_plist_path: Option<PathBuf>,
    /// Apple `resources` directory path relatively to project path.
    ///
    /// If specified more than one - all resources will be placed into one directory.
    #[serde(default)]
    pub resources: Vec<PathBuf>,
    /// Custom Apple `assets` directory path relatively to project path.
    ///
    /// If specified more than one - all assets will be placed into one directory.
    ///
    /// **Important:** This property has higher priority than global property.
    #[serde(default)]
    pub assets: Vec<PathBuf>,
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
