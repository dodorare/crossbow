use crossbundle_tools::types::{
    android_manifest::{Service, UsesFeature, UsesPermission, UsesPermissionSdk23},
    *,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub app_name: Option<String>,
    pub version_name: Option<String>,
    pub version_code: Option<u32>,
    pub min_sdk_version: Option<u32>,
    pub target_sdk_version: Option<u32>,
    pub max_sdk_version: Option<u32>,
    pub icon: Option<String>,

    #[serde(default)]
    pub use_android_manifest: bool,
    pub android_manifest_path: Option<PathBuf>,

    #[serde(default)]
    pub use_info_plist: bool,
    pub info_plist_path: Option<PathBuf>,

    /// Android package name to place in AndroidManifest.xml.
    pub android_package: Option<String>,
    /// Android resources directory path relatively to project path.
    pub android_res: Option<PathBuf>,
    /// Android assets directory path relatively to project path.
    pub android_assets: Option<PathBuf>,
    /// Android build targets.
    pub android_build_targets: Option<Vec<AndroidTarget>>,

    // Android permissions for target sdk version = 22 and lower
    pub android_permissions: Option<Vec<UsesPermission>>,
    /// To declare a permission only on devices that support runtime permissions—that is,
    /// devices that run Android 6.0 (API level 23) or higher—include the uses-permission-sdk-23
    /// element instead of the uses-permission element.
    pub android_permissions_sdk_23: Option<Vec<UsesPermissionSdk23>>,
    /// Declares a single hardware or software android feature that is used by the application
    pub android_features: Option<Vec<UsesFeature>>,

    /// Android service to place in AndroidManifest.xml.
    pub android_service: Option<Vec<Service>>,

    /// Apple build targets.
    pub apple_build_targets: Option<Vec<AppleTarget>>,
    /// Apple resources directory path relatively to project path.
    pub apple_res: Option<PathBuf>,
    /// Apple assets directory path relatively to project path.
    pub apple_assets: Option<PathBuf>,
}
