use crate::error::*;
use std::{fs::copy, path::Path};

/// Copies profiles into `@app_path/embedded.mobileprovision`.
pub fn copy_profile(app_path: &Path, profile_path: &Path) -> Result<()> {
    if !profile_path.exists() {
        return Err(AppleError::CodeSigningProfileNotFound(profile_path.to_owned()).into());
    }
    let embedded_provisioning_profile = app_path.join("embedded.mobileprovision");
    copy(profile_path, embedded_provisioning_profile)?;
    Ok(())
}
