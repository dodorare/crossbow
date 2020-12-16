use super::{BinOrLib, Command};
use crate::error::{Error, StdResult};
use crate::target::*;

use itertools::Itertools;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

#[derive(Debug, Clone)]
pub enum AndroidOrAppleTargets {
    Android(Vec<AndroidTarget>),
    Apple(Vec<AppleTarget>),
}

#[derive(Debug, Clone)]
pub struct RustCompile {
    pub target: BinOrLib,
    pub targets: AndroidOrAppleTargets,
    pub project_path: PathBuf,
    pub release: bool,
    pub cargo_args: Vec<String>,
    pub crate_types: Vec<CrateType>,
}

impl Command for RustCompile {
    type Deps = ();
    type Output = ();

    fn run(&self, (): Self::Deps) -> StdResult<Self::Output> {
        let mut cargo = ProcessCommand::new("cargo");
        cargo.arg("rustc");
        cargo.current_dir(self.project_path.clone());
        match &self.target {
            BinOrLib::Bin(name) => cargo.args(&["--bin", &name]),
            BinOrLib::Lib => cargo.arg("--lib"),
        };
        if self.release {
            cargo.arg("--release");
        }
        for arg in self.cargo_args.iter() {
            cargo.arg(arg);
        }
        // match self.targets {}
        // for triple in self.targets.iter().map(|t| t.rust_triple()) {
        //     cargo.arg("--target").arg(triple);
        // }
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
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo).into());
        }
        Ok(())
    }
}
