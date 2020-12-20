use super::cargo_rustc_command;
use crate::commands::Command;
use crate::deps::*;
use crate::error::*;
use crate::types::*;

use std::path::PathBuf;
// use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AndroidRustCompile {
    pub target: Target,
    pub build_target: BuildTarget,
    pub project_path: PathBuf,
    pub release: bool,
    pub cargo_args: Vec<String>,
    pub crate_types: Vec<CrateType>,
}

impl Command for AndroidRustCompile {
    type Deps = (AndroidSdk, AndroidNdk);
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
        // let (clang, clang_pp) = ndk.clang(target, sdk_version)?;
        // cargo.env(format!("CC_{}", triple), &clang);
        // cargo.env(format!("CXX_{}", triple), &clang_pp);
        // cargo.env(cargo_env_target_cfg("LINKER", triple), &clang);

        // let ar = ndk.toolchain_bin("ar", target)?;
        // cargo.env(format!("AR_{}", triple), &ar);
        // cargo.env(cargo_env_target_cfg("AR", triple), &ar);
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo).into());
        }
        Ok(())
    }
}
