use displaydoc::Display;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code
    CmdFailed(Command),
    /// Android SDK is not found
    AndroidSdkNotFound,
    /// Android NDK is not found
    AndroidNdkNotFound,
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// Android SDK has no build tools
    BuildToolsNotFound,
    /// Android SDK has no platforms installed
    NoPlatformFound,
}
