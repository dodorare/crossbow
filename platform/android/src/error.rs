use displaydoc::Display;
use thiserror::Error;

/// `Result` type that used in `crossbow-permissions`.
pub type Result<T> = std::result::Result<T, AndroidError>;

/// Permissions error type.
#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Unsupported JNI Rust Type
    UnsupportedJniRustType(String),
    /// Wrong JNI Rust Type
    WrongJniRustType,
    /// Rust Jni library error
    Jni(jni::errors::Error),
    /// Anyhow library errors
    Anyhow(anyhow::Error),
}

impl From<jni::errors::Error> for AndroidError {
    fn from(error: jni::errors::Error) -> Self {
        Self::Jni(error)
    }
}

impl From<anyhow::Error> for AndroidError {
    fn from(error: anyhow::Error) -> Self {
        Self::Anyhow(error)
    }
}
