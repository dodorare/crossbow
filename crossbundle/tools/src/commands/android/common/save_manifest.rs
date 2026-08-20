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
    File::create(&manifest_path)?.write_all(xml.as_bytes())?;
    Ok(manifest_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::parse_project_config;

    #[test]
    fn writes_special_characters_with_one_escape_layer() {
        let metadata = parse_project_config(serde_json::json!({
            "build_variables": {
                "LABEL": { "env": "IGNORED_LABEL", "default": "R&D <Preview> &amp; \"quoted\" ✓" }
            },
            "android": { "manifest": {
                "package": "dev.crossbow.example",
                "application": { "label": "{{crossbow.LABEL}}" }
            }}
        }))
        .unwrap()
        .resolve_with(|_| Ok(None))
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path =
            save_android_manifest(dir.path(), metadata.android.manifest.as_ref().unwrap()).unwrap();
        let xml = std::fs::read_to_string(path).unwrap();
        assert!(
            xml.contains("R&amp;D &lt;Preview&gt; &amp;amp; &quot;quoted&quot; ✓"),
            "{xml}"
        );
        let events = xml::EventReader::new(xml.as_bytes())
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
        let label = events.into_iter().find_map(|event| match event {
            xml::reader::XmlEvent::StartElement {
                name, attributes, ..
            } if name.local_name == "application" => attributes
                .into_iter()
                .find(|attribute| attribute.name.local_name == "label")
                .map(|attribute| attribute.value),
            _ => None,
        });
        assert_eq!(label.as_deref(), Some("R&D <Preview> &amp; \"quoted\" ✓"));
    }
}
