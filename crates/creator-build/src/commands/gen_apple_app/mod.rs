use super::Command;
use crate::error::*;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::fs::create_dir_all;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GenAppleApp {
    pub target_dir: PathBuf,
    pub project_name: String,
    pub resources_dir: PathBuf,
    pub assets_dir: PathBuf,
}

impl GenAppleApp {
    pub fn new(
        target_dir: PathBuf,
        project_name: String,
        resources_dir: PathBuf,
        assets_dir: PathBuf,
    ) -> Self {
        Self {
            target_dir,
            project_name,
            resources_dir,
            assets_dir,
        }
    }
}

impl Command for GenAppleApp {
    type Deps = ();
    type Output = PathBuf;

    fn run(&self) -> Result<Self::Output> {
        if !self.target_dir.exists() {
            create_dir_all(&self.target_dir)?;
        }
        // Create app folder
        let app_path = self
            .target_dir
            .join("apple")
            .join(format!("{}.app", self.project_name));
        create_dir_all(&app_path)?;
        // Copy resources to app folder
        if !self.resources_dir.exists() {
            return Err(AppleError::ResourcesNotFound.into());
        }
        let resources_path = app_path.join("res");
        create_dir_all(&resources_path)?;
        copy_dir(&self.resources_dir, &resources_path, &CopyOptions::new())?;
        // Copy assets to app folder
        if !self.assets_dir.exists() {
            return Err(AppleError::AssetsNotFound.into());
        }
        let assets_path = app_path.join("assets");
        create_dir_all(&assets_path)?;
        copy_dir(&self.assets_dir, &assets_path, &CopyOptions::new())?;
        Ok(app_path)
    }
}
