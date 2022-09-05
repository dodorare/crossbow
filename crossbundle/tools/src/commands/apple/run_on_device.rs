use crate::error::*;
use std::{path::Path, process::Command};

/// Runs and debugs app on device.
/// Runs `ios-deploy ...` command.
pub fn run_and_debug(
    app_path: &Path,
    debug: bool,
    just_launch: bool,
    non_interactive: bool,
    id: Option<&String>,
) -> Result<()> {
    let mut cmd = Command::new("ios-deploy");
    if debug {
        cmd.arg("--debug");
    }
    if just_launch {
        cmd.arg("--justlaunch");
    }
    if let Some(id) = id {
        cmd.args(["--id", id]);
    }
    cmd.arg("--bundle");
    cmd.arg(app_path);
    if non_interactive {
        cmd.arg("--noninteractive");
    }
    cmd.arg("--no-wifi");
    cmd.output_err(true)?;
    Ok(())
}
