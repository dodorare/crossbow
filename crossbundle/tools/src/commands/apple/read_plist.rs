use crate::{
    error::{AppleError, Result},
    types::{
        BuildVariables, CrossbowMetadata, exact_variable, interpolate_string,
        update_info_plist_with_default,
    },
};
use apple_bundle::{plist, prelude::InfoPlist};
use std::path::Path;

/// Read file and deserializes `Info.plist` into
/// [`InfoPlist`](apple_bundle::prelude::InfoPlist).
pub fn read_info_plist(path: &Path) -> Result<InfoPlist> {
    read_info_plist_with_variables(path, &BuildVariables::default())
}

/// Reads an XML or binary Info.plist after recursively expanding declared Crossbow build
/// variables.
fn read_info_plist_with_variables(path: &Path, variables: &BuildVariables) -> Result<InfoPlist> {
    if !path.exists() {
        return Err(AppleError::FailedToFindInfoPlist(path.to_string_lossy().to_string()).into());
    }
    let mut value = plist::Value::from_file(path)?;
    interpolate_plist(&mut value, variables)?;
    Ok(plist::from_value(&value)?)
}

fn interpolate_plist(value: &mut plist::Value, variables: &BuildVariables) -> Result<()> {
    match value {
        plist::Value::Array(values) => {
            for value in values {
                interpolate_plist(value, variables)?;
            }
        }
        plist::Value::Dictionary(values) => {
            for value in values.values_mut() {
                interpolate_plist(value, variables)?;
            }
        }
        plist::Value::String(template) => {
            if let Some(resolved) = exact_variable(template, variables)? {
                *value = plist::to_value(resolved)?;
            } else {
                *template = interpolate_string(template, variables)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Resolves the same typed Info.plist used by Apple builds without writing it.
pub fn resolve_info_plist(
    metadata: &CrossbowMetadata,
    package_name: &str,
    configured_path: Option<&Path>,
) -> Result<InfoPlist> {
    if let Some(path) = configured_path {
        return read_info_plist_with_variables(path, metadata.build_variables());
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

    fn metadata_with_variables() -> CrossbowMetadata {
        crate::types::deserialize_crossbow_metadata(serde_json::json!({
            "build_variables": {
                "NAME": { "env": "IGNORED_NAME", "default": "Crossbow ✓" },
                "FULLSCREEN": { "env": "IGNORED_FULLSCREEN", "type": "boolean", "default": true },
                "BUILD": { "env": "IGNORED_BUILD", "type": "integer", "default": 42 }
            }
        }))
        .unwrap()
    }

    #[test]
    fn resolution_applies_typed_permissions() {
        let mut metadata = CrossbowMetadata::default();
        metadata.permissions.push(Permission::Camera);
        let plist = resolve_info_plist(&metadata, "example", None).unwrap();
        let description = plist.camera_and_microphone.camera_usage_description;
        assert!(description.is_some());
    }

    #[test]
    fn recursively_interpolates_typed_plist_values() {
        let metadata = metadata_with_variables();
        let mut value = plist::Value::Array(vec![
            plist::Value::String("{{crossbow.FULLSCREEN}}".into()),
            plist::Value::Dictionary(plist::Dictionary::from_iter([(
                "nested".to_owned(),
                plist::Value::String("build-{{crossbow.BUILD}}".into()),
            )])),
        ]);
        interpolate_plist(&mut value, metadata.build_variables()).unwrap();
        assert_eq!(
            value,
            plist::Value::Array(vec![
                plist::Value::Boolean(true),
                plist::Value::Dictionary(plist::Dictionary::from_iter([(
                    "nested".to_owned(),
                    plist::Value::String("build-42".into()),
                )])),
            ])
        );
    }

    #[test]
    fn reads_variables_from_xml_and_binary_plists() {
        let metadata = metadata_with_variables();
        for binary in [false, true] {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("Info.plist");
            let value = plist::Value::Dictionary(plist::Dictionary::from_iter([
                (
                    "CFBundleIdentifier".to_owned(),
                    plist::Value::String("dev.crossbow.example".into()),
                ),
                (
                    "CFBundleName".to_owned(),
                    plist::Value::String("{{crossbow.NAME}}".into()),
                ),
                (
                    "UIRequiresFullScreen".to_owned(),
                    plist::Value::String("{{crossbow.FULLSCREEN}}".into()),
                ),
            ]));
            if binary {
                value.to_file_binary(&path).unwrap();
            } else {
                value.to_file_xml(&path).unwrap();
            }
            let plist = read_info_plist_with_variables(&path, metadata.build_variables()).unwrap();
            assert_eq!(plist.naming.bundle_name.as_deref(), Some("Crossbow ✓"));
            assert_eq!(plist.styling.requires_full_screen, Some(true));
        }
    }
}
