use crate::error::*;
use crate::tools::AndroidSdk;
use std::path::Path;

/// Installs given APK in emulator or connected device.
/// Runs `adb install -r ...` command.
pub fn install_apk(sdk: &AndroidSdk, apk_path: &Path) -> Result<()> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("install").arg("-r").arg(apk_path);
    adb.output_err(true)?;
    Ok(())
}
