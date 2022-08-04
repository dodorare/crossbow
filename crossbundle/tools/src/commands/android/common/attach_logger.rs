use crate::{error::*, types::AndroidSdk};
use std::process::Command;

/// Returns `adb logcat` command
fn logcat_cmd(sdk: &AndroidSdk) -> Result<Command> {
    let mut adb = sdk.platform_tool(bin!("adb"))?;
    adb.arg("logcat");
    Ok(adb)
}

/// Attach logger to device with filter that passes only Rust Stdout or Stderr.
/// Runs`adb logcat RustStdoutStderr:D '*:S'` command
pub fn attach_logger_only_rust(sdk: &AndroidSdk) -> Result<()> {
    let mut adb = logcat_cmd(sdk)?;
    adb.arg("RustStdoutStderr:D").arg("SAPP:D").arg("*:S");
    adb.spawn()?.wait()?;
    Ok(())
}
