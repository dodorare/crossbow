use cargo_toml::Manifest;

use super::utils::*;
use crate::builder::android::metadata::AndroidMetadata;
use crate::error::*;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CreatorMetadata {
    pub android: AndroidMetadata,
}

pub type CargoManifest = Manifest<CreatorMetadata>;

pub fn current_manifest() -> Result<CargoManifest, Error> {
    let path = current_manifest_path()?;
    CargoManifest::from_path_with_metadata(path).map_err(|err| err.into())
}
