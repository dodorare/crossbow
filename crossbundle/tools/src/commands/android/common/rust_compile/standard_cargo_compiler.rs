use super::cmake_env;
use crate::{
    commands::{CargoBuild, CargoPackage},
    error::*,
    types::*,
};
use std::path::{Path, PathBuf};

/// Build an Android shared library through Cargo's public command-line interface.
#[allow(clippy::too_many_arguments)]
pub fn standard_cargo_compile(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    package: &CargoPackage,
    library_target_name: &str,
    profile: Profile,
    features: &[String],
    all_features: bool,
    no_default_features: bool,
    min_sdk_version: u32,
    target_dir: &Path,
) -> Result<PathBuf> {
    let triple = build_target.rust_triple();
    let (clang, clang_pp) = ndk.clang(build_target, min_sdk_version)?;
    let ar = ndk.toolchain_bin("ar", build_target)?;
    let clang_target = format!(
        "--target={}{}",
        build_target.ndk_llvm_triple(),
        min_sdk_version
    );
    let build_dir = target_dir.join(triple).join(profile);
    std::fs::create_dir_all(&build_dir)?;
    let cmake = cmake_env(build_target, ndk, min_sdk_version, &build_dir)?;
    let target = CargoTargetSelection::Lib(library_target_name.to_owned());
    let artifact = CargoBuild {
        package,
        target: &target,
        target_triple: triple,
        target_dir,
        profile,
        features,
        all_features,
        no_default_features,
    }
    .run(|cargo| {
        cargo
            .env(format!("CC_{triple}"), &clang)
            .env(format!("CFLAGS_{triple}"), &clang_target)
            .env(format!("CXX_{triple}"), &clang_pp)
            .env(format!("CXXFLAGS_{triple}"), &clang_target)
            .env(format!("AR_{triple}"), &ar)
            .env(cargo_env_target_cfg("LINKER", triple), &clang)
            .env(cargo_env_target_cfg("AR", triple), &ar)
            .env("CXXSTDLIB", "c++")
            .envs(cmake);
    })?;

    if !artifact
        .crate_types
        .iter()
        .any(|crate_type| crate_type == "cdylib")
    {
        return Err(anyhow::anyhow!(
            "Cargo library target `{library_target_name}` did not produce a cdylib. Add `crate-type = [\"cdylib\", \"rlib\"]` under `[lib]` in Cargo.toml."
        )
        .into());
    }
    let path = artifact
        .filenames
        .into_iter()
        .find(|path| path.extension().is_some_and(|extension| extension == "so"))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Cargo did not report an Android shared library for target `{library_target_name}`"
            )
        })?;
    if path.is_file() {
        Ok(path)
    } else {
        Err(Error::PathNotFound(path))
    }
}

fn cargo_env_target_cfg(key: &str, target: &str) -> String {
    format!(
        "CARGO_TARGET_{}_{}",
        target.to_uppercase().replace('-', "_"),
        key
    )
}
