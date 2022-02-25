use anyhow::Error;
use cargo::{
    core::{EitherManifest, Manifest, SourceId},
    util::toml::read_manifest,
    Config,
};
use std::path::Path;

pub fn parse_manifest(manifest_path: &Path) -> crate::error::Result<Manifest> {
    let source_id = SourceId::for_path(&manifest_path)
        .map_err(|_| Error::msg("Failed to create source_id from filesystem path"))?;
    let cargo_config =
        Config::default().map_err(|_| Error::msg("Failed to create a new config instance"))?;
    let either_manifest = read_manifest(&manifest_path, source_id, &cargo_config)
        .map_err(|_| Error::msg("Failed to read. Check the path to the manifest"))?
        .0;
    let manifest = match either_manifest {
        EitherManifest::Real(manifest) => manifest,
        _ => {
            let description = String::from("Received a virtual Cargo.toml data.");
            return Err(crate::error::Error::FailedToFindManifest(description));
        }
    };
    Ok(manifest)
}
