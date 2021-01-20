use crate::{error::Result, utils, Manifest};
use std::path::{Path, PathBuf};

pub struct BuildContext {
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub manifest: Manifest,
    pub target_dir: PathBuf,
}

impl BuildContext {
    pub fn init(current_dir: &Path, target_dir: Option<PathBuf>) -> Result<Self> {
        let workspace_manifest_path = utils::find_workspace_manifest_path(&current_dir)?;
        let package_manifest_path = utils::find_package_manifest_path(&current_dir)?;
        let project_path = package_manifest_path.parent().unwrap().to_owned();
        let target_dir =
            target_dir.unwrap_or_else(|| workspace_manifest_path.parent().unwrap().join("target"));
        info!("Parsing Cargo.toml");
        let manifest = Manifest::from_path_with_metadata(&package_manifest_path)?;
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            project_path,
            manifest,
            target_dir,
        })
    }
}
