use ndk_build::error::NdkError;
use std::io::Error as IoError;
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid args.")]
    InvalidArgs,
    #[error("Didn't find Cargo.toml")]
    ManifestNotFound,
    #[error("Didn't find rustc.")]
    RustcNotFound,
    #[error("Failed to parse config.")]
    Config(#[from] TomlError),
    #[error(transparent)]
    Ndk(#[from] NdkError),
    #[error(transparent)]
    Io(#[from] IoError),
}
