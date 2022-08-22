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
    adb.arg("RustStdoutStderr:D")
        .arg("SAPP:D")
        .arg("Crossbow:D")
        .arg("CrossbowPlugin:D")
        .arg("*:S");
    adb.spawn()?.wait()?;
    Ok(())
}

/// Attach logger to device with filter that filters only App Stdout or Stderr.
/// Runs`adb logcat --pid=`adb shell pidof -s com.crossbow.game`` command
pub fn attach_logger_only_app(sdk: &AndroidSdk) -> Result<()> {
    let mut adb_shell = sdk.platform_tool(bin!("adb"))?;
    adb_shell.args(["shell", "pidof", "-s", "com.crossbow.game"]);
    let res = adb_shell.output()?.stdout;
    let pid = String::from_utf8_lossy(&res).to_string();

    let mut adb = logcat_cmd(sdk)?;
    adb.args(["--pid", &pid.trim()]);
    adb.spawn()?.wait()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attach_logger_only_app() -> Result<()> {
        let sdk = AndroidSdk::from_env()?;
        attach_logger_only_app(&sdk)?;
        Ok(())
    }
}
