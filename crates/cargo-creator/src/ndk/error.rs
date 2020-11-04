use std::io::Error as IoError;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;
use displaydoc::Display;

#[derive(Display, Debug, Error)]
pub enum NdkError {
    /// Android SDK is not found.
    /// Please set the path to the Android SDK with the $ANDROID_SDK_ROOT
    /// environment variable
    SdkNotFound,
    /// Android NDK is not found.
    /// Please set the path to the Android NDK with $ANDROID_NDK_ROOT
    /// environment variable
    NdkNotFound,
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// Command {0} not found
    CmdNotFound(String),
    /// Android SDK has no build tools
    BuildToolsNotFound,
    /// Android SDK has no platforms installed
    NoPlatformFound,
    /// Platform {0} is not installed
    PlatformNotFound(u32),
    /// Target is not supported
    UnsupportedTarget,
    /// Host {0} is not supported
    UnsupportedHost(String),
    /// IO error
    Io(#[from] IoError),
    /// Invalid semver
    InvalidSemver,
    /// Command '{0:?}' had a non-zero exit code
    CmdFailed(Command),
}
