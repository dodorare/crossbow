mod android;
mod apple;

pub use android::*;
pub use apple::*;

use crate::types::*;
use itertools::Itertools;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

pub(super) fn cargo_rustc_command(
    target: &Target,
    project_path: &PathBuf,
    release: &bool,
    cargo_args: &Vec<String>,
    build_target: &BuildTarget,
    crate_types: &Vec<CrateType>,
) -> ProcessCommand {
    let mut cargo = ProcessCommand::new("cargo");
    cargo.arg("rustc");
    match &target {
        Target::Bin(name) => cargo.args(&["--bin", &name]),
        Target::Example(name) => cargo.args(&["--example", &name]),
        Target::Lib => cargo.arg("--lib"),
    };
    cargo.current_dir(project_path.clone());
    if *release {
        cargo.arg("--release");
    }
    for arg in cargo_args.iter() {
        cargo.arg(arg);
    }
    let triple = build_target.rust_triple();
    cargo.args(&["--target", &triple]);
    if crate_types.len() > 0 {
        // Creates a comma-separated string
        let crate_types: String = crate_types
            .iter()
            .map(|v| v.rust_triple())
            .intersperse(",")
            .collect();
        cargo.args(&["--", "--crate-type", &crate_types]);
    }
    cargo
}
