use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

pub fn gen_aab_from_modules(
    package_label: &str,
    zip_modules: &[PathBuf],
    build_dir: &Path,
) -> Result<PathBuf> {
    let aab = build_dir.join(format!("{}_unsigned.aab", package_label));
    std::fs::remove_file(&aab)?;
    BuildBundle::new(zip_modules, &aab).run()?;
    Ok(aab)
}
