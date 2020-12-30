use crate::deps::*;
use crate::error::*;
use std::path::{Path, PathBuf};

/// Key for signing APK
#[derive(Debug)]
pub struct Key {
    pub path: PathBuf,
    pub password: String,
}

/// Sign APK with given key
pub fn sign_apk(sdk: &AndroidSdk, apk_path: &Path, key: Key) -> Result<()> {
    let mut apksigner = sdk.build_tool(bat!("apksigner"))?;
    apksigner
        .arg("sign")
        .arg("--ks")
        .arg(&key.path)
        .arg("--ks-pass")
        .arg(format!("pass:{}", &key.password))
        .arg(apk_path);
    apksigner.output_err()?;
    Ok(())
}
