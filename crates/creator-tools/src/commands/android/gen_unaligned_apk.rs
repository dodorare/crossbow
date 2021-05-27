use android_manifest::*;

use crate::deps::*;
use crate::error::*;
use crate::types::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

/// Generates unaligned APK with given `manifest_path`, `assets` and `res`.
/// Uses `aapt` build tool.
pub fn gen_unaligned_apk(
    sdk: &AndroidSdk,
    build_dir: &Path,
    manifest_path: &Path,
    assets: Option<PathBuf>,
    res: Option<PathBuf>,
    manifest: AndroidManifest,
) -> Result<PathBuf> {
    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
    }
    let apk_path = build_dir.join(format!("{}-unaligned.apk", manifest.package));
    let mut aapt = sdk.build_tool(bin!("aapt"), None)?;
    aapt.arg("package")
        .arg("-f")
        .arg("-F")
        .arg(&apk_path)
        .arg("-M")
        .arg(manifest_path)
        .arg("-I")
        .arg(sdk.android_jar(manifest.uses_sdk.unwrap().target_sdk_version.unwrap() as u32)?);
    if let Some(res) = &res {
        aapt.arg("-S").arg(dunce::simplified(res));
    }
    if let Some(assets) = &assets {
        aapt.arg("-A").arg(dunce::simplified(assets));
    }
    aapt.output_err(true)?;
    Ok(apk_path)
}
