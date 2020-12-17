use super::*;
use crate::deps::*;
use crate::error::*;
use crate::target::*;

use itertools::Itertools;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RustCompile {
    pub target: BinOrLib,
    pub build_target: AndroidOrAppleTarget,
    pub project_path: PathBuf,
    pub release: bool,
    pub cargo_args: Vec<String>,
    pub crate_types: Vec<CrateType>,
}

impl Command for RustCompile {
    type Deps = ();
    type OptDeps = (Option<Arc<AndroidNdk>>,);
    type Output = ();

    fn run(&self, (): Self::Deps, (android_ndk,): Self::OptDeps) -> StdResult<Self::Output> {
        let mut cargo = ProcessCommand::new("cargo");
        cargo.arg("rustc");
        match &self.target {
            BinOrLib::Bin(name) => cargo.args(&["--bin", &name]),
            BinOrLib::Lib => cargo.arg("--lib"),
        };
        cargo.current_dir(self.project_path.clone());
        if self.release {
            cargo.arg("--release");
        }
        for arg in self.cargo_args.iter() {
            cargo.arg(arg);
        }
        let triple = self.build_target.rust_triple();
        cargo.args(&["--target", &triple]);
        if self.crate_types.len() > 0 {
            // Creates a comma-separated string
            let crate_types: String = self
                .crate_types
                .iter()
                .map(|v| v.rust_triple())
                .intersperse(",")
                .collect();
            cargo.args(&["--", "--crate-type", &crate_types]);
        }
        if let Some(_android_ndk) = android_ndk {
            // let (clang, clang_pp) = ndk.clang(target, sdk_version)?;
            // cargo.env(format!("CC_{}", triple), &clang);
            // cargo.env(format!("CXX_{}", triple), &clang_pp);
            // cargo.env(cargo_env_target_cfg("LINKER", triple), &clang);
    
            // let ar = ndk.toolchain_bin("ar", target)?;
            // cargo.env(format!("AR_{}", triple), &ar);
            // cargo.env(cargo_env_target_cfg("AR", triple), &ar);
        }
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo).into());
        }
        Ok(())
    }
}