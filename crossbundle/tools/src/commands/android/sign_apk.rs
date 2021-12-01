use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

/// Key for signing APK
#[derive(Debug)]
pub struct Key {
    pub path: PathBuf,
    pub password: String,
}

/// Signs APK with given key.
/// Uses `apksigner` build tool
pub fn sign_apk(sdk: &AndroidSdk, apk_path: &Path, key: Key) -> Result<()> {
    let mut apksigner = sdk.build_tool(bat!("apksigner"), None)?;
    apksigner
        .arg("sign")
        .arg("--ks")
        .arg(&key.path)
        .arg("--ks-pass")
        .arg(format!("pass:{}", &key.password))
        .arg(apk_path);
    apksigner.output_err(true)?;
    Ok(())
}
