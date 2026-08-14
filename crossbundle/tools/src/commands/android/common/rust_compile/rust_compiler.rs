use super::*;
use crate::{error::*, types::*};
use anyhow::Context as _;

pub fn rust_compile(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    project_path: &std::path::Path,
    target_dir: &std::path::Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    min_sdk_version: u32,
    lib_name: &str,
    app_wrapper: AppWrapper,
) -> Result<()> {
    // Specify path to workspace
    let rust_triple = build_target.rust_triple();

    // Configure the tools inherited by Cargo and its build-script subprocesses without
    // mutating the process-global environment. Environment mutation is unsafe in Rust 2024
    // because other threads may be reading it concurrently.
    let (clang, clang_pp) = ndk.clang(build_target, min_sdk_version)?;
    let ar = ndk.toolchain_bin("ar", build_target)?;

    let build_target_dir = target_dir.join(rust_triple).join(profile);
    std::fs::create_dir_all(&build_target_dir).unwrap();

    let mut build_script_env = vec![
        (format!("CC_{rust_triple}"), clang.clone().into_os_string()),
        (format!("CXX_{rust_triple}"), clang_pp.into_os_string()),
        (format!("AR_{rust_triple}"), ar.into_os_string()),
        ("CXXSTDLIB".to_owned(), "c++".into()),
    ];
    build_script_env.extend(cmake_env(
        build_target,
        ndk,
        min_sdk_version,
        &build_target_dir,
    )?);

    let mut cargo_context = cargo::util::GlobalContext::default()?;
    configure_cargo(
        &mut cargo_context,
        rust_triple,
        clang.as_os_str(),
        &build_script_env,
    )?;
    let workspace = cargo::core::Workspace::new(&project_path.join("Cargo.toml"), &cargo_context)?;

    // Configure compilation options so that we will build the desired build_target
    let opts = compile_options::compile_options(
        &workspace,
        build_target,
        &features,
        all_features,
        no_default_features,
        &build_target_dir,
        lib_name,
        profile,
    )?;

    // Create the executor
    let executor: std::sync::Arc<dyn cargo::core::compiler::Executor> =
        std::sync::Arc::new(SharedLibraryExecutor {
            min_sdk_version,
            build_target_dir,
            build_target,
            ndk: ndk.clone(),
            app_wrapper,
        });

    // Compile all targets for the requested build target
    cargo::ops::compile_with_exec(&workspace, &opts, &executor)?;
    Ok(())
}

/// Executor which builds binary and example targets as static libraries
struct SharedLibraryExecutor {
    min_sdk_version: u32,
    build_target_dir: std::path::PathBuf,
    build_target: AndroidTarget,
    ndk: AndroidNdk,
    app_wrapper: AppWrapper,
}

impl cargo::core::compiler::Executor for SharedLibraryExecutor {
    fn exec(
        &self,
        cmd: &cargo_util::ProcessBuilder,
        _id: cargo::core::PackageId,
        target: &cargo::core::Target,
        mode: cargo::core::compiler::CompileMode,
        on_stdout_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
        on_stderr_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
    ) -> cargo::util::errors::CargoResult<()> {
        if mode == cargo::core::compiler::CompileMode::Build
            && (target.kind() == &cargo::core::manifest::TargetKind::Bin
                || target.kind() == &cargo::core::manifest::TargetKind::ExampleBin)
        {
            let mut new_args = cmd.get_args().cloned().collect::<Vec<_>>();

            let extra_code = match self.app_wrapper {
                AppWrapper::Quad => consts::QUAD_EXTRA_CODE,
                AppWrapper::NdkGlue => consts::NDK_GLUE_EXTRA_CODE,
                AppWrapper::Cargo => {
                    return Err(anyhow::Error::msg(
                        "the Cargo app wrapper cannot use the legacy compiler",
                    ));
                }
            };

            let path =
                if let cargo::core::manifest::TargetSourcePath::Path(path) = target.src_path() {
                    path.to_owned()
                } else {
                    // Ignore other values
                    return Ok(());
                };

            // Generate tmp_file with bevy or quad extra code depending on either quad or ndk glue
            // dependency
            let tmp_file = match self.app_wrapper {
                AppWrapper::Quad => gen_tmp_lib_file::generate_lib_file(&path, extra_code)?,
                AppWrapper::NdkGlue => gen_tmp_lib_file::generate_lib_file(&path, extra_code)?,
                AppWrapper::Cargo => unreachable!("handled above"),
            };

            // Replace source argument
            let filename = path.file_name().unwrap().to_owned();
            let source_arg = new_args.iter_mut().find_map(|arg| {
                let tmp = std::path::Path::new(&arg).file_name().unwrap();
                if filename == tmp { Some(arg) } else { None }
            });

            if let Some(source_arg) = source_arg {
                // Build a new relative path to the temporary source file and use it as the source
                // argument Using an absolute path causes compatibility issues in
                // some cases under windows If a UNC path is used then relative
                // paths used in "include* macros" may not work if the relative path
                // includes "/" instead of "\"
                let mut path_arg = std::path::PathBuf::from(&source_arg);
                path_arg.set_file_name(tmp_file.path().file_name().unwrap());
                *source_arg = path_arg.into_os_string();
            } else {
                return Err(anyhow::Error::msg(format!(
                    "Unable to replace source argument when building target: {}",
                    target.name()
                )));
            }

            // Create output directory inside the build target directory
            let build_path = self.build_target_dir.join("build");
            std::fs::create_dir_all(&build_path)
                .map_err(|_| anyhow::Error::msg("Failed to create build target directory"))?;

            // Change crate-type from bin to cdylib
            let mut iter = new_args.iter_mut().rev().peekable();
            while let Some(arg) = iter.next() {
                if let Some(prev_arg) = iter.peek() {
                    if *prev_arg == "--crate-type" && arg == "bin" {
                        *arg = "cdylib".into();
                    } else if *prev_arg == "--out-dir" {
                        *arg = self.build_target_dir.clone().into();
                    }
                }
            }
            let mut cmd = cmd.clone();
            // Workaround from https://github.com/rust-windowing/android-ndk-rs/issues/149:
            // Rust (1.56 as of writing) still requires libgcc during linking, but this does
            // not ship with the NDK anymore since NDK r23 beta 3.
            // See https://github.com/rust-lang/rust/pull/85806 for a discussion on why libgcc
            // is still required even after replacing it with libunwind in the source.
            // XXX: Add an upper-bound on the Rust version whenever this is not necessary anymore.
            if self.ndk.build_tag() > 7272597 {
                let mut args = search_for_libgcc_and_libunwind(
                    &self.build_target,
                    build_path,
                    &self.ndk,
                    self.min_sdk_version,
                )?;
                new_args.append(&mut args);
            } else {
                let mut args =
                    add_clinker_args(&self.ndk, &self.build_target, self.min_sdk_version)?;
                new_args.append(&mut args);
            }
            // Create new command

            cmd.args_replace(&new_args);

            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .with_context(|| format!("failed to execute Android compiler command: {cmd}"))
                .map(drop)?;
        } else if mode == cargo::core::compiler::CompileMode::Test {
            // This occurs when --all-targets is specified
            return Err(anyhow::Error::msg(format!(
                "Ignoring CompileMode::Test for target: {}",
                target.name()
            )));
        } else if mode == cargo::core::compiler::CompileMode::Build {
            let mut new_args = cmd.get_args().cloned().collect::<Vec<_>>();

            // Change crate-type from cdylib to rlib
            let mut iter = new_args.iter_mut().rev().peekable();
            while let Some(arg) = iter.next() {
                if let Some(prev_arg) = iter.peek()
                    && *prev_arg == "--crate-type"
                    && arg == "cdylib"
                {
                    *arg = "rlib".into();
                }
            }
            let mut cmd = cmd.clone();
            cmd.args_replace(&new_args);
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)?
        } else {
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)?
        }
        Ok(())
    }
}

