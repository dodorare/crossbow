use displaydoc::Display;
use std::process::Command;
use thiserror::Error;

pub type StdError = Box<dyn std::error::Error>;
pub type StdResult<T> = std::result::Result<T, StdError>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code
    CmdFailed(Command),
    /// Android SDK is not found.
    AndroidSdkNotFound,
}
