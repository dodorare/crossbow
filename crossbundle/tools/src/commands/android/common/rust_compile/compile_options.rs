use crate::error::*;
use crate::types::*;
use cargo::{
    core::{
        Workspace,
        compiler::{CompileKind, CompileTarget, UserIntent},
        resolver::CliFeatures,
    },
    ops::{CompileFilter, CompileOptions},
};

use std::path::Path;

/// Configure compilation options so that we will build the desired build_target
pub fn compile_options(
    workspace: &Workspace,
    build_target: AndroidTarget,
    features: &[String],
    all_features: bool,
    no_default_features: bool,
    build_target_dir: &Path,
    lib_name: &str,
    profile: Profile,
) -> Result<CompileOptions> {
    // Configure compilation options so that we will build the desired build_target
    let mut opts = CompileOptions::new(workspace.gctx(), UserIntent::Build)?;

    // The legacy executor turns one binary root target into a shared library and passes
    // output-specific arguments to rustc. Cargo requires those arguments to apply to exactly
    // one target, so do not rely on its default target selection when a package also has a
    // library target.
    let binary_targets = workspace
        .current()?
        .targets()
        .iter()
        .filter(|target| target.is_bin())
        .map(|target| target.name().to_owned())
        .collect::<Vec<_>>();
    let [binary_target] = binary_targets.as_slice() else {
        return Err(anyhow::Error::msg(format!(
            "legacy Android wrappers require exactly one binary target, found {}",
            binary_targets.len()
        ))
        .into());
    };
    opts.filter = CompileFilter::single_bin(binary_target.clone());

    // Set the compilation target
    opts.build_config.requested_kinds = vec![CompileKind::Target(CompileTarget::new(
        build_target.rust_triple(),
        false,
    )?)];

    // Set features options
    opts.cli_features =
        CliFeatures::from_command_line(features, all_features, !no_default_features)?;

    // Set the path and file name for the generated shared library
    opts.target_rustc_args = Some(vec![format!(
        "--emit=link={}",
        build_target_dir
            .join(lib_name)
            .into_os_string()
            .into_string()
            .unwrap()
    )]);

    // Set desired profile
    if profile == Profile::Release {
        opts.build_config.requested_profile = "release".into();
    }

    Ok(opts)
}
