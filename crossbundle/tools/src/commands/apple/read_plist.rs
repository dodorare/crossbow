use crate::error::{AppleError, Result};
use apple_bundle::prelude::InfoPlist;
use std::path::Path;

/// Read file and deserializes `Info.plist` into
/// [`InfoPlist`](apple_bundle::prelude::InfoPlist).
pub fn read_info_plist(path: &Path) -> Result<InfoPlist> {
    if !path.exists() {
        return Err(AppleError::FailedToFindInfoPlist(path.to_string_lossy().to_string()).into());
    }
    let res = apple_bundle::from_file(path)?;
    Ok(res)
}
