#[cfg(feature = "android")]
use crossbundle_tools::types::android_manifest;
use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Can't find target to run
    CantFindTargetToRun,
    /// Team identifier not provided
    TeamIdentifierNotProvided,
    /// Invalid metadata in manifest: {0:?}
    InvalidMetadata(anyhow::Error),
    /// IO error: {0:?}
    Io(#[from] std::io::Error),
    /// Clap error: {0:?}
    Clap(#[from] clap::Error),
    /// Anyhow error: {0:?}
    AnyhowError(#[from] anyhow::Error),
    /// Crossbundle Tools error: {0:?}
    CrossbundleTools(#[from] crossbundle_tools::error::Error),
    /// AndroidManifest error: {0:?}
    #[cfg(feature = "android")]
    AndroidManifest(#[from] android_manifest::error::Error),
    /// FsExtra error: {0:?}
    FsExtra(#[from] fs_extra::error::Error),
    /// Path {0:?} doesn't exist
    PathNotFound(std::path::PathBuf),
    /// Home dir not found
    HomeDirNotFound,
    /// Failed to download jar file: {0:?}
    DownloadFailed(Box<ureq::Error>),
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
#[cfg(feature = "android")]
impl From<crossbundle_tools::types::AndroidToolsError> for Error {
    fn from(error: crossbundle_tools::types::AndroidToolsError) -> Self {
        Error::CrossbundleTools(error.into())
    }
}
