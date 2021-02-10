use crate::types::*;
use std::path::Path;
use std::process::Command;

/// Initialises `cargo rustc` [`Command`] with given args and return it.
///
/// [`Command`]: std::process::Command
pub fn cargo_rustc_command(
    target: &Target,
    project_path: &Path,
    profile: &Profile,
    features: &[String],
    all_features: bool,
    no_default_features: bool,
    build_target: &BuildTarget,
    crate_types: &[CrateType],
) -> Command {
    let mut cargo = Command::new("cargo");
    cargo.arg("rustc");
    match &target {
        Target::Bin(name) => cargo.args(&["--bin", name]),
        Target::Example(name) => cargo.args(&["--example", name]),
        Target::Lib => cargo.arg("--lib"),
    };
    cargo.current_dir(project_path);
    if profile == &Profile::Release {
        cargo.arg("--release");
    };
    for feature in features.iter() {
        cargo.args(&["--feature", feature]);
    }
    if all_features {
        cargo.arg("--all-features");
    };
    if no_default_features {
        cargo.arg("--no-default-features");
    };
    let triple = build_target.rust_triple();
    cargo.args(&["--target", &triple]);
    if !crate_types.is_empty() {
        // Creates a comma-separated string
        let crate_types: String =
            itertools::Itertools::intersperse(crate_types.iter().map(|v| v.as_ref()), ",")
                .collect();
        cargo.args(&["--", "--crate-type", &crate_types]);
    };
    cargo
}
