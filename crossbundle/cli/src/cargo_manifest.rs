pub use cargo_toml::Package as CargoPackage;
use crossbundle_tools::types::{android_manifest::UsesPermission, *};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type CargoManifest = cargo_toml::Manifest<Metadata>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub app_name: Option<String>,
    pub version_name: Option<String>,
    pub version_code: Option<String>,
    pub min_sdk_version: Option<u32>,
    pub target_sdk_version: Option<u32>,
    pub max_sdk_version: Option<u32>,
    pub icon: Option<String>,

    #[serde(default)]
    pub android_permissions: Vec<UsesPermission>,

    #[serde(default)]
    pub use_android_manifest: bool,
    pub android_manifest_path: Option<PathBuf>,

    #[serde(default)]
    pub use_info_plist: bool,
    pub info_plist_path: Option<PathBuf>,

    /// Android package name to place in AndroidManifest.xml.
    pub android_package_name: Option<String>,
    /// Android resources directory path relatively to project path.
    pub android_res: Option<PathBuf>,
    /// Android assets directory path relatively to project path.
    pub android_assets: Option<PathBuf>,
    /// Android build targets.
    pub android_build_targets: Option<Vec<AndroidTarget>>,

    /// Apple build targets.
    pub apple_build_targets: Option<Vec<AppleTarget>>,
    /// Apple resources directory path relatively to project path.
    pub apple_res: Option<PathBuf>,
    /// Apple assets directory path relatively to project path.
    pub apple_assets: Option<PathBuf>,
}
