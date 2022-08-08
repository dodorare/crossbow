use displaydoc::Display;
use thiserror::Error;

/// Result type wrapper with AndroidError.
pub type Result<T> = std::result::Result<T, AndroidError>;

/// Permissions error type.
#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Could not send to channel {0:?}
    CouldNotSendToSignalChannel(#[from] async_channel::TrySendError<crate::plugin::Signal>),
    /// Signal Sender with `{0}` singleton name not available
    SignalSenderNotAvailable(String),
    /// Singleton with `{0}` name not found or haven't registered
    SingletonNotRegistered(String),
    /// Unsupported JNI Rust Type: {0}
    UnsupportedJniRustType(String),
    /// Wrong JNI Rust Type
    WrongJniRustType,
    /// Rust Jni library error: {0:?}
    Jni(#[from] jni::errors::Error),
    /// Anyhow library errors: {0:?}
    Anyhow(#[from] anyhow::Error),
}
