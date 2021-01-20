use crate::{error::Result, Manifest};
use creator_tools::{
    commands::{find_package_cargo_manifest_path, find_workspace_cargo_manifest_path},
    utils::Config,
};
use std::path::PathBuf;

pub struct BuildContext {
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub manifest: Manifest,
    pub target_dir: PathBuf,
}

impl BuildContext {
    pub fn init(config: &Config, target_dir: Option<PathBuf>) -> Result<Self> {
        let workspace_manifest_path = find_workspace_cargo_manifest_path(config.current_dir())?;
        let package_manifest_path = find_package_cargo_manifest_path(config.current_dir())?;
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
