use crate::error::{AndroidError, Result};
use android_manifest::AndroidManifest;
use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

/// Saves a manifest as `AndroidManifest.xml`.
pub fn save_android_manifest(out_dir: &Path, manifest: &AndroidManifest) -> Result<PathBuf> {
    create_dir_all(out_dir)?;
    let manifest_path = out_dir.join("AndroidManifest.xml");
    let xml = android_manifest::to_string_pretty(manifest).map_err(AndroidError::from)?;
    File::create(&manifest_path)?.write_all(&normalize_xml(&xml)?)?;
    Ok(manifest_path)
}

// `android-manifest` serializes wrapped strings through a nested XML writer. Decode that writer's
// entity layer, then let the final writer apply the one layer required by the document.
fn normalize_xml(xml: &str) -> Result<Vec<u8>> {
    use xml::reader::XmlEvent;

    let mut output = Vec::new();
    let mut writer = xml::EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut output);
    for event in xml::EventReader::new(xml.as_bytes()) {
        let mut event = event.map_err(|error| anyhow::anyhow!("invalid generated XML: {error}"))?;
        match &mut event {
            XmlEvent::StartElement { attributes, .. } => {
                for attribute in attributes {
                    attribute.value = decode_nested_entities(&attribute.value);
                }
            }
            XmlEvent::Characters(value) | XmlEvent::CData(value) => {
                *value = decode_nested_entities(value);
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

fn decode_nested_entities(value: &str) -> String {
    [
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&#xA;", "\n"),
        ("&#xD;", "\r"),
        ("&amp;", "&"),
    ]
    .into_iter()
    .fold(value.to_owned(), |value, (entity, character)| {
        value.replace(entity, character)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::deserialize_crossbow_metadata;

    #[test]
    fn writes_special_characters_with_one_escape_layer() {
        let metadata = deserialize_crossbow_metadata(serde_json::json!({
            "build_variables": {
                "LABEL": { "env": "IGNORED_LABEL", "default": "R&D <Preview> ✓" }
            },
            "android": { "manifest": {
                "package": "dev.crossbow.example",
                "application": { "label": "{{crossbow.LABEL}}" }
            }}
        }))
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path =
            save_android_manifest(dir.path(), metadata.android.manifest.as_ref().unwrap()).unwrap();
        let xml = std::fs::read_to_string(path).unwrap();
        assert!(xml.contains("R&amp;D &lt;Preview&gt; ✓"), "{xml}");
        assert!(!xml.contains("&amp;amp;"), "{xml}");
        xml::EventReader::new(xml.as_bytes())
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
    }
}
