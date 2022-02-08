use crate::error::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Allows to extract files from generated APK to use that to generate `.aab`
pub fn extract_archive(archive_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    let filename = Path::new(archive_path);
    let file = fs::File::open(&filename)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(output_dir)?;
    Ok(output_dir.to_owned())
}
