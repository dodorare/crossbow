//! Contains `Error`, `AndroidError`, `AppleError` types used by `creator-tools`.

use apple_bundle::plist;
use displaydoc::Display;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

/// `Result` type that used in `creator-tools`.
pub type Result<T> = std::result::Result<T, Error>;

/// Android specific error type
#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Android SDK is not found
    AndroidSdkNotFound,
    /// Android NDK is not found
    AndroidNdkNotFound,
    /// Bundletool is not found
    BundletoolNotFound,
    /// Android SDK has no build tools
    BuildToolsNotFound,
    /// Android SDK has no platforms installed
    NoPlatformsFound,
    /// Failed to create directory
    DirectoryWasNotCreated,
    /// Platform {0} is not installed
    PlatformNotFound(u32),
    /// Target is not supported
    UnsupportedTarget,
    /// Host {0} is not supported
    UnsupportedHost(String),
    /// Invalid semver
    InvalidSemver,
    /// Unsupported or invalid target: {0}
    InvalidBuildTarget(String),
    /// Failed to find AndroidManifest.xml in path: {0}
    FailedToFindAndroidManifest(String),
    /// AndroidManifest error
    AndroidManifest(#[from] android_manifest::error::Error),
}

/// Apple specific error type.
#[derive(Display, Debug, Error)]
pub enum AppleError {
    /// Code signing profile not found
    CodeSigningProfilesNotFound,
    /// Code signing profile not provided
    CodeSigningProfileNotProvided,
    /// Codesign failed {0}
    CodesignFailed(String),
    /// Codesign allocate not found
    CodesignAllocateNotFound,
    /// Simctl error
    Simctl(simctl::Error),
    /// Target dir does not exists
    TargetNotFound,
    /// Resources dir does not exists
    ResourcesNotFound,
    /// Assets dir does not exists
    AssetsNotFound,
    /// Failed to find Info.plist in path: {0}
    FailedToFindInfoPlist(String),
    /// Plist data error
    Plist(#[from] plist::Error),
}

/// Main error type.
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
    /// Failed to find cargo manifest: {0}
    FailedToFindCargoManifest(String),
    /// Failed to choose shell string color.
    /// Argument for --color must be auto, always, or never, but found `{}`
    FailedToChooseShellStringColor(String),
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

/// Extension trait for [`Command`] that helps
/// to wrap output and print logs from command execution.
///
/// [`Command`]: std::process::Command
pub trait CommandExt {
    /// Executes the command as a child process, then captures an output and return it.
    /// If command termination wasn't successful wraps an output into error and return it.
    fn output_err(self, print_logs: bool) -> Result<std::process::Output>;
}

impl CommandExt for Command {
    fn output_err(mut self, print_logs: bool) -> Result<std::process::Output> {
        // Enables log print during command execution
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
