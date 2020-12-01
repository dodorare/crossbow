use super::artifact::Artifact;
use super::profile::Profile;

use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct BuildConfig {
    pub artifacts: Option<Vec<Artifact>>,
    pub build_targets: Option<Vec<String>>,
    pub build_dir: Option<PathBuf>,
    pub version_name: Option<String>,
    pub version_code: Option<String>,
    pub profile: Option<Profile>,
}
