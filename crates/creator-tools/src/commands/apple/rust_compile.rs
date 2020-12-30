use crate::commands::shared::cargo_rustc_command;
use crate::error::*;
use crate::types::*;

use std::path::{Path, PathBuf};

pub fn apple_rust_compile(
    target_name: &str,
    build_target: AppleTarget,
    project_path: &Path,
    profile: Profile,
    cargo_args: Vec<String>,
) -> Result<PathBuf> {
    let cargo = cargo_rustc_command(
        &Target::Bin(target_name.to_owned()),
        project_path,
        &profile,
        &cargo_args,
        &build_target.into(),
        &[],
    );
    cargo.output_err()?;
    let out_dir = project_path
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    Ok(out_dir)
}
