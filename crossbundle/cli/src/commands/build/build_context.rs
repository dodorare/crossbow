use crate::{error::*, types::*};
use crossbundle_tools::{commands::*, utils::*};
use std::path::PathBuf;

pub struct BuildContext {
    // Paths
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub target_dir: PathBuf,
    // Configurations
    pub manifest: cargo::core::Manifest,
    pub config: CrossbowMetadata,
}

impl BuildContext {
    /// Create new instance of build context
    pub fn new(config: &Config, target_dir: Option<PathBuf>) -> Result<Self> {
        let workspace_manifest_path = find_workspace_cargo_manifest_path(config.current_dir())?;
        let package_manifest_path = find_package_cargo_manifest_path(config.current_dir())?;
        let project_path = package_manifest_path.parent().unwrap().to_owned();
        let target_dir =
            target_dir.unwrap_or_else(|| workspace_manifest_path.parent().unwrap().join("target"));
        info!("Parsing Cargo.toml");
        let manifest = parse_manifest(&package_manifest_path)?;
        let crossbow_metadata = if let Some(cargo_metadata) = manifest.custom_metadata() {
            cargo_metadata
                .clone()
                .try_into::<CrossbowMetadata>()
                .map_err(|e| Error::InvalidMetadata(e.into()))?
        } else {
            CrossbowMetadata::default()
        };
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            project_path,
            target_dir,
            config: crossbow_metadata,
            manifest,
        })
    }

    /// Get package name from cargo manifest
    pub fn package_name(&self) -> String {
        self.manifest.summary().name().to_string()
    }

    /// Get package version from cargo manifest
    pub fn package_version(&self) -> String {
        self.manifest.summary().version().to_string()
    }
}
