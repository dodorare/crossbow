use crate::error::*;
use crate::tools::AndroidSdk;
use crate::types::AndroidTarget;

/// Returns current android target.
/// Runs `adb shell getprop ro.product.cpu.abi` command
pub fn detect_abi(sdk: &AndroidSdk) -> Result<AndroidTarget> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("shell").arg("getprop").arg("ro.product.cpu.abi");
    let stdout = adb.output_err(true)?.stdout;
    let abi = std::str::from_utf8(&stdout).or(Err(AndroidError::UnsupportedTarget))?;
    AndroidTarget::from_android_abi(abi.trim())
}
