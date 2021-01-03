use crate::{Error, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn find_workspace_manifest_path(current_dir: &Path) -> Result<PathBuf> {
    find_manifest_path(current_dir, true)
}

pub fn find_package_manifest_path(current_dir: &Path) -> Result<PathBuf> {
    find_manifest_path(current_dir, false)
}

fn find_manifest_path(current_dir: &Path, workspace: bool) -> Result<PathBuf> {
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
