use super::SharedBuildCommand;
use crate::{error::*, types::*};
use crossbundle_tools::{
    commands::*,
    types::{CliContext, parse_project_config},
};
use std::path::PathBuf;

pub struct BuildContext {
    // Paths
    pub project_path: PathBuf,
    pub target_dir: PathBuf,
    // Configurations
    pub project: CargoProject,
    pub project_config: ProjectConfig,
}

impl BuildContext {
    /// Create new instance of build context
    pub fn new(context: &CliContext, command: &SharedBuildCommand) -> Result<Self> {
        info!("Reading Cargo metadata");
        let loaded = LoadedProject::load_with_features(
            context.current_dir(),
            &command.features,
            command.all_features,
            command.no_default_features,
        )?;
        let project_path = loaded.root;
        let project = loaded.cargo;
        let target_dir = match &command.target_dir {
            Some(path) if path.is_relative() => {
                std::path::absolute(context.current_dir().join(path))?
            }
            Some(path) => path.clone(),
            None => project.target_directory.clone(),
        };
        let mut project_config = parse_project_config(project.package.metadata.clone())
            .and_then(|metadata| metadata.resolve())
            .map_err(Error::InvalidMetadata)?;
        project_config.resolve_paths(&project_path);
        Ok(Self {
            project_path,
            target_dir,
            project_config,
            project,
        })
    }
}
