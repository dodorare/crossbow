use crate::ndk::error::NdkError;
use displaydoc::Display;
use std::io::Error as IoError;
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// invalid args
    InvalidArgs,
    /// didn't find Cargo.toml
    ManifestNotFound,
    /// didn't find rustc
    RustcNotFound,
    /// failed to parse config
    Config(#[from] TomlError),
    /// ndk error
    Ndk(#[from] NdkError),
    /// io error
    Io(#[from] IoError),
}
