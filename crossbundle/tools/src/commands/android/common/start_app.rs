use crate::{error::*, types::AndroidSdk};

/// Installing APK or AAB on emulator or connected device.
/// Runs `adb shell am start ...` command
pub fn start_app(sdk: &AndroidSdk, package: &str, activity: &str) -> Result<()> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("shell")
        .arg("am")
        .arg("start")
        .arg("-a")
        .arg("android.intent.action.MAIN")
        .arg("-n")
        .arg(format!("{}/{}", package, activity));
    adb.output_err(true)?;
    Ok(())
}
