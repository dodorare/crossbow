use crate::{error::*, types::*};
use std::path::{Path, PathBuf};

/// Allows to generate aab from archive with files extracted from apk or set of archives
/// to specified storage
pub fn gen_aab_from_modules(
    package_name: &str,
    zip_modules: &[PathBuf],
    build_dir: &Path,
) -> Result<PathBuf> {
    let aab = build_dir.join(format!("{}_unsigned.aab", package_name));
    BuildBundle::new(zip_modules, &aab).run()?;
    Ok(aab)
}

pub fn gen_aab_from_modules_with_toolchain(
    package_name: &str,
    zip_modules: &[PathBuf],
    build_dir: &Path,
    java: &Path,
    bundletool: &Path,
) -> Result<PathBuf> {
    let aab = build_dir.join(format!("{}_unsigned.aab", package_name));
    let mut command = std::process::Command::new(java);
    command
        .arg("-jar")
        .arg(bundletool)
        .arg("build-bundle")
        .arg("--modules")
        .arg(
            zip_modules
                .iter()
                .map(|path| path.to_string_lossy())
                .collect::<Vec<_>>()
                .join(","),
        )
        .arg("--output")
        .arg(&aab);
    command.output_err(true)?;
    Ok(aab)
}
