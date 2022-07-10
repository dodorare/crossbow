use crossbundle_tools::types::android_manifest;
use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Build targets not provided
    BuildTargetsNotProvided,
    /// Can't find target to run
    CantFindTargetToRun,
    /// Team identifier not provided
    TeamIdentifierNotProvided,
    /// Invalid cargo metadata values
    InvalidCargoMetadata,
    /// Invalid metadata in manifest
    InvalidMetadata(anyhow::Error),
    /// IO error
    Io(#[from] std::io::Error),
    /// Clap error
    Clap(#[from] clap::Error),
    /// Anyhow error
    AnyhowError(#[from] anyhow::Error),
    /// Crossbundle Tools error
    CrossbundleTools(#[from] crossbundle_tools::error::Error),
    /// AndroidManifest error
    AndroidManifest(#[from] android_manifest::error::Error),
    /// FsExtra error
    FsExtra(#[from] fs_extra::error::Error),
    /// Path {0:?} doesn't exist
    PathNotFound(std::path::PathBuf),
    /// Home dir not found
    HomeDirNotFound,
    /// Failed to download jar file
    DownloadFailed(ureq::Error),
    /// Failed to create jar file in specified path `{path}` cause of `{cause}`
    JarFileCreationFailed {
        path: std::path::PathBuf,
        cause: std::io::Error,
    },
    /// Failed to copy file in specified path `{path}` cause of `{cause}`
    CopyToFileFailed {
        path: std::path::PathBuf,
        cause: std::io::Error,
    },
}

// TODO: Fix this. Is there a better casting for it?
impl From<crossbundle_tools::tools::AndroidToolsError> for Error {
    fn from(error: crossbundle_tools::tools::AndroidToolsError) -> Self {
        Error::CrossbundleTools(error.into())
    }
}
