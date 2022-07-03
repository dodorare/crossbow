use displaydoc::Display;
use thiserror::Error;

/// `Result` type that used in `crossbow-permissions`.
pub type Result<T> = std::result::Result<T, PermissionError>;

/// Permissions error type.
#[derive(Display, Debug, Error)]
pub enum PermissionError {
    /// Rust Jni library error
    Jni(jni::errors::Error),
    /// Anyhow library errors
    Anyhow(anyhow::Error),
}

impl From<jni::errors::Error> for PermissionError {
    fn from(error: jni::errors::Error) -> Self {
        PermissionError::Jni(error)
    }
}

impl From<anyhow::Error> for PermissionError {
    fn from(error: anyhow::Error) -> Self {
        PermissionError::Anyhow(error)
    }
}
