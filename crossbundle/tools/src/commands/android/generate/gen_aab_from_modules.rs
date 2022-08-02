use crate::commands::android::remove;
use crate::commands::android::write_zip;
use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

/// Allows to generate aab from archive with files extracted from apk or set of archives
/// to specified storage
pub fn gen_aab_from_modules(
    build_dir: &Path,
    package_name: &str,
    extracted_apk_path: &Path,
) -> Result<PathBuf> {
    let zip_path = build_dir.join(format!("{}_module.zip", package_name));
    write_zip::dirs_to_write(extracted_apk_path)?;
    write_zip::write(extracted_apk_path, &zip_path)?;

    let unsigned_aab = format!("{}_unsigned.aab", package_name);
    // Remove generated aab file cause of missing overwrite flag
    remove(vec![&build_dir.join(&unsigned_aab)])?;

    let aab = build_dir.join(&unsigned_aab);
    BuildBundle::new(&[zip_path], &aab).run()?;
    Ok(aab)
}
