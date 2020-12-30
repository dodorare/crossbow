use crate::commands::AndroidManifest;
use crate::deps::*;
use crate::error::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

/// Gen unaligned APK with given `manifest_path`, `assets` and `res`
pub fn gen_unaligned_apk(
    sdk: &AndroidSdk,
    build_dir: &Path,
    manifest_path: &Path,
    assets: Option<PathBuf>,
    res: Option<String>,
    manifest: &AndroidManifest,
) -> Result<PathBuf> {
    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
    }
    let apk_path = build_dir.join(format!("{}-unaligned.apk", manifest.package_label));
    let mut aapt = sdk.build_tool(bin!("aapt"))?;
    aapt.arg("package")
        .arg("-f")
        .arg("-F")
        .arg(&apk_path)
        .arg("-M")
        .arg(manifest_path)
        .arg("-I")
        .arg(sdk.android_jar(manifest.target_sdk_version)?);
    if let Some(res) = &res {
        aapt.arg("-S").arg(res);
    }
    if let Some(assets) = &assets {
        aapt.arg("-A").arg(dunce::simplified(assets));
    }
    aapt.output_err()?;
    Ok(apk_path)
}
