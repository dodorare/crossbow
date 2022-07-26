use crate::error::*;
use crate::tools::*;
use android_tools::java_tools::Key;
use std::path::Path;

/// Signs APK with given key.
/// Uses `apksigner` build tool
pub fn sign_apk(sdk: &AndroidSdk, apk_path: &Path, key: Key) -> Result<std::path::PathBuf> {
    let mut apksigner = sdk.build_tool(bat!("apksigner"), None)?;
    apksigner
        .arg("sign")
        .arg("--ks")
        .arg(&key.key_path)
        .arg("--ks-pass")
        .arg(format!("pass:{}", &key.key_pass))
        .arg(apk_path);
    apksigner.output_err(true)?;
    let apk_path = apk_path.to_path_buf();
    Ok(apk_path)
}
