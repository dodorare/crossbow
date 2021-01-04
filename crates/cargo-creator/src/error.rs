use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Code signing flags not provided
    CodeSigningFlagsNotProvided,
    /// Invalid manifest
    InvalidManifest,
    /// Invalid metadata in manifest
    InvalidManifestMetadata,
    /// Failed to find manifest: {0}
    FailedToFindManifest(String),
    /// IO error
    Io(#[from] std::io::Error),
    /// Clap error
    Clap(#[from] clap::Error),
    /// Cargo toml parse error
    CargoToml(#[from] cargo_toml::Error),
    /// Creator Tools error
    CreatorTools(#[from] creator_tools::error::Error),
}
