use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Clap error
    ClapError(clap::Error),
    /// Creator Tools error
    CreatorToolsError(#[from] creator_tools::error::Error),
}

impl From<clap::Error> for Error {
    fn from(error: clap::Error) -> Self {
        Error::ClapError(error)
    }
}
