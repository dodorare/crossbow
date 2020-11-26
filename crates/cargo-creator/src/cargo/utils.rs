use crate::error::*;
use std::path::PathBuf;

pub fn current_manifest_path() -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = dunce::canonicalize(current_dir.clone())?;
    let mut paths = current_dir
        .ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .filter(|dir| dir.exists());
    paths.next().ok_or(Error::ManifestNotFound)
}
