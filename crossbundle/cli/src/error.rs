use creator_tools::types::android_manifest;
use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Build targets not provided
    BuildTargetsNotProvided,
    /// Can't find target to run
    CantFindTargetToRun,
    /// Unsupported feature
    UnsupportedFeature,
    /// Team identifier not provided
    TeamIdentifierNotProvided,
    /// Invalid manifest
    InvalidManifest,
    /// Invalid metadata in manifest
    InvalidManifestMetadata,
    /// IO error
    Io(#[from] std::io::Error),
    /// Clap error
    Clap(#[from] clap::Error),
    /// Cargo toml parse error
    CargoToml(#[from] cargo_toml::Error),
    /// Creator Tools error
    CreatorTools(#[from] creator_tools::error::Error),
    /// AndroidManifest error
    AndroidManifest(#[from] android_manifest::error::Error),
}
