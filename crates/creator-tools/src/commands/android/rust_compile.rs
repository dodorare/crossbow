use crate::commands::common::cargo_rustc_command;
use crate::error::*;
use crate::tools::*;
use crate::types::*;
use std::path::Path;

/// Compiles rust code for android.
pub fn compile_rust_for_android(
    ndk: &AndroidNdk,
    target: Target,
    build_target: AndroidTarget,
    project_path: &Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    target_sdk_version: u32,
) -> Result<()> {
    let crate_types = vec![CrateType::Cdylib];
    let mut cargo = cargo_rustc_command(
        &target,
        project_path,
        &profile,
        &features,
        all_features,
        no_default_features,
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
    cargo.output_err(true)?;
    Ok(())
}

fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace("-", "_");
    let env = format!("CARGO_TARGET_{}_{}", &utarget, tool);
    env.to_uppercase()
}
