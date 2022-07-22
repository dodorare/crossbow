use crossbundle_tools::types::{apple_bundle::prelude::*, AppleTarget};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AppleConfig {
    /// Application Name.
    pub app_name: Option<String>,
    /// Application version name.
    pub version_name: Option<String>,
    /// Application version code.
    pub version_code: Option<u32>,
    /// Icon name in resources.
    pub icon: Option<String>,

    /// Path to Info.plist file.
    pub info_plist_path: Option<PathBuf>,

    /// Apple Info.plist configuration.
    pub info_plist: Option<InfoPlist>,
    /// Apple build targets.
    pub build_targets: Option<Vec<AppleTarget>>,
    /// Apple resources directory path relatively to project path.
    pub res: Option<PathBuf>,
    /// Apple assets directory path relatively to project path.
    pub assets: Option<PathBuf>,
}
