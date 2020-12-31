use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Clap error
    ClapError(#[from] clap::Error),
    /// Cargo toml parse error
    CargoTomlError(#[from] cargo_toml::Error),
    /// Creator Tools error
    CreatorToolsError(#[from] creator_tools::error::Error),
}
