use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn install_apks() -> Result<()> {
    let mut install_apks = Command::new("java");
    install_apks.arg("-jar");
    if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
        install_apks.arg(bundletool_path);
    } else {
        return Err(AndroidError::BundletoolNotFound.into());
    }
    install_apks.arg("build-apks");
    Ok(())
}
