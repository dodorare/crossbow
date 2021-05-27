use android_manifest::*;

use crate::error::*;
use crate::types::*;
use std::io::prelude::*;
use std::{fs::create_dir_all, io::BufWriter};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Saves given manifest in new `AndroidManifest.xml` file.
pub fn create_android_manifest(out_dir: &Path, manifest: AndroidManifest) -> Result<PathBuf> {
    if !out_dir.exists() {
        create_dir_all(out_dir)?;
    }
    let manifest_path = out_dir.join("AndroidManifest.xml");
    let mut file = File::create(&manifest_path)?;
    let given_xml = to_string(&manifest).unwrap();
    file.write_all(given_xml.as_bytes());
    Ok(manifest_path)
}
