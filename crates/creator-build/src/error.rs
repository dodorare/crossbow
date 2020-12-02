use displaydoc::Display;
use thiserror::Error;

pub type StdError = Box<dyn std::error::Error>;
pub type StdResult<T> = std::result::Result<T, StdError>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Android SDK is not found.
    AndroidSdkNotFound,
}
