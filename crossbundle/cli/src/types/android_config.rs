use crossbundle_tools::{
    commands::android::AndroidGradlePlugins,
    types::{android_manifest::*, AndroidTarget},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const MIN_SDK_VERSION: u32 = 19;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AndroidConfig {
    /// Application name.
    pub app_name: Option<String>,
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

    /// Android permissions for target sdk version = 22 and lower.
    pub permissions: Option<Vec<UsesPermission>>,
    /// To declare a permission only on devices that support runtime permissions—that is,
    /// devices that run Android 6.0 (API level 23) or higher—include the uses-permission-sdk-23
    /// element instead of the uses-permission element.
    pub permissions_sdk_23: Option<Vec<UsesPermissionSdk23>>,
    /// Declares a single hardware or software android feature that is used by the application
    pub features: Option<Vec<UsesFeature>>,
    /// Android service to place in AndroidManifest.xml.
    pub service: Option<Vec<Service>>,
    /// Android application meta_data to place in AndroidManifest.xml.
    pub meta_data: Option<Vec<MetaData>>,
    /// Android queries to place in AndroidManifest.xml.
    pub queries: Option<Queries>,
    /// Android package name to place in AndroidManifest.xml.
    pub package: Option<String>,
    /// Android resources directory path relatively to project path.
    pub res: Option<PathBuf>,
    /// Android assets directory path relatively to project path.
    pub assets: Option<PathBuf>,
    /// Android build targets.
    pub build_targets: Option<Vec<AndroidTarget>>,
    #[serde(flatten)]
    pub plugins: AndroidGradlePlugins,
}
