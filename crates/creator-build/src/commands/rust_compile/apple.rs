use super::cargo_rustc_command;
use crate::commands::Command;
use crate::error::*;
use crate::types::*;

use std::path::PathBuf;
// use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AppleRustCompile {
    pub target: Target,
    pub build_target: BuildTarget,
    pub project_path: PathBuf,
    pub release: bool,
    pub cargo_args: Vec<String>,
    pub crate_types: Vec<CrateType>,
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
            &self.build_target,
            &self.crate_types,
        );
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo).into());
        }
        Ok(())
    }
}
