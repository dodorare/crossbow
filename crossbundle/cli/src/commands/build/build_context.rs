use super::SharedBuildCommand;
use crate::{error::*, types::*};
use crossbundle_tools::{
    commands::*,
    types::{Config, deserialize_crossbow_metadata},
};
use std::path::PathBuf;

pub struct BuildContext {
    // Paths
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub target_dir: PathBuf,
    // Configurations
    pub project: CargoProject,
    pub config: CrossbowMetadata,
}

impl BuildContext {
    /// Create new instance of build context
    pub fn new(config: &Config, command: &SharedBuildCommand) -> Result<Self> {
        let package_manifest_path = find_package_cargo_manifest_path(config.current_dir())?;
        let project_path = package_manifest_path.parent().unwrap().to_owned();
        info!("Reading Cargo metadata");
        let project = CargoProject::load_with_features(
            &package_manifest_path,
            &command.features,
            command.all_features,
            command.no_default_features,
        )?;
        let workspace_manifest_path = project.workspace_manifest_path.clone();
        let target_dir = command
            .target_dir
            .clone()
            .unwrap_or_else(|| project.target_directory.clone());
        let crossbow_metadata = deserialize_crossbow_metadata(project.package.metadata.clone())
            .map_err(Error::InvalidMetadata)?;
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            project_path,
            target_dir,
            config: crossbow_metadata,
            project,
        })
    }

    /// Get package name from cargo manifest
    pub fn package_name(&self) -> String {
        self.project.package.name.clone()
    }

    /// Get package version from cargo manifest
    pub fn package_version(&self) -> String {
        self.project.package.version.clone()
    }
}
