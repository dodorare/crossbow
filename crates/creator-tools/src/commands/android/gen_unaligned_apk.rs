use crate::error::*;
use crate::tools::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

/// Generates unaligned APK with given `manifest_path`, `assets` and `res`.
/// Uses `aapt` build tool.
pub fn gen_unaligned_apk(
    sdk: &AndroidSdk,
    project_path: &Path,
    build_dir: &Path,
    manifest_path: &Path,
    assets: Option<PathBuf>,
    res: Option<PathBuf>,
    package_label: String,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
    }
    let apk_path = build_dir.join(format!("{}-unaligned.apk", package_label));
    let mut aapt = sdk.build_tool(bin!("aapt"), Some(project_path))?;
    aapt.arg("package")
        .arg("-f")
        .arg("-F")
        .arg(&apk_path)
        .arg("-M")
        .arg(manifest_path)
        .arg("-I")
        .arg(sdk.android_jar(target_sdk_version)?);
    if let Some(res) = &res {
        aapt.arg("-S").arg(dunce::simplified(res));
    }
    if let Some(assets) = &assets {
        aapt.arg("-A").arg(dunce::simplified(assets));
    }
    aapt.output_err(true)?;
    Ok(apk_path)
}
