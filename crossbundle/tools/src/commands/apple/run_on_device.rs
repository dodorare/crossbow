use crate::error::*;
use std::{path::Path, process::Command};

/// Installs and launches an iOS application on a physical device with `ios-deploy`.
pub fn launch_ios_device_app(app_path: &Path, debug: bool, device_id: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("ios-deploy");
    if debug {
        cmd.arg("--debug");
    }
    if let Some(device_id) = device_id {
        cmd.args(["--id", device_id]);
    }
    cmd.arg("--bundle").arg(app_path).arg("--no-wifi");
    cmd.output_err(true)?;
    Ok(())
}
