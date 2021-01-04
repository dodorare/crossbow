use creator_tools::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppleMetadata {
    /// Build targets.
    pub build_targets: Option<Vec<AppleTarget>>,
    /// Resources directory path relatively to project path.
    #[serde(rename = "res")]
    pub resources: Option<String>,
    /// Assets directory path relatively to project path.
    pub assets: Option<String>,
    /// Info.plist specification.
    #[serde(rename = "info-plist")]
    pub info_plist: InfoPlist,
}
