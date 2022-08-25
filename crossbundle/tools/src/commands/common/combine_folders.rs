use crate::error::Result;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

/// Place all folders' inner files into output directory.
pub fn combine_folders(folder_paths: &[PathBuf], output: &Path) -> Result<()> {
    // Create output directory if it doesn't exist.
    if !output.exists() {
        create_dir_all(output)?;
    }

    // Copy options
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    for folder_path in folder_paths {
        copy_dir(dunce::simplified(folder_path), output, &options)?;
    }
    Ok(())
}
