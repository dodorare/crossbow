use crossbundle_tools::{
    commands::android::AndroidGradlePlugins,
    types::{android_manifest::AndroidManifest, AndroidTarget},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const MIN_SDK_VERSION: u32 = 19;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AndroidConfig {
    /// Application name.
    pub app_name: Option<String>,
    /// Android package name to place in AndroidManifest.xml.
    pub package: Option<String>,
    /// Application version name.
    pub version_name: Option<String>,
    /// Application version code.
    pub version_code: Option<u32>,
    /// Minimum SDK version supported.
    pub min_sdk_version: Option<u32>,
    /// Target SDK version.
    pub target_sdk_version: Option<u32>,
    /// Maximum SDK version supported.
    pub max_sdk_version: Option<u32>,
    /// Icon name in resources.
    pub icon: Option<String>,

    /// Path to AndroidManifest.xml file.
    pub manifest_path: Option<PathBuf>,

    /// AndroidManifest.xml configuration.
    pub manifest: Option<AndroidManifest>,

    /// Android resources directory path relatively to project path.
    pub res: Option<PathBuf>,
    /// Android assets directory path relatively to project path.
    pub assets: Option<PathBuf>,
    /// Android build targets.
    pub build_targets: Option<Vec<AndroidTarget>>,
    #[serde(flatten)]
    pub plugins: AndroidGradlePlugins,
}
