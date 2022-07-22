use displaydoc::Display;
use thiserror::Error;

/// Result type wrapper with AndroidError.
pub type Result<T> = std::result::Result<T, AndroidError>;

/// Permissions error type.
#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Signal Sender with provided singleton name not available
    SignalSenderNotAvailable(String),
    /// Singleton with provided name not found or haven't registered
    SingletonNotRegistered(String),
    /// Unsupported JNI Rust Type
    UnsupportedJniRustType(String),
    /// Wrong JNI Rust Type
    WrongJniRustType,
    /// Rust Jni library error
    Jni(#[from] jni::errors::Error),
    /// Anyhow library errors
    Anyhow(#[from] anyhow::Error),
}
