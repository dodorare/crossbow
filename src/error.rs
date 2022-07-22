use displaydoc::Display;
use thiserror::Error;

/// Result type wrapper with CrossbowError.
pub type Result<T> = std::result::Result<T, CrossbowError>;

/// Permissions error type.
#[derive(Display, Debug, Error)]
pub enum CrossbowError {
    /// Ios errors
    #[cfg(all(target_os = "android", feature = "android"))]
    AndroidError(#[from] crate::android::error::AndroidError),
    /// Ios errors
    #[cfg(all(target_os = "ios", feature = "ios"))]
    IosError(#[from] crate::ios::error::IosError),
    /// Anyhow library errors
    Anyhow(#[from] anyhow::Error),
}
