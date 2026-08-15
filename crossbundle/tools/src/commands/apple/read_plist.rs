use crate::{
    error::{AppleError, Result},
    types::{CrossbowMetadata, update_info_plist_with_default},
};
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

/// Resolves the same typed Info.plist used by Apple builds without writing it.
pub fn resolve_info_plist(
    metadata: &CrossbowMetadata,
    package_name: &str,
    configured_path: Option<&Path>,
) -> Result<InfoPlist> {
    if let Some(path) = configured_path {
        return read_info_plist(path);
    }
    let mut plist = metadata.apple.info_plist.clone().unwrap_or_default();
    update_info_plist_with_default(&mut plist, package_name, metadata.app_name.clone());
    for permission in &metadata.permissions {
        permission.update_info_plist(&mut plist);
    }
    Ok(plist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbow::Permission;

    #[test]
    fn resolution_applies_typed_permissions() {
        let mut metadata = CrossbowMetadata::default();
        metadata.permissions.push(Permission::Camera);
        let plist = resolve_info_plist(&metadata, "example", None).unwrap();
        let description = plist.camera_and_microphone.camera_usage_description;
        assert!(description.is_some());
    }
}
