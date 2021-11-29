use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

/// Allows to generate aab from archive with files extracted from apk or set of archives
/// to specified storage
pub fn gen_aab_from_modules(
    package_label: &str,
    zip_modules: &[PathBuf],
    build_dir: &Path,
) -> Result<PathBuf> {
    let aab = build_dir.join(format!("{}_unsigned.aab", package_label));
    BuildBundle::new(zip_modules, &aab).run()?;
    Ok(aab)
}
