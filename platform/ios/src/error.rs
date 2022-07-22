use displaydoc::Display;
use thiserror::Error;

/// Result type wrapper with IosError.
pub type Result<T> = std::result::Result<T, IosError>;

/// Permissions error type.
#[derive(Display, Debug, Error)]
pub enum IosError {
    /// Anyhow library errors: {0:?}
    Anyhow(#[from] anyhow::Error),
}
