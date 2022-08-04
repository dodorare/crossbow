use crate::error::*;
use crate::types::*;
use cargo::{
    core::{
        compiler::{CompileKind, CompileMode, CompileTarget},
        resolver::CliFeatures,
        shell::Verbosity,
        Workspace,
    },
    ops::CompileOptions,
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
    let config = workspace.config();

    // Avoid too much log info
    config.shell().set_verbosity(Verbosity::Normal);

    let mut opts = CompileOptions::new(config, CompileMode::Build)?;

    // Set the compilation target
    opts.build_config.requested_kinds = vec![CompileKind::Target(CompileTarget::new(
        build_target.rust_triple(),
    )?)];

    // Set features options
    opts.cli_features =
        CliFeatures::from_command_line(features, all_features, no_default_features)?;

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
