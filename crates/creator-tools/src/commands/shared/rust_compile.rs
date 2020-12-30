use crate::types::*;
use itertools::Itertools;
use std::path::Path;
use std::process::Command as ProcessCommand;

pub fn cargo_rustc_command(
    target: &Target,
    project_path: &Path,
    profile: &Profile,
    cargo_args: &[String],
    build_target: &BuildTarget,
    crate_types: &[CrateType],
) -> ProcessCommand {
    let mut cargo = ProcessCommand::new("cargo");
    cargo.arg("rustc");
    match &target {
        Target::Bin(name) => cargo.args(&["--bin", &name]),
        Target::Example(name) => cargo.args(&["--example", &name]),
        Target::Lib => cargo.arg("--lib"),
    };
    cargo.current_dir(project_path);
    if profile == &Profile::Release {
        cargo.arg("--release");
    };
    for arg in cargo_args.iter() {
        cargo.arg(arg);
    }
    let triple = build_target.rust_triple();
    cargo.args(&["--target", &triple]);
    if !crate_types.is_empty() {
        // Creates a comma-separated string
        let crate_types: String = crate_types
            .iter()
            .map(|v| v.as_ref())
            .intersperse(",")
            .collect();
        cargo.args(&["--", "--crate-type", &crate_types]);
    };
    cargo
}
