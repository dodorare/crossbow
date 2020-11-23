use cargo_toml::Error as CargoTomlError;
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
    /// Failed to parse config: {0}
    Config(#[from] TomlError),
    /// IO error: {0}
    Io(#[from] IoError),
}

impl From<CargoTomlError> for Error {
    fn from(error: CargoTomlError) -> Error {
        match error {
            CargoTomlError::Io(io) => io.into(),
            CargoTomlError::Parse(toml) => toml.into(),
        }
    }
}
