use displaydoc::Display;
use thiserror::Error;

/// `Result` type that used in `crossbow-permissions`.
pub type Result<T> = std::result::Result<T, PermissionError>;

/// `Crossbow-permissions` error type.
#[derive(Display, Debug, Error)]
pub enum PermissionError {
    /// Requesting permission on the wrong platform
    PermissionWrongPlatform,
    /// Rust Jni library error
    #[cfg(target_os = "android")]
    Jni(jni::errors::Error),
    /// Anyhow library errors
    Anyhow(anyhow::Error),
}

#[cfg(target_os = "android")]
impl From<jni::errors::Error> for PermissionError {
    fn from(error: jni::errors::Error) -> Self {
        PermissionError::Jni(error.into())
    }
}

impl From<anyhow::Error> for PermissionError {
    fn from(error: anyhow::Error) -> Self {
        PermissionError::Anyhow(error)
    }
}
