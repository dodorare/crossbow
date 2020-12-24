use super::cargo_rustc_command;
use crate::commands::Command;
use crate::deps::*;
use crate::error::*;
use crate::types::*;

use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AndroidRustCompile {
    pub ndk: Rc<AndroidNdk>,
    pub target: Target,
    pub build_target: AndroidTarget,
    pub project_path: PathBuf,
    pub profile: Profile,
    pub cargo_args: Vec<String>,
    pub crate_types: Vec<CrateType>,
    pub target_sdk_version: u32,
}

impl AndroidRustCompile {
    pub fn new(
        ndk: Rc<AndroidNdk>,
        build_target: AndroidTarget,
        project_path: PathBuf,
        profile: Profile,
        cargo_args: Vec<String>,
        target_sdk_version: u32,
    ) -> Self {
        Self {
            ndk,
            target: Target::Lib,
            build_target,
            project_path,
            profile,
            cargo_args,
            crate_types: vec![CrateType::Cdylib],
            target_sdk_version,
        }
    }
}

impl Command for AndroidRustCompile {
    type Deps = AndroidNdk;
    type Output = PathBuf;

    fn run(&self) -> Result<Self::Output> {
        let mut cargo = cargo_rustc_command(
            &self.target,
            &self.project_path,
            &self.profile,
            &self.cargo_args,
            &self.build_target.into(),
            &self.crate_types,
        );
        let triple = self.build_target.rust_triple();
        // Takes clang and clang_pp paths
        let (clang, clang_pp) = self.ndk.clang(self.build_target, self.target_sdk_version)?;
        cargo.env(format!("CC_{}", triple), &clang);
        cargo.env(format!("CXX_{}", triple), &clang_pp);
        cargo.env(cargo_env_target_cfg("LINKER", triple), &clang);
        let ar = self.ndk.toolchain_bin("ar", self.build_target)?;
        cargo.env(format!("AR_{}", triple), &ar);
        cargo.env(cargo_env_target_cfg("AR", triple), &ar);
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo));
        }
        let out_dir = self
            .project_path
            .join("target")
            .join(self.build_target.rust_triple())
            .join(self.profile.as_ref());
        Ok(out_dir)
    }
}

fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace("-", "_");
    let env = format!("CARGO_TARGET_{}_{}", &utarget, tool);
    env.to_uppercase()
}
