use displaydoc::Display;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Android SDK is not found
    AndroidSdkNotFound,
    /// Android NDK is not found
    AndroidNdkNotFound,
    /// Android SDK has no build tools
    BuildToolsNotFound,
    /// Android SDK has no platforms installed
    NoPlatformsFound,
    /// Platform {0} is not installed
    PlatformNotFound(u32),
    /// Target is not supported
    UnsupportedTarget,
    /// Host {0} is not supported
    UnsupportedHost(String),
    /// Invalid semver
    InvalidSemver,
}

#[derive(Display, Debug, Error)]
pub enum AppleError {
    /// Plist data error
    Plist(#[from] plist::Error),
    /// Simctl error
    Simctl(simctl::Error),
    /// Target dir does not exists
    TargetNotFound,
    /// Resources dir does not exists
    ResourcesNotFound,
    /// Assets dir does not exists
    AssetsNotFound,
}

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code. Stdout: {1} Stderr: {2}
    CmdFailed(Command, String, String),
    /// Command {0} not found
    CmdNotFound(String),
    /// Invalid profile: {0}
    InvalidProfile(String),
    /// Invalid interface orientation: {0:?}
    InvalidInterfaceOrientation(String),
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// IO error
    Io(#[from] std::io::Error),
    /// FS Extra error
    FsExtra(#[from] fs_extra::error::Error),
    /// Android error
    Android(#[from] AndroidError),
    /// Apple error
    Apple(#[from] AppleError),
    /// Other error
    OtherError(#[from] Box<dyn std::error::Error>),
}

pub trait CommandExt {
    fn output_err(self, print_logs: bool) -> Result<std::process::Output>;
}

impl CommandExt for std::process::Command {
    fn output_err(mut self, print_logs: bool) -> Result<std::process::Output> {
        let output = match print_logs {
            true => self.spawn().and_then(|p| p.wait_with_output())?,
            false => self.output()?,
        };
        if !output.status.success() {
            return Err(Error::CmdFailed(
                self,
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(output)
    }
}

impl From<plist::Error> for Error {
    fn from(error: plist::Error) -> Self {
        AppleError::from(error).into()
    }
}

impl From<simctl::Error> for Error {
    fn from(error: simctl::Error) -> Self {
        AppleError::Simctl(error).into()
    }
}
