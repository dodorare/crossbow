use crate::error::Result;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use super::{ExistingFile, copy_directory_contents};

/// Place all folders' inner files into output directory.
pub fn combine_folders(folder_paths: &[PathBuf], output: &Path) -> Result<()> {
    // Create output directory if it doesn't exist.
    if !output.exists() {
        create_dir_all(output)?;
    }

    for folder_path in folder_paths {
        copy_directory_contents(
            dunce::simplified(folder_path),
            output,
            ExistingFile::Overwrite,
        )?;
    }
    Ok(())
}
