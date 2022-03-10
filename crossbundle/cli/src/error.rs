use std::path::PathBuf;

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
    /// Unsupported feature
    UnsupportedFeature,
    /// Team identifier not provided
    TeamIdentifierNotProvided,
    /// Invalid manifest
    InvalidManifest,
    /// Invalid metadata in manifest
    InvalidManifestMetadata,
    /// IO error
    Io(#[from] std::io::Error),
    /// Clap error
    Clap(#[from] clap::Error),
    /// Crossbundle Tools error
    CrossbundleTools(#[from] crossbundle_tools::error::Error),
    /// AndroidManifest error
    AndroidManifest(#[from] android_manifest::error::Error),
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// Home dir not found
    HomeDirNotFound,
    /// Failed to download jar file
    DownloadFailed(ureq::Error),
    /// Failed to create jar file in specified path `{path}` cause of `{cause}`
    JarFileCreationFailed {
        path: PathBuf,
        cause: std::io::Error,
    },
    /// Failed to copy file in specified path `{path}` cause of `{cause}`
    CopyToFileFailed {
        path: PathBuf,
        cause: std::io::Error,
    },
}

// TODO: Fix this. Is there a better casting for it?
impl From<crossbundle_tools::tools::AndroidToolsError> for Error {
    fn from(error: crossbundle_tools::tools::AndroidToolsError) -> Self {
        Error::CrossbundleTools(error.into()).into()
    }
}
