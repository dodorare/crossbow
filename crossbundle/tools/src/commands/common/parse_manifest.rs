use crate::error::Error;
use cargo::{
    core::{EitherManifest, Manifest, SourceId},
    util::toml::read_manifest,
    Config,
};
use std::path::Path;

/// Read manifest files and deserialize it
pub fn parse_manifest(manifest_path: &Path) -> crate::error::Result<Manifest> {
    let source_id = SourceId::for_path(manifest_path)?;
    let cargo_config = Config::default()?;
    let either_manifest = read_manifest(manifest_path, source_id, &cargo_config)
        .map_err(|_| Error::FailedToFindManifest(manifest_path.to_owned()))?
        .0;
    match either_manifest {
        EitherManifest::Real(manifest) => Ok(manifest),
        _ => Err(Error::FailedToFindManifest(manifest_path.to_owned())),
    }
}
