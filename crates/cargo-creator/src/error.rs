use crate::builder::android::error::NdkError;
use cargo_toml::Error as CargoTomlError;
use displaydoc::Display;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Invalid args
    InvalidArgs,
    /// Didn't find Cargo.toml
    ManifestNotFound,
    /// Didn't find rustc
    RustcNotFound,
    /// Failed to parse config: {0}
    Config(#[from] CargoTomlError),
    /// IO error: {0}
    Io(#[from] IoError),
    /// NDK error: {0}
    Ndk(#[from] NdkError),
}
