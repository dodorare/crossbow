use displaydoc::Display;
use thiserror::Error;

/// `Result` type that used in `crossbow-services`.
pub type Result<T> = std::result::Result<T, ServiceError>;

/// `Crossbow-services` error type.
#[derive(Display, Debug, Error)]
pub enum ServiceError {
    /// Google admob rust library errors
    Admob(google_admob1::Error),
    /// Anyhow library errors
    Anyhow(anyhow::Error),
    /// Authenticator errors
    Authenticator(std::io::Error),
}

impl From<google_admob1::Error> for ServiceError {
    fn from(error: google_admob1::Error) -> Self {
        ServiceError::Admob(error)
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(error: anyhow::Error) -> Self {
        ServiceError::Anyhow(error)
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> Self {
        ServiceError::Authenticator(error)
    }
}
