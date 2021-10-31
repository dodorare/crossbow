use crate::commands::android::write_zip;
use crate::error::*;
use std::path::{Path, PathBuf};

pub fn gen_zip_modules(
    build_dir: &Path,
    package_label: &str,
    extracted_apk_files: &PathBuf,
) -> Result<PathBuf> {
    let zip_path = build_dir.join(format!("{}_module.zip", package_label));
    write_zip::dirs_to_write(&extracted_apk_files.to_owned())?;
    write_zip::write(&extracted_apk_files.to_owned(), &zip_path).unwrap();
    Ok(zip_path.to_path_buf())
}
