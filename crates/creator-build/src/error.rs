use displaydoc::Display;
use thiserror::Error;

pub type StdError = Box<dyn std::error::Error>;
pub type StdResult<T> = std::result::Result<T, StdError>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// No package metadata found for {0}
    NoPackageMetadata(String),
}
