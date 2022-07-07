use crossbundle_tools::types::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppleConfig {
    pub apple_app_name: Option<String>,
    pub apple_version_name: Option<String>,
    pub apple_version_code: Option<u32>,
    pub apple_min_sdk_version: Option<u32>,
    pub apple_target_sdk_version: Option<u32>,
    pub apple_max_sdk_version: Option<u32>,
    pub apple_icon: Option<String>,

    #[serde(default)]
    pub use_info_plist: bool,
    pub info_plist_path: Option<PathBuf>,

    /// Apple build targets.
    pub apple_build_targets: Option<Vec<AppleTarget>>,
    /// Apple resources directory path relatively to project path.
    pub apple_res: Option<PathBuf>,
    /// Apple assets directory path relatively to project path.
    pub apple_assets: Option<PathBuf>,
}
