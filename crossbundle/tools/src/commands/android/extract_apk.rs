use crate::error::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Allows to extract files from generated APK to use that to generate `.aab`
pub fn extract_apk(apk_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    let filename = Path::new(apk_path);
    let file = fs::File::open(&filename)?;
    let mut apk = zip::ZipArchive::new(file)?;
    apk.extract(output_dir)?;
    Ok(output_dir.to_owned())
}
