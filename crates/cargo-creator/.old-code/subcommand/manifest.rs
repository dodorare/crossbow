use crate::error::Error;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub workspace: Option<Workspace>,
    pub package: Option<Package>,
    pub lib: Option<Lib>,
}

impl Manifest {
    pub fn parse_from_toml(path: &Path) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Lib {
    pub name: Option<String>,
}
