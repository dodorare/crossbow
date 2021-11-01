use crate::commands::common::cargo_rustc_command;
use crate::error::*;
use crate::types::*;
use std::path::Path;

/// Compiles rust code for iOS.
pub fn compile_rust_for_ios(
    target: Target,
    build_target: AppleTarget,
    project_path: &Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
) -> Result<()> {
    let cargo = cargo_rustc_command(
        &target,
        project_path,
        &profile,
        &features,
        all_features,
        no_default_features,
        &build_target.into(),
        &[],
    );
    cargo.output_err(true)?;
    Ok(())
}
