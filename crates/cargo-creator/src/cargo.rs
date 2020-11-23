use cargo_toml::Manifest;
use std::path::PathBuf;

use crate::builder::android::metadata::Metadata as AndroidMetadata;
use crate::error::*;

pub type AndroidCargoManifest = Manifest<AndroidMetadata>;

pub fn parse_android_manifest(path: PathBuf) -> Result<AndroidCargoManifest, Error> {
    AndroidCargoManifest::from_path_with_metadata(path).map_err(|err| err.into())
}
