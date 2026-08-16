use crate::error::*;
use std::{path::Path, process::Command};

/// Signs app. Runs `codesign ...` command.
pub fn codesign(
    item_path: &Path,
    force: bool,
    sign_identity: Option<&str>,
    entitlements: Option<&Path>,
) -> Result<()> {
    if !item_path.exists() {
        return Err(Error::PathNotFound(item_path.to_owned()));
    }
    let mut xcrun = Command::new("xcrun");
    xcrun.args(["--find", "codesign_allocate"]);
    let output = xcrun.output_err(false)?;
    let codesign_allocate =
        String::from_utf8(output.stdout).map_err(|error| Error::OtherError(Box::new(error)))?;

    let mut cmd = Command::new("codesign");
    cmd.env("CODESIGN_ALLOCATE", codesign_allocate.trim());
    if force {
        cmd.arg("--force");
    }
    cmd.arg("--sign")
        .arg(sign_identity.unwrap_or("-"))
        .arg("--timestamp=none");
    if let Some(entitlements) = entitlements {
        cmd.arg("--entitlements").arg(entitlements);
    }
    cmd.arg(item_path);
    cmd.output_err(false)?;
    Ok(())
}
