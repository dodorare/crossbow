use crate::deps::AndroidSdk;
use crate::error::*;
use std::path::Path;

pub fn install_apk(sdk: &AndroidSdk, apk_path: &Path) -> Result<()> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("install").arg("-r").arg(apk_path);
    adb.output_err(true)?;
    Ok(())
}
