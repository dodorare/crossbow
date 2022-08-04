use crate::{commands::android::common::write_zip, error::*};
use std::path::{Path, PathBuf};

/// Allows to generate archive from files extracted from APK
pub fn gen_zip_modules(
    build_dir: &Path,
    package_name: &str,
    extracted_apk_files: &Path,
) -> Result<PathBuf> {
    let zip_path = build_dir.join(format!("{}_module.zip", package_name));
    write_zip::dirs_to_write(extracted_apk_files)?;
    write_zip::write(extracted_apk_files, &zip_path)?;
    Ok(zip_path)
}
