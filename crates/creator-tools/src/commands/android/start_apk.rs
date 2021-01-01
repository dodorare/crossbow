use crate::deps::AndroidSdk;
use crate::error::*;

pub fn start_apk(sdk: &AndroidSdk, package_name: &str) -> Result<()> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("shell")
        .arg("am")
        .arg("start")
        .arg("-a")
        .arg("android.intent.action.MAIN")
        .arg("-n")
        .arg(format!("{}/android.app.NativeActivity", package_name));
    adb.output_err(true)?;
    Ok(())
}
