use crate::{
    error::{AndroidError, Result},
    types::{BuildVariableValue, BuildVariables},
};
use android_manifest::AndroidManifest;
use std::fs::create_dir_all;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Saves given manifest in new `AndroidManifest.xml` file
pub fn save_android_manifest(out_dir: &Path, manifest: &AndroidManifest) -> Result<PathBuf> {
    save_android_manifest_with_variables(out_dir, manifest, &BuildVariables::default())
}

/// Saves a manifest while preserving arbitrary characters imported through build variables.
/// `android-manifest` escapes string-like attribute wrappers before the XML writer escapes the
/// complete attribute; this format-aware pass removes that extra layer only for declared values.
pub fn save_android_manifest_with_variables(
    out_dir: &Path,
    manifest: &AndroidManifest,
    variables: &BuildVariables,
) -> Result<PathBuf> {
    if !out_dir.exists() {
        create_dir_all(out_dir)?;
    }
    let manifest_path = out_dir.join("AndroidManifest.xml");
    let mut file = File::create(&manifest_path)?;
    let given_xml = android_manifest::to_string_pretty(manifest).map_err(AndroidError::from)?;
    let given_xml = normalize_variable_escaping(&given_xml, variables)?;
    file.write_all(&given_xml)?;
    Ok(manifest_path)
}

fn normalize_variable_escaping(xml: &str, variables: &BuildVariables) -> Result<Vec<u8>> {
    use xml::reader::XmlEvent as ReaderEvent;

    let mut replacements = variables
        .iter()
        .filter_map(|(_, value)| match value {
            BuildVariableValue::String(value) => {
                let escaped = xml::escape::escape_str_attribute(value).into_owned();
                (escaped != *value).then_some((escaped, value.as_str()))
            }
            BuildVariableValue::Integer(_) | BuildVariableValue::Boolean(_) => None,
        })
        .collect::<Vec<_>>();
    replacements.sort_by_key(|replacement| std::cmp::Reverse(replacement.0.len()));
    if replacements.is_empty() {
        return Ok(xml.as_bytes().to_vec());
    }

    let mut output = Vec::new();
    let mut writer = xml::EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut output);
    for event in xml::EventReader::new(xml.as_bytes()) {
        let mut event = event.map_err(|error| anyhow::anyhow!("invalid generated XML: {error}"))?;
        match &mut event {
            ReaderEvent::StartElement { attributes, .. } => {
                for attribute in attributes {
                    for (escaped, raw) in &replacements {
                        attribute.value = attribute.value.replace(escaped, raw);
                    }
                }
            }
            ReaderEvent::Characters(value) | ReaderEvent::CData(value) => {
                for (escaped, raw) in &replacements {
                    *value = value.replace(escaped, raw);
                }
            }
            _ => {}
        }
        if let Some(event) = event.as_writer_event() {
            writer
                .write(event)
                .map_err(|error| anyhow::anyhow!("failed to write generated XML: {error}"))?;
        }
    }
    drop(writer);
    Ok(output)
}

#[cfg(test)]
mod build_variable_tests {
    use super::*;
    use crate::types::deserialize_crossbow_metadata;

    #[test]
    fn writes_special_characters_with_one_correct_xml_escape_layer() {
        let metadata = deserialize_crossbow_metadata(serde_json::json!({
            "build_variables": {
                "LABEL": { "env": "IGNORED_LABEL", "default": "R&D <Preview> ✓" }
            },
            "android": {
                "manifest": {
                    "package": "dev.crossbow.example",
                    "application": { "label": "{{crossbow.LABEL}}" }
                }
            }
        }))
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = save_android_manifest_with_variables(
            dir.path(),
            metadata.android.manifest.as_ref().unwrap(),
            &metadata.build_variables,
        )
        .unwrap();
        let xml = std::fs::read_to_string(path).unwrap();
        assert!(xml.contains("R&amp;D &lt;Preview&gt; ✓"), "{xml}");
        assert!(!xml.contains("&amp;amp;"), "{xml}");
        xml::EventReader::new(xml.as_bytes())
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
    }
}
