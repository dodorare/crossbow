use crate::{
    error::{AndroidError, Result},
    types::{BuildVariables, interpolate_json_build_variables, interpolate_typed_build_variables},
};
use android_manifest::AndroidManifest;
use std::{fs::File, io::BufReader, path::Path};

/// Reads and deserializes `AndroidManifest.xml`.
pub fn read_android_manifest(path: &Path) -> Result<AndroidManifest> {
    read_android_manifest_with_variables(path, &BuildVariables::default())
}

/// Reads an Android manifest and expands declared Crossbow build variables in its typed XML
/// attributes. Resolving after XML parsing lets the manifest serializer safely escape arbitrary
/// variable values when the generated manifest is written.
pub fn read_android_manifest_with_variables(
    path: &Path,
    variables: &BuildVariables,
) -> Result<AndroidManifest> {
    let file = File::open(path).map_err(|_| {
        AndroidError::FailedToFindAndroidManifest(path.to_string_lossy().to_string())
    })?;
    let reader = BufReader::new(&file);
    let xml = transform_typed_xml_variables(reader, variables)?;
    let manifest = android_manifest::from_reader(xml.as_slice()).map_err(AndroidError::from)?;
    interpolate_android_manifest(manifest, variables)
}

fn transform_typed_xml_variables(
    reader: impl std::io::Read,
    variables: &BuildVariables,
) -> Result<Vec<u8>> {
    use xml::{reader::XmlEvent as ReaderEvent, writer::XmlEvent as WriterEvent};

    let mut output = Vec::new();
    let mut writer = xml::EmitterConfig::new()
        .perform_indent(false)
        .create_writer(&mut output);
    for event in xml::EventReader::new(reader) {
        let mut event = event.map_err(|error| anyhow::anyhow!("invalid Android XML: {error}"))?;
        match &mut event {
            ReaderEvent::StartElement { attributes, .. } => {
                for attribute in attributes {
                    attribute.value =
                        interpolate_typed_build_variables(&attribute.value, variables)?;
                }
                writer
                    .write(
                        event
                            .as_writer_event()
                            .expect("start elements are writable"),
                    )
                    .map_err(|error| anyhow::anyhow!("failed to rewrite Android XML: {error}"))?;
            }
            ReaderEvent::Characters(value) => {
                let value = interpolate_typed_build_variables(value, variables)?;
                writer
                    .write(WriterEvent::characters(&value))
                    .map_err(|error| anyhow::anyhow!("failed to rewrite Android XML: {error}"))?;
            }
            ReaderEvent::CData(value) => {
                let value = interpolate_typed_build_variables(value, variables)?;
                writer
                    .write(WriterEvent::cdata(&value))
                    .map_err(|error| anyhow::anyhow!("failed to rewrite Android XML: {error}"))?;
            }
            _ => {
                if let Some(event) = event.as_writer_event() {
                    writer.write(event).map_err(|error| {
                        anyhow::anyhow!("failed to rewrite Android XML: {error}")
                    })?;
                }
            }
        }
    }
    drop(writer);
    Ok(output)
}

fn interpolate_android_manifest(
    manifest: AndroidManifest,
    variables: &BuildVariables,
) -> Result<AndroidManifest> {
    let mut value = serde_json::to_value(manifest)
        .map_err(|error| anyhow::anyhow!("failed to inspect Android manifest: {error}"))?;
    interpolate_json_build_variables(&mut value, variables)?;
    serde_json::from_value(value)
        .map_err(|error| anyhow::anyhow!("invalid resolved Android manifest: {error}").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::deserialize_crossbow_metadata;

    fn variables() -> BuildVariables {
        deserialize_crossbow_metadata(serde_json::json!({
            "build_variables": {
                "LABEL": { "env": "IGNORED_LABEL", "default": "R&D <Preview> ✓" },
                "CODE": { "env": "IGNORED_CODE", "type": "integer", "default": 42 },
                "LOCATION": { "env": "IGNORED_LOCATION", "default": "auto" }
            }
        }))
        .unwrap()
        .build_variables
    }

    #[test]
    fn transforms_xml_values_safely_without_touching_android_placeholders() {
        let xml = br#"<?xml version="1.0" encoding="utf-8"?>
            <manifest xmlns:android="http://schemas.android.com/apk/res/android"
                package="com.example.${applicationId}">
                <application android:label="{{crossbow.LABEL}}" />
            </manifest>"#;
        let xml = transform_typed_xml_variables(xml.as_slice(), &variables()).unwrap();
        let manifest = android_manifest::from_reader(xml.as_slice()).unwrap();
        let manifest = interpolate_android_manifest(manifest, &variables()).unwrap();
        assert_eq!(
            manifest.application.label.unwrap().to_string(),
            "R&D <Preview> ✓"
        );
        assert_eq!(
            manifest.package.as_deref(),
            Some("com.example.${applicationId}")
        );
    }

    #[test]
    fn resolves_typed_attributes_before_xml_deserialization() {
        let xml = br#"<manifest xmlns:android="http://schemas.android.com/apk/res/android"
            package="dev.crossbow.example" android:versionCode="{{crossbow.CODE}}"
            android:installLocation="{{crossbow.LOCATION}}">
            <application android:label="{{crossbow.LABEL}}" />
        </manifest>"#;
        let transformed = transform_typed_xml_variables(xml.as_slice(), &variables()).unwrap();
        let transformed = String::from_utf8(transformed).unwrap();
        assert!(transformed.contains("android:versionCode=\"42\""));
        assert!(transformed.contains("android:installLocation=\"auto\""));
        assert!(transformed.contains("{{crossbow.LABEL}}"));
        let manifest = android_manifest::from_reader(transformed.as_bytes()).unwrap();
        let manifest = interpolate_android_manifest(manifest, &variables()).unwrap();
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
    fn rejects_undeclared_manifest_placeholders() {
        let xml = br#"<manifest package="{{crossbow.NOT_DECLARED}}"><application /></manifest>"#;
        let error = transform_typed_xml_variables(xml.as_slice(), &BuildVariables::default())
            .unwrap_err()
            .to_string();
        assert!(error.contains("not declared"));

        let manifest = android_manifest::from_reader(xml.as_slice()).unwrap();
        let error = interpolate_android_manifest(manifest, &BuildVariables::default())
            .unwrap_err()
            .to_string();
        assert!(error.contains("not declared"));
    }
}
