use super::Command;
use crate::error::StdResult;
use crate::target::{AndroidTarget, IntoRustTriple};

use std::path::PathBuf;
use std::process::Command as ProcessCommand;

pub struct AndroidRustCompile {
    targets: Vec<AndroidTarget>,
    target_dir: PathBuf,
    // all other arguments but without `target` and `target-dir`
    cargo_args: Vec<String>,
}

impl Command for AndroidRustCompile {
    type Deps = ();
    type Output = CompiledRustPackage;

    fn run(&self, (): Self::Deps) -> StdResult<Self::Output> {
        let mut cargo = ProcessCommand::new("cargo");
        cargo.arg("rustc");
        for arg in self.cargo_args.iter() {
            cargo.arg(arg);
        }
        for triple in self.targets.iter().map(|t| t.rust_triple()) {
            cargo.arg("--target").arg(triple);
        }
        cargo.arg("--");
        // cargo.arg("--crate-type");
        // cargo.arg("cdylib");

        // cargo.status()?.success()
        Ok(CompiledRustPackage {
            target_dir: self.target_dir.clone(),
        })
    }
}

pub struct CompiledRustPackage {
    pub target_dir: PathBuf,
}
