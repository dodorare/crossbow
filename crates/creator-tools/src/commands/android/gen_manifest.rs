use crate::error::*;
use crate::types::*;
use std::fs::create_dir_all;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn gen_android_manifest(out_dir: &Path, manifest: &AndroidManifest) -> Result<PathBuf> {
    if !out_dir.exists() {
        create_dir_all(out_dir)?;
    }
    let manifest_path = out_dir.join("AndroidManifest.xml");
    let mut file = File::create(&manifest_path)?;
    writeln!(file, "{}", manifest.to_string())?;
    Ok(manifest_path)
}
