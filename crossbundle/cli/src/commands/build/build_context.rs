use super::SharedBuildCommand;
use crate::{error::*, types::*};
use crossbundle_tools::{
    commands::*,
    types::{Config, deserialize_crossbow_metadata},
};
use std::path::PathBuf;

pub struct BuildContext {
    // Paths
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
        let target_dir = match &command.target_dir {
            Some(path) if path.is_relative() => {
                std::path::absolute(config.current_dir().join(path))?
            }
            Some(path) => path.clone(),
            None => project.target_directory.clone(),
        };
        let crossbow_metadata = deserialize_crossbow_metadata(project.package.metadata.clone())
            .map_err(Error::InvalidMetadata)?;
        Ok(Self {
            project_path,
            target_dir,
            config: crossbow_metadata,
            project,
        })
    }
}
