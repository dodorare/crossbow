use crate::{
    error::{AndroidError, Result},
    types::{BuildVariables, interpolate_string},
};
use android_manifest::AndroidManifest;
use std::{fs::File, io::BufReader, path::Path};
use xml::reader::XmlEvent as ReaderEvent;

/// Reads and deserializes `AndroidManifest.xml`.
pub fn read_android_manifest(path: &Path) -> Result<AndroidManifest> {
    read_android_manifest_with_variables(path, &BuildVariables::default())
}

/// Reads an Android manifest after expanding allow-listed variables with XML-aware escaping.
pub fn read_android_manifest_with_variables(
    path: &Path,
    variables: &BuildVariables,
) -> Result<AndroidManifest> {
    let file = File::open(path).map_err(|_| {
        AndroidError::FailedToFindAndroidManifest(path.to_string_lossy().to_string())
    })?;
    let xml = interpolate_xml(BufReader::new(file), variables)?;
    Ok(android_manifest::from_reader(xml.as_slice()).map_err(AndroidError::from)?)
}

fn interpolate_xml(reader: impl std::io::Read, variables: &BuildVariables) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut writer = xml::EmitterConfig::new()
        .perform_indent(false)
        .create_writer(&mut output);
    for event in xml::EventReader::new(reader) {
        let mut event = event.map_err(|error| anyhow::anyhow!("invalid Android XML: {error}"))?;
        match &mut event {
            ReaderEvent::StartElement { attributes, .. } => {
                for attribute in attributes {
                    attribute.value = interpolate_string(&attribute.value, variables)?;
                }
            }
            ReaderEvent::Characters(value) | ReaderEvent::CData(value) => {
                *value = interpolate_string(value, variables)?;
            }
            _ => {}
        }
        if let Some(event) = event.as_writer_event() {
            writer
                .write(event)
                .map_err(|error| anyhow::anyhow!("failed to rewrite Android XML: {error}"))?;
        }
    }
    drop(writer);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::parse_project_config;

    fn variables() -> BuildVariables {
        parse_project_config(serde_json::json!({
            "build_variables": {
                "LABEL": { "env": "IGNORED_LABEL", "default": "R&D <Preview> ✓" },
                "CODE": { "env": "IGNORED_CODE", "type": "integer", "default": 42 },
                "LOCATION": { "env": "IGNORED_LOCATION", "default": "auto" }
            }
        }))
        .unwrap()
        .resolve_with(|_| Ok(None))
        .unwrap()
        .build_variables()
        .clone()
    }

    #[test]
    fn interpolates_strings_and_typed_attributes_without_platform_collisions() {
        let xml = br#"<manifest xmlns:android="http://schemas.android.com/apk/res/android"
            package="dev.${applicationId}" android:versionCode="{{crossbow.CODE}}"
            android:installLocation="{{crossbow.LOCATION}}">
            <application android:label="{{crossbow.LABEL}}" />
        </manifest>"#;
        let xml = interpolate_xml(xml.as_slice(), &variables()).unwrap();
        let manifest = android_manifest::from_reader(xml.as_slice()).unwrap();
        assert_eq!(manifest.package.as_deref(), Some("dev.${applicationId}"));
        assert_eq!(manifest.version_code, Some(42));
        assert_eq!(
            manifest.install_location,
            Some(android_manifest::InstallLocation::Auto)
        );
        assert_eq!(
            manifest.application.label.unwrap().to_string(),
            "R&D <Preview> ✓"
        );
    }

    #[test]
    fn rejects_undeclared_placeholders() {
        let xml = br#"<manifest package="{{crossbow.NOT_DECLARED}}"><application /></manifest>"#;
        assert!(
            interpolate_xml(xml.as_slice(), &BuildVariables::default())
                .unwrap_err()
                .to_string()
                .contains("not declared")
        );
    }

    #[test]
    fn reads_boolean_wrappers_without_variables() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("AndroidManifest.xml");
        std::fs::write(
            &path,
            r#"<manifest xmlns:android="http://schemas.android.com/apk/res/android">
                <application android:hasCode="true" />
            </manifest>"#,
        )
        .unwrap();
        let manifest = read_android_manifest(&path).unwrap();
        assert_eq!(
            manifest.application.has_code,
            Some(android_manifest::VarOrBool::Bool(true))
        );
    }
}
