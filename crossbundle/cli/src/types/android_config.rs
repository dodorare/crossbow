use crossbundle_tools::{
    commands::android::*,
    types::{android_manifest::AndroidManifest, AndroidTarget, AppWrapper},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Full Android configuration.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AndroidConfig {
    /// Specifies what application wrapper to use on build.
    #[serde(default)]
    pub app_wrapper: AppWrapper,
    /// AndroidManifest.xml configuration.
    pub manifest: Option<AndroidManifest>,
    /// Path to AndroidManifest.xml file.
    ///
    /// **Important:** If this field specified - `manifest` property will be ignored.
    pub manifest_path: Option<PathBuf>,
    /// Android resources directory path relatively to project path.
    pub res: Option<PathBuf>,
    /// Custom Android assets directory path relatively to project path.
    ///
    /// **Important:** This property has higher priority than global property.
    pub assets: Option<PathBuf>,
    /// Icon generation
    pub icon: Option<PathBuf>,
    /// Android debug build targets.
    #[serde(default)]
    pub debug_build_targets: Vec<AndroidTarget>,
    /// Android release build targets.
    #[serde(default)]
    pub release_build_targets: Vec<AndroidTarget>,
    /// Crossbow Android Plugins.
    #[serde(flatten)]
    pub plugins: AndroidGradlePlugins,
}
