//! Contains `Error`, `AndroidError`, `AppleError` types used by `crossbundle-tools`.

#[cfg(feature = "apple")]
use apple_bundle::plist;
use displaydoc::Display;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

/// `Result` type that used in `crossbundle-tools`.
pub type Result<T> = std::result::Result<T, Error>;

/// Android specific error type.
#[cfg(feature = "android")]
#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Android NDK is not found
    AndroidNdkNotFound,
    /// Failed to read source.properties
    FailedToReadSourceProperties,
    /// Invalid source.properties: {0}
    InvalidSourceProperties(String),
    /// Gradle Dependency project dir not found: {0}
    GradleDependencyProjectNotFound(PathBuf),
    /// Gradle Dependency project doesn't contain build.gradle: {0}
    GradleDependencyProjectNoBuildFile(PathBuf),
    /// Gradle is not found
    GradleNotFound,
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
    /// Unsupported or invalid target: {0}
    InvalidBuildTarget(String),
    /// Unsupported or invalid app wrapper: {0}
    InvalidAppWrapper(String),
    /// Unsupported or invalid build strategy: {0}
    InvalidBuildStrategy(String),
    /// Failed to find AndroidManifest.xml in path: {0}
    FailedToFindAndroidManifest(String),
    /// Unable to find NDK file
    UnableToFindNDKFile,
    /// AndroidTools error: {0:?}
    AndroidTools(#[from] android_tools::error::Error),
    /// AndroidManifest error: {0:?}
    AndroidManifest(#[from] android_manifest::error::Error),
}

/// Apple specific error type.
#[cfg(feature = "apple")]
#[derive(Display, Debug, Error)]
pub enum AppleError {
    /// Code signing profile not found
    CodeSigningProfilesNotFound,
    /// Code signing profile not provided
    CodeSigningProfileNotProvided,
    /// Codesign failed {0}
    CodesignFailed(String),
    /// Failed to archive payload
    ZipCommandFailed,
    /// Codesign allocate not found
    CodesignAllocateNotFound,
    /// Simctl error: {0:?}
    Simctl(simctl::Error),
    /// Target dir does not exists
    TargetNotFound,
    /// Resources dir does not exists
    ResourcesNotFound,
    /// Unsupported or invalid build strategy: {0}
    InvalidBuildStrategy(String),
    /// Unsupported or invalid target: {0}
    InvalidBuildTarget(String),
    /// Assets dir does not exists
    AssetsNotFound,
    /// Failed to find Info.plist in path: {0}
    FailedToFindInfoPlist(String),
    /// Plist data error: {0:?}
    Plist(#[from] plist::Error),
}

/// Main error type.
#[derive(Display, Debug, Error)]
#[ignore_extra_doc_attributes]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code. Stdout: {1} Stderr: {2}
    CmdFailed(Command, String, String),
    /// Command {0} not found
    CmdNotFound(String),
    /// Failed to copy file in specified path `{path}` cause of `{cause}`
    CopyToFileFailed {
        path: PathBuf,
        cause: std::io::Error,
    },
    /// Failed to find the manifest in path: {0}
    FailedToFindManifest(PathBuf),
    /// Invalid profile: {0}
    InvalidProfile(String),
    /// GNU toolchain binary `{gnu_bin}` nor LLVM toolchain binary `{llvm_bin}` found in
    /// `{toolchain_path:?}`
    ToolchainBinaryNotFound {
        toolchain_path: PathBuf,
        gnu_bin: String,
        llvm_bin: String,
    },
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// Failed to find cargo manifest: {0}
    FailedToFindCargoManifest(String),
    /// Failed to choose shell string color.
    /// Argument for --color must be auto, always, or never, but found `{}`
    FailedToChooseShellStringColor(String),
    /// IO error: {0:?}
    Io(#[from] std::io::Error),
    /// FS Extra error: {0:?}
    FsExtra(#[from] fs_extra::error::Error),
    /// Zip error: {0:?}
    Zip(#[from] zip::result::ZipError),
    /// Android error: {0:?}
    #[cfg(feature = "android")]
    Android(#[from] AndroidError),
    /// Apple error: {0:?}
    #[cfg(feature = "apple")]
    Apple(#[from] AppleError),
    /// Anyhow error: {0:?}
    AnyhowError(#[from] anyhow::Error),
    /// Other error: {0:?}
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

#[cfg(feature = "apple")]
impl From<plist::Error> for Error {
    fn from(error: plist::Error) -> Self {
        AppleError::from(error).into()
    }
}

#[cfg(feature = "apple")]
impl From<simctl::Error> for Error {
    fn from(error: simctl::Error) -> Self {
        AppleError::Simctl(error).into()
    }
}

#[cfg(feature = "android")]
impl From<android_tools::error::Error> for Error {
    fn from(error: android_tools::error::Error) -> Self {
        AndroidError::AndroidTools(error).into()
    }
}