/// Helper function that allows to return environment argument with specified tool
pub fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace('-', "_");
    let env = format!("CARGO_TARGET_{}_{}", utarget, tool);
    env.to_uppercase()
}

/// Configure embedded Cargo without changing the process-global environment.
fn configure_cargo(
    cargo_context: &mut cargo::util::GlobalContext,
    rust_triple: &str,
    linker: &std::ffi::OsStr,
    build_script_env: &[(String, std::ffi::OsString)],
) -> Result<()> {
    let mut overrides = Vec::with_capacity(build_script_env.len() * 2 + 2);
    if std::env::var_os("RUSTC").is_none() {
        overrides.push(config_value(
            "build.rustc",
            active_rustc_path()?.as_os_str(),
        ));
    }

    overrides.push(config_value(
        &format!("target.{rust_triple}.linker"),
        linker,
    ));
    for (name, value) in build_script_env {
        // Cargo's `--config` parser does not accept inline tables, so provide the two
        // fields as dotted keys. `force` preserves the old process-environment behavior:
        // Crossbundle's selected NDK tools take precedence over inherited host values.
        overrides.push(format!("env.{name}.value = {}", toml_string(value)));
        overrides.push(format!("env.{name}.force = true"));
    }

    cargo_context.configure(0, false, None, false, false, false, &None, &[], &overrides)?;
    Ok(())
}

fn config_value(key: &str, value: &std::ffi::OsStr) -> String {
    format!("{key} = {}", toml_string(value))
}

fn toml_string(value: &std::ffi::OsStr) -> String {
    format!("{:?}", value.to_string_lossy())
}

fn active_rustc_path() -> Result<std::path::PathBuf> {
    let mut command = std::process::Command::new("rustc");
    command.args(["--print", "sysroot"]);
    let output = command.output_err(false)?;
    let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let rustc = std::path::PathBuf::from(sysroot)
        .join("bin")
        .join(bin!("rustc"));
    if !rustc.is_file() {
        return Err(Error::PathNotFound(rustc));
    }
    Ok(rustc)
}

#[cfg(test)]
mod tests {
    use super::{active_rustc_path, configure_cargo};
    use std::ffi::{OsStr, OsString};
    use std::process::Command;

    #[test]
    fn cargo_accepts_build_script_environment_overrides() {
        let mut context = cargo::util::GlobalContext::default().unwrap();
        configure_cargo(
            &mut context,
            "aarch64-linux-android",
            OsStr::new("/android/clang"),
            &[(
                "CC_aarch64-linux-android".to_owned(),
                OsString::from("/android/clang"),
            )],
        )
        .unwrap();
    }

    #[test]
    fn resolved_rustc_ignores_dependency_toolchain_override() {
        let rustc = active_rustc_path().unwrap();
        assert!(rustc.is_absolute());

        let dependency_dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dependency_dir.path().join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"crossbundle-must-not-use-this-toolchain\"\n",
        )
        .unwrap();

        let output = Command::new(rustc)
            .arg("--version")
            .current_dir(dependency_dir.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "absolute rustc unexpectedly honored a dependency-local toolchain override: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
