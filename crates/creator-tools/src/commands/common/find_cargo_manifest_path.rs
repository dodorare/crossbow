use crate::error::{Error, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// Finds workspace/root `Cargo.toml` in given path or parent's paths.
pub fn find_workspace_cargo_manifest_path(current_dir: &Path) -> Result<PathBuf> {
    find_cargo_manifest_path(current_dir, true)
}

/// Finds a `Cargo.toml` in given path.
pub fn find_package_cargo_manifest_path(current_dir: &Path) -> Result<PathBuf> {
    find_cargo_manifest_path(current_dir, false)
}

/// Finds a `Cargo.toml` in given path,
/// if `workspace` is set tries to find workspace/root manifest.
fn find_cargo_manifest_path(current_dir: &Path, workspace: bool) -> Result<PathBuf> {
    let mut cargo = Command::new("cargo");
    cargo.current_dir(current_dir);
    cargo.args(&["locate-project", "--message-format", "plain"]);
    if workspace {
        cargo.arg("--workspace");
    }
    let output = cargo.output()?;
    if !output.status.success() {
        return Err(Error::FailedToFindManifest(
            String::from_utf8(output.stderr)
                .unwrap()
                .replace("error: ", "")
                .replace("\n", ""),
        ));
    }
    let workspace_path = String::from_utf8(output.stdout).unwrap();
    Ok(PathBuf::from(workspace_path.replace("\n", "")))
}
