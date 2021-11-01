use crate::error::Result;
use android_manifest::AndroidManifest;
use std::fs::create_dir_all;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Saves given manifest in new `AndroidManifest.xml` file.
pub fn save_android_manifest(out_dir: &Path, manifest: &AndroidManifest) -> Result<PathBuf> {
    if !out_dir.exists() {
        create_dir_all(out_dir)?;
    }
    let manifest_path = out_dir.join("AndroidManifest.xml");
    let mut file = File::create(&manifest_path)?;
    let given_xml = android_manifest::to_string_pretty(manifest).unwrap();
    file.write_all(given_xml.as_bytes())?;
    Ok(manifest_path)
}
