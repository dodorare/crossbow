use crate::error::*;
use cargo::{
    core::{EitherManifest, Manifest, SourceId},
    util::GlobalContext,
    util::toml::read_manifest,
};
use std::path::Path;

/// Read manifest files and deserialize it
pub fn parse_manifest(manifest_path: &Path) -> Result<Manifest> {
    let source_id = SourceId::for_path(manifest_path)?;
    let cargo_context = GlobalContext::default()?;
    let either_manifest = read_manifest(manifest_path, source_id, &cargo_context)
        .map_err(|_| Error::FailedToFindManifest(manifest_path.to_owned()))?;
    match either_manifest {
        EitherManifest::Real(manifest) => Ok(manifest),
        _ => Err(Error::FailedToFindManifest(manifest_path.to_owned())),
    }
}
