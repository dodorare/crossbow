use crate::commands::shared::cargo_rustc_command;
use crate::error::*;
use crate::types::*;
use std::path::Path;

pub fn apple_rust_compile(
    target_name: &str,
    build_target: AppleTarget,
    project_path: &Path,
    profile: Profile,
    cargo_args: Vec<String>,
) -> Result<()> {
    let cargo = cargo_rustc_command(
        &Target::Bin(target_name.to_owned()),
        project_path,
        &profile,
        &cargo_args,
        &build_target.into(),
        &[],
    );
    cargo.output_err(true)?;
    Ok(())
}
