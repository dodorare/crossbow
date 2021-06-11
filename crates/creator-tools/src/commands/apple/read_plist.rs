use crate::error::{AppleError, Result};
use apple_bundle::prelude::InfoPlist;
use std::path::Path;

/// Read file and deserializes `Info.plist` into [`InfoPlist`](apple_bundle::prelude::InfoPlist).
pub fn read_info_plist(path: &Path) -> Result<InfoPlist> {
    apple_bundle::from_file(path)
        .map_err(|_| AppleError::FailedToFindInfoPlist(path.to_string_lossy().to_string()).into())
}
