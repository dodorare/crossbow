use cargo_toml::Manifest;

use crate::builder::android::metadata::AndroidMetadata;
use crate::error::*;

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CreatorMetadata {
    pub android: AndroidMetadata,
}

pub type CargoManifest = Manifest<CreatorMetadata>;

pub fn load_cargo_manifest(path: Option<PathBuf>) -> Result<CargoManifest, Error> {
    if let Some(path) = path {
        CargoManifest::from_path_with_metadata(path).map_err(|err| err.into())
    } else {
        let path = current_cargo_manifest_path()?;
        CargoManifest::from_path_with_metadata(path).map_err(|err| err.into())
    }
}

pub fn current_cargo_manifest_path() -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir()?;
    let current_dir = dunce::canonicalize(current_dir.clone())?;
    let mut paths = current_dir
        .ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .filter(|dir| dir.exists());
    paths.next().ok_or(Error::ManifestNotFound)
}
