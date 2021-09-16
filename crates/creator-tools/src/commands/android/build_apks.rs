use crate::commands::android::android_dir;
use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::AabKey;

pub fn build_apks(aab_path: &Path, output_apks: &Path, key: AabKey) -> Result<PathBuf> {
    if !output_apks.exists() {
        std::fs::create_dir_all(&output_apks)?;
    }
    let mut build_apks = Command::new("java");
    build_apks.arg("-jar");
    if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
        build_apks.arg(bundletool_path);
    } else {
        return Err(AndroidError::BundletoolNotFound.into());
    }
    build_apks
        .arg("build-apks")
        .arg("--bundle")
        .arg(aab_path)
        .arg("--output")
        .arg(output_apks)
        .arg("--overwrite")
        .arg("--ks")
        .arg(key.key_path.unwrap_or(android_dir()?.join("aab.keystore")))
        .arg(format!(
            "--ks-pass=pass:{}",
            key.key_pass.unwrap_or("android".to_string())
        ))
        .arg("--ks-key-alias")
        .arg(key.key_alias.unwrap_or("androiddebugkey".to_string()));
    build_apks.output_err(true)?;
    Ok(output_apks.to_path_buf())
}
