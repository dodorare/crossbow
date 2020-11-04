use crate::ndk::error::NdkError;
use displaydoc::Display;
use std::io::Error as IoError;
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Invalid args
    InvalidArgs,
    /// Didn't find Cargo.toml
    ManifestNotFound,
    /// Didn't find rustc
    RustcNotFound,
    /// Failed to parse config
    Config(#[from] TomlError),
    /// NDK error
    Ndk(#[from] NdkError),
    /// IO error
    Io(#[from] IoError),
}
