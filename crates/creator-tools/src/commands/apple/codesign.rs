use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const XCODE_PATH: &str = "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/codesign_allocate";
const BIN_PATH: &str = "/usr/bin/codesign_allocate";

pub fn codesign(
    item_path: &Path,
    force: bool,
    sign_identity: Option<String>,
    entitlements: Option<PathBuf>,
) -> Result<()> {
    if !item_path.exists() {
        return Err(AppleError::CodesignFailed("Item not found".to_owned()).into());
    }
    let mut codesign_allocate_path = XCODE_PATH;
    if !Path::new(codesign_allocate_path).exists() {
        codesign_allocate_path = BIN_PATH;
        if !Path::new(codesign_allocate_path).exists() {
            return Err(AppleError::CodesignAllocateNotFound.into());
        }
    }
    let mut cmd = Command::new("codesign");
    cmd.env("CODESIGN_ALLOCATE", codesign_allocate_path);
    if force {
        cmd.arg("--force");
    }
    if let Some(sign_identity) = sign_identity {
        cmd.args(&["--sign", &sign_identity]);
    } else {
        cmd.args(&["--sign", "-"]);
    }
    cmd.arg("--timestamp=none");
    if let Some(entitlements) = entitlements {
        cmd.args(&["--entitlements", entitlements.to_str().unwrap()]);
    }
    cmd.arg(item_path);
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppleError::CodesignFailed(
            String::from_utf8(output.stderr)
                .unwrap()
                .replace("error: ", "")
                .replace("\n", ""),
        )
        .into());
    }
    Ok(())
}
