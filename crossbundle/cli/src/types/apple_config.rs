use crossbundle_tools::types::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AppleConfig {
    pub app_name: Option<String>,
    pub version_name: Option<String>,
    pub version_code: Option<u32>,
    pub icon: Option<String>,
    pub info_plist_path: Option<PathBuf>,

    /// Apple build targets.
    pub build_targets: Option<Vec<AppleTarget>>,
    /// Apple resources directory path relatively to project path.
    pub res: Option<PathBuf>,
    /// Apple assets directory path relatively to project path.
    pub assets: Option<PathBuf>,
}
