use crate::error::*;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn gen_apple_app(
    target_dir: &Path,
    project_name: &str,
    resources_dir: Option<PathBuf>,
    assets_dir: Option<PathBuf>,
) -> Result<PathBuf> {
    if !target_dir.exists() {
        create_dir_all(target_dir)?;
    }
    // Create app folder
    let app_path = target_dir
        .join("apple")
        .join(format!("{}.app", project_name));
    create_dir_all(&app_path)?;
    // Copy resources to app folder if provided
    if let Some(resources_dir) = &resources_dir {
        if !resources_dir.exists() {
            return Err(AppleError::ResourcesNotFound.into());
        }
        let resources_path = app_path.join("res");
        create_dir_all(&resources_path)?;
        copy_dir(resources_dir, &resources_path, &CopyOptions::new())?;
    }
    // Copy assets to app folder if provided
    if let Some(assets_dir) = &assets_dir {
        if !assets_dir.exists() {
            return Err(AppleError::AssetsNotFound.into());
        }
        let assets_path = app_path.join("assets");
        create_dir_all(&assets_path)?;
        copy_dir(assets_dir, &assets_path, &CopyOptions::new())?;
    }
    Ok(app_path)
}
