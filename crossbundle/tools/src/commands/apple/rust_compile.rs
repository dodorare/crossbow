use crate::{error::*, types::*};
use std::{path::Path, process::Command};

/// Compiles rust code for iOS.
///
/// Initialises `cargo rustc` [`Command`] with given args and return it.
///
/// [`Command`]: std::process::Command
pub fn compile_rust_for_ios(
    target: Target,
    build_target: IosTarget,
    project_path: &Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    crate_types: &[CrateType],
) -> Result<()> {
    let mut cargo = Command::new("cargo");
    cargo.arg("rustc");
    match &target {
        Target::Bin(name) => cargo.args(["--bin", name]),
        Target::Example(name) => cargo.args(["--example", name]),
        Target::Lib => cargo.arg("--lib"),
    };
    cargo.current_dir(project_path);
    if profile == Profile::Release {
        cargo.arg("--release");
    };
    for feature in features.iter() {
        cargo.args(["--feature", feature]);
    }
    if all_features {
        cargo.arg("--all-features");
    };
    if no_default_features {
        cargo.arg("--no-default-features");
    };
    let triple = build_target.rust_triple();
    cargo.args(["--target", triple]);
    if !crate_types.is_empty() {
        // Creates a comma-separated string
        let crate_types: String =
            itertools::Itertools::intersperse(crate_types.iter().map(|v| v.as_ref()), ",")
                .collect();
        cargo.args(["--", "--crate-type", &crate_types]);
    };
    cargo.output_err(true)?;
    Ok(())
}
