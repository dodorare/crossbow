use crate::error::*;
use std::{
    fs::copy,
    path::{Path, PathBuf},
};

/// Copies profiles into `@app_path/embedded.mobileprovision`.
pub fn copy_profile(
    app_path: &Path,
    profile_name: Option<String>,
    profile_path: Option<PathBuf>,
) -> Result<()> {
    let profile_path = if let Some(path) = profile_path {
        path
    } else if let Some(name) = profile_name {
        dirs::home_dir()
            .unwrap()
            .join("Library")
            .join("MobileDevice")
            .join("Provisioning Profiles")
            .join(name)
    } else {
        return Err(AppleError::CodeSigningProfileNotProvided.into());
    };
    if !profile_path.exists() {
        return Err(AppleError::CodeSigningProfilesNotFound.into());
    }
    let embedded_provisioning_profile = app_path.join("embedded.mobileprovision");
    copy(profile_path, embedded_provisioning_profile)?;
    Ok(())
}
