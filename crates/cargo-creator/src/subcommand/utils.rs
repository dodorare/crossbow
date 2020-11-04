use super::manifest::Manifest;
use crate::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn list_rust_files(dir: &Path) -> Result<Vec<String>, Error> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let path = entry?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("rs")) {
                let name = path.file_stem().unwrap().to_str().unwrap();
                files.push(name.to_string());
            }
        }
    }
    Ok(files)
}

fn member(manifest: &Path, members: &[String], package: &str) -> Result<Option<PathBuf>, Error> {
    for member in members {
        if member.contains('*') {
            let member = member.replace("*", "").split('/').collect::<PathBuf>();
            let paths = std::fs::read_dir(manifest.parent().unwrap().join(member)).unwrap();
            for path in paths {
                let path = path.unwrap().path();
                // TODO: Refactor this and use recursion.
                let manifest_path = path.join("Cargo.toml");
                let manifest = Manifest::parse_from_toml(&manifest_path)?;
                if let Some(p) = manifest.package.as_ref() {
                    if p.name == package {
                        return Ok(Some(manifest_path));
                    }
                }
            }
            continue;
        }
        let member = member.split('/').collect::<PathBuf>();
        let manifest_path = manifest.parent().unwrap().join(member).join("Cargo.toml");
        let manifest = Manifest::parse_from_toml(&manifest_path)?;
        if let Some(p) = manifest.package.as_ref() {
            if p.name == package {
                return Ok(Some(manifest_path));
            }
        }
    }
    Ok(None)
}

pub fn find_package(path: &Path, name: Option<&str>) -> Result<(PathBuf, String, Option<String>), Error> {
    let path = std::fs::canonicalize(path)?;
    for manifest_path in path
        .ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .filter(|dir| dir.exists())
    {
        let manifest = Manifest::parse_from_toml(&manifest_path)?;
        let lib_name = manifest.lib.as_ref().and_then(|lib| lib.name.clone());
        if let Some(p) = manifest.package.as_ref() {
            if let (Some(n1), n2) = (name, &p.name) {
                if n1 == n2 {
                    return Ok((manifest_path, p.name.clone(), lib_name));
                }
            } else {
                return Ok((manifest_path, p.name.clone(), lib_name));
            }
        }
        if let (Some(w), Some(name)) = (manifest.workspace.as_ref(), name) {
            if let Some(manifest_path) = member(&manifest_path, &w.members, name)? {
                return Ok((manifest_path, name.to_string(), lib_name));
            }
        }
    }
    Err(Error::ManifestNotFound)
}

pub fn find_workspace(manifest: &Path, name: &str) -> Result<Option<PathBuf>, Error> {
    let dir = manifest.parent().unwrap();
    for manifest_path in dir
        .ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .filter(|dir| dir.exists())
    {
        let manifest = Manifest::parse_from_toml(&manifest_path)?;
        if let Some(w) = manifest.workspace.as_ref() {
            if let Ok(Some(_)) = member(&manifest_path, &w.members, name) {
                return Ok(Some(manifest_path));
            }
        }
    }
    Ok(None)
}
