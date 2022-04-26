use displaydoc::Display;
use std::process::Command;
use thiserror::Error;

/// `Result` type that used in `crossbow-permissions`.
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type.
#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code. Stdout: {1} Stderr: {2}
    CmdFailed(Command, String, String),
    /// Command {0} not found
    CmdNotFound(String),
    /// Failed to copy file in specified path `{path}` cause of `{cause}`
    CopyToFileFailed {
        path: std::path::PathBuf,
        cause: std::io::Error,
    },
    /// Failed to find the manifest in path: {0}
    FailedToFindManifest(std::path::PathBuf),
    /// Invalid profile: {0}
    InvalidProfile(String),
    /// Invalid interface orientation: {0:?}
    InvalidInterfaceOrientation(String),
    /// Home dir not found
    HomeDirNotFound,
    /// Failed to create jar file in specified path `{path}` cause of `{cause}`
    JarFileCreationFailed {
        path: std::path::PathBuf,
        cause: std::io::Error,
    },
    /// GNU toolchain binary `{gnu_bin}` nor LLVM toolchain binary `{llvm_bin}` found in
    /// `{toolchain_path:?}`
    ToolchainBinaryNotFound {
        toolchain_path: std::path::PathBuf,
        gnu_bin: String,
        llvm_bin: String,
    },
    /// Path {0:?} doesn't exist
    PathNotFound(std::path::PathBuf),
    /// Failed to find cargo manifest: {0}
    FailedToFindCargoManifest(String),
    /// Failed to choose shell string color.
    /// Argument for --color must be auto, always, or never, but found `{}`
    FailedToChooseShellStringColor(String),
}
