use crate::commands::android::write_zip;
use crate::error::*;
use std::path::{Path, PathBuf};

/// Allows to generate archive from files extracted from APK
pub fn gen_zip_modules(
    build_dir: &Path,
    package_label: &str,
    extracted_apk_files: &Path,
) -> Result<PathBuf> {
    let zip_path = build_dir.join(format!("{}_module.zip", package_label));
    write_zip::dirs_to_write(&extracted_apk_files.to_owned())?;
    write_zip::write(&extracted_apk_files.to_owned(), &zip_path)?;
    Ok(zip_path)
}
