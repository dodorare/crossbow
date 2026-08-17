use crate::{commands::ExistingFile, error::*};
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};

/// Generates an apple app folder.
pub fn gen_apple_app_folder(
    target_dir: &Path,
    project_name: &str,
    assets_dir: Option<PathBuf>,
    resources_dir: Option<PathBuf>,
) -> Result<PathBuf> {
    if !target_dir.exists() {
        create_dir_all(target_dir)?;
    }
    // Create app folder
    let app_path = target_dir.join(format!("{}.app", project_name));
    remove_dir_all(&app_path).ok();
    create_dir_all(&app_path)?;
    // Copy resources to app folder if provided
    if let Some(resources_dir) = &resources_dir {
        if !resources_dir.exists() {
            return Err(AppleError::ResourcesNotFound.into());
        }
        crate::commands::copy_directory_contents(resources_dir, &app_path, ExistingFile::Skip)?;
    }
    // Copy assets to app folder if provided
    if let Some(assets_dir) = &assets_dir {
        if !assets_dir.exists() {
            return Err(AppleError::AssetsNotFound.into());
        }
        let assets_path = app_path.join("assets");
        create_dir_all(&assets_path)?;
        crate::commands::copy_directory_contents(assets_dir, &assets_path, ExistingFile::Skip)?;
    }
    Ok(app_path)
}
