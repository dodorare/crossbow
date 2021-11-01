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
        .arg(key.key_path)
        .arg(format!("--ks-pass=pass:{}", key.key_pass))
        .arg("--ks-key-alias")
        .arg(key.key_alias);
    build_apks.output_err(true)?;
    Ok(output_apks.to_path_buf())
}
