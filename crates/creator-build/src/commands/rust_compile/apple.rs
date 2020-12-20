use super::cargo_rustc_command;
use crate::commands::Command;
use crate::error::*;
use crate::types::*;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppleRustCompile {
    pub target: Target,
    pub build_target: AppleTarget,
    pub project_path: PathBuf,
    pub release: bool,
    pub cargo_args: Vec<String>,
    pub crate_types: Vec<CrateType>,
}

impl AppleRustCompile {
    pub fn new(
        target_name: String,
        build_target: AppleTarget,
        project_path: PathBuf,
        release: bool,
        cargo_args: Vec<String>,
    ) -> Self {
        Self {
            target: Target::Bin(target_name),
            build_target,
            project_path,
            release,
            cargo_args,
            crate_types: vec![],
        }
    }
}

impl Command for AppleRustCompile {
    type Deps = ();
    type Output = ();

    fn run(&self) -> Result<Self::Output> {
        let mut cargo = cargo_rustc_command(
            &self.target,
            &self.project_path,
            &self.release,
            &self.cargo_args,
            &self.build_target.into(),
            &self.crate_types,
        );
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo).into());
        }
        Ok(())
    }
}
