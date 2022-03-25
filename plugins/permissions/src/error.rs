use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Crossbundle Permissions error
    CrossbundlePermissions,
}
