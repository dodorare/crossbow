pub mod aab;
pub mod apk;

pub use aab::*;
pub use apk::*;

pub(crate) fn library_name(path: &std::path::Path) -> crate::error::Result<&str> {
    path.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| crate::error::Error::PathNotFound(path.to_owned()))
}
