use crate::error::*;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Generates an apple ipa.
pub fn gen_apple_ipa(target_dir: &Path, app_dir: &Path, project_name: &str) -> Result<PathBuf> {
    if !target_dir.exists() {
        create_dir_all(target_dir)?;
    }
    // Create Payload folder
    let payload_path = target_dir.join("Payload");
    remove_dir_all(&payload_path).ok();
    create_dir_all(&payload_path)?;
    // Copy options
    let mut options = CopyOptions::new();
    options.skip_exist = true;
    options.content_only = true;
    copy_dir(app_dir, &payload_path, &options)?;
    // Generate result ipa path
    let ipa_path = target_dir.join(format!("{}.ipa", project_name));
    // Archive Payload into ipa file
    let mut cmd = Command::new("zip");
    cmd.arg("-Xvr");
    cmd.arg(ipa_path.clone());
    cmd.arg(payload_path.clone());
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppleError::ZipCommandFailed.into());
    }
    remove_dir_all(&payload_path).ok();
    Ok(ipa_path)
}
