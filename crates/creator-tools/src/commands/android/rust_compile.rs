use crate::commands::shared::cargo_rustc_command;
use crate::deps::*;
use crate::error::*;
use crate::types::*;

use std::path::{Path, PathBuf};

/// Compile rust lib for android.
pub fn compile_rust_for_android(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    project_path: &Path,
    profile: Profile,
    cargo_args: Vec<String>,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let target = Target::Lib;
    let crate_types = vec![CrateType::Cdylib];
    let mut cargo = cargo_rustc_command(
        &target,
        project_path,
        &profile,
        &cargo_args,
        &build_target.into(),
        &crate_types,
    );
    let triple = build_target.rust_triple();
    // Takes clang and clang_pp paths
    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;
    cargo.env(format!("CC_{}", triple), &clang);
    cargo.env(format!("CXX_{}", triple), &clang_pp);
    cargo.env(cargo_env_target_cfg("LINKER", triple), &clang);
    let ar = ndk.toolchain_bin("ar", build_target)?;
    cargo.env(format!("AR_{}", triple), &ar);
    cargo.env(cargo_env_target_cfg("AR", triple), &ar);
    cargo.output_err()?;
    let out_dir = project_path
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    Ok(out_dir)
}

fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace("-", "_");
    let env = format!("CARGO_TARGET_{}_{}", &utarget, tool);
    env.to_uppercase()
}
