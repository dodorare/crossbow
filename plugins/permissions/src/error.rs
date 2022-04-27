use anyhow::Error;
use displaydoc::Display;
use thiserror::Error;

/// `Result` type that used in `crossbow-permissions`.
pub type Result<T> = std::result::Result<T, Error>;

/// `Crossbow-permissions` error type.
#[derive(Display, Debug, Error)]
pub enum CrossbowPermissionsError {
    /// Rust Jni library error
    #[cfg(target_os = "android")]
    Jni(jni::errors::Error),
    /// Anyhow library errors
    Anyhow(anyhow::Error),
}

#[cfg(target_os = "android")]
impl From<jni::errors::Error> for CrossbowPermissionsError {
    fn from(error: jni::errors::Error) -> Self {
        CrossbowPermissionsError::Jni(error.into()).into()
    }
}

impl From<anyhow::Error> for CrossbowPermissionsError {
    fn from(error: anyhow::Error) -> Self {
        CrossbowPermissionsError::Anyhow(error.into()).into()
    }
}
