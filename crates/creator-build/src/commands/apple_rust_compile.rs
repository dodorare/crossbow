use super::Command;
use crate::error::{Error, StdResult};
use crate::target::{AppleTarget, IntoRustTriple};

use std::path::PathBuf;
use std::process::Command as ProcessCommand;

pub struct AppleRustCompile {
    targets: Vec<AppleTarget>,
    target_dir: PathBuf,
    // all other arguments but without `target` and `target-dir`
    cargo_args: Vec<String>,
}

impl Command for AppleRustCompile {
    type Deps = ();
    type Output = PathBuf;

    fn run(&self, (): Self::Deps) -> StdResult<Self::Output> {
        let mut cargo = ProcessCommand::new("cargo");
        cargo.arg("rustc");
        for arg in self.cargo_args.iter() {
            cargo.arg(arg);
        }
        for triple in self.targets.iter().map(|t| t.rust_triple()) {
            cargo.arg("--target").arg(triple);
        }
        cargo.args(&["--", "--crate-type", "staticlib"]);
        if !cargo.status()?.success() {
            return Err(Error::CmdFailed(cargo).into());
        }
        Ok(self.target_dir.clone())
    }
}
