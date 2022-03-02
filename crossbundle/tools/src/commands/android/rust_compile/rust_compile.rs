use crate::error::*;
use crate::tools::*;
use crate::types::*;

pub fn rust_compile(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    project_path: &std::path::Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    target_sdk_version: u32,
    lib_name: &str,
    quad: bool,
) -> Result<()> {
    // Specify path to workspace
    let cargo_config = cargo::util::Config::default()?;
    let workspace = cargo::core::Workspace::new(&project_path.join("Cargo.toml"), &cargo_config)?;
    let rust_triple = build_target.rust_triple();

    // Define directory to build project
    let build_target_dir = workspace
        .root()
        .join("target")
        .join(rust_triple)
        .join(profile);
    std::fs::create_dir_all(&build_target_dir).unwrap();

    // Set environment variables needed for use with the cc crate
    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;
    let ar = ndk.toolchain_bin("ar", build_target)?;

    std::env::set_var(format!("CC_{}", rust_triple), &clang);
    std::env::set_var(format!("CXX_{}", rust_triple), &clang_pp);
    std::env::set_var(format!("AR_{}", rust_triple), &ar);
    std::env::set_var(super::cargo_env_target_cfg("LINKER", rust_triple), &clang);

    super::set_cmake_vars(build_target, ndk, target_sdk_version, &build_target_dir)?;

    // Use libc++. It is current default C++ runtime
    std::env::set_var("CXXSTDLIB", "c++");

    // Configure compilation options so that we will build the desired build_target
    let opts = super::compile_options::compile_options(
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
            target_sdk_version,
            build_target_dir,
            build_target,
            ndk: ndk.clone(),
            profile,
            nostrip: false,
            quad,
        });

    // Compile all targets for the requested build target
    cargo::ops::compile_with_exec(&workspace, &opts, &executor)?;
    Ok(())
}

/// Executor which builds binary and example targets as static libraries
struct SharedLibraryExecutor {
    target_sdk_version: u32,
    build_target_dir: std::path::PathBuf,
    build_target: AndroidTarget,
    ndk: AndroidNdk,
    profile: Profile,
    nostrip: bool,
    quad: bool,
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
            // Determine source path
            let path =
                if let cargo::core::manifest::TargetSourcePath::Path(path) = target.src_path() {
                    path.to_owned()
                } else {
                    // Ignore other values
                    return Ok(());
                };

            let extra_code = match self.quad {
                true => super::consts::SOKOL_EXTRA_CODE,
                false => super::consts::NDK_GLUE_EXTRA_CODE,
            };

            let tmp_file = super::gen_tmp_lib_file::generate_lib_file(&path, extra_code)?;

            // Replaces source argument and returns collection of arguments
            get_cmd_args(
                &path,
                &self.ndk,
                tmp_file,
                &self.build_target_dir,
                target,
                cmd,
                &self.build_target,
                self.target_sdk_version,
                self.nostrip,
                self.profile,
                self.quad,
                on_stdout_line,
                on_stderr_line,
            )?;
        } else if mode == cargo::core::compiler::CompileMode::Test {
            // This occurs when --all-targets is specified
            return Err(anyhow::Error::msg(format!(
                "Ignoring CompileMode::Test for target: {}",
                target.name()
            )));
        } else if mode == cargo::core::compiler::CompileMode::Build {
            let mut new_args = cmd.get_args().to_owned();

            // Change crate-type from cdylib to rlib
            let mut iter = new_args.iter_mut().rev().peekable();
            while let Some(arg) = iter.next() {
                if let Some(prev_arg) = iter.peek() {
                    if *prev_arg == "--crate-type" && arg == "cdylib" {
                        *arg = "rlib".into();
                    }
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

/// Get the program arguments and execute program with it
fn get_cmd_args(
    path: &std::path::Path,
    ndk: &AndroidNdk,
    tmp_file: tempfile::NamedTempFile,
    build_target_dir: &std::path::Path,
    target: &cargo::core::Target,
    cmd: &cargo_util::ProcessBuilder,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
    nostrip: bool,
    profile: Profile,
    quad: bool,
    on_stdout_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
    on_stderr_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
) -> cargo::util::CargoResult<()> {
    let mut new_args = cmd.get_args().to_owned();

    // Replace source argument
    let filename = path.file_name().unwrap().to_owned();
    let source_arg = new_args.iter_mut().find_map(|arg| {
        let path_arg = std::path::Path::new(&arg);
        let tmp = path_arg.file_name().unwrap();

        if filename == tmp {
            Some(arg)
        } else {
            None
        }
    });

    if let Some(source_arg) = source_arg {
        // Build a new relative path to the temporary source file and use it as the source
        // argument Using an absolute path causes compatibility issues in
        // some cases under windows If a UNC path is used then relative
        // paths used in "include* macros" may not work if the relative path
        // includes "/" instead of "\"
        let path_arg = std::path::Path::new(&source_arg);
        let mut path_arg = path_arg.to_path_buf();
        path_arg.set_file_name(tmp_file.path().file_name().unwrap());
        *source_arg = path_arg.into_os_string();
    } else {
        return Err(anyhow::Error::msg(format!(
            "Unable to replace source argument when building target: {}",
            target.name()
        )));
    }

    // Create output directory inside the build target directory
    if !build_target_dir.exists() {
        std::fs::create_dir_all(&build_target_dir).unwrap();
    }

    // Change crate-type from bin to cdylib
    // Replace output directory with the directory we created
    let mut iter = new_args.iter_mut().rev().peekable();
    while let Some(arg) = iter.next() {
        if let Some(prev_arg) = iter.peek() {
            if *prev_arg == "--crate-type" && arg == "bin" {
                *arg = "cdylib".into();
            } else if *prev_arg == "--out-dir" {
                *arg = build_target_dir.clone().into();
            }
        }
    }

    let build_tag = ndk.build_tag();
    let tool_root = ndk.toolchain_dir().unwrap();
    // Workaround from https://github.com/rust-windowing/android-ndk-rs/issues/149:
    // Rust (1.56 as of writing) still requires libgcc during linking, but this does
    // not ship with the NDK anymore since NDK r23 beta 3.
    // See https://github.com/rust-lang/rust/pull/85806 for a discussion on why libgcc
    // is still required even after replacing it with libunwind in the source.
    // XXX: Add an upper-bound on the Rust version whenever this is not necessary anymore.
    if build_tag > 7272597 {
        let error_msg = anyhow::Error::msg("Failed to write content into libgcc.a file");
        let mut args = match quad {
            true => new_quad_cmd_args(tool_root, build_target, target_sdk_version)
                .map_err(|_| error_msg)?,
            false => super::linker_args(&tool_root).map_err(|_| error_msg)?,
        };
        new_args.append(&mut args);
    } else {
        if quad {
            let mut old_ndk_args = old_ndk_quad_args(
                ndk,
                build_target,
                target_sdk_version,
                nostrip,
                profile,
                cmd,
                on_stdout_line,
                on_stderr_line,
            )
            .map_err(|_| {
                anyhow::Error::msg("Failed to get arguments for macroquad in old ndk version")
            })?;
            new_args.append(&mut old_ndk_args)
        }
    }
    Ok(())
}

/// Invocate vector of arguments if ndk version <23 was found
pub fn old_ndk_quad_args(
    ndk: &AndroidNdk,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
    nostrip: bool,
    profile: Profile,
    cmd: &cargo_util::ProcessBuilder,
    on_stdout_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
    on_stderr_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
) -> crate::error::Result<Vec<std::ffi::OsString>> {
    let mut new_args = cmd.get_args().to_owned();
    // Determine paths to linker and libgcc using in ndk =< 22
    let tool_root = ndk.toolchain_dir().unwrap();
    let linker_path = tool_root
        .join("bin")
        .join(format!("{}-ld.gold", build_target.ndk_triple()));
    let gcc_lib_path = tool_root
        .join("lib/gcc")
        .join(build_target.ndk_triple())
        .join("4.9.x");
    let sysroot = tool_root.join("sysroot");
    let version_independent_libraries_path = sysroot
        .join("usr")
        .join("lib")
        .join(build_target.ndk_triple());
    let version_specific_libraries_path =
        AndroidNdk::find_ndk_path(target_sdk_version, |platform| {
            version_independent_libraries_path.join(platform.to_string())
        })
        .map_err(|_| anyhow::Error::msg("Android SDK not found"))?;

    // Add linker arguments
    // Specify linker
    new_args.push(build_arg("-Clinker=", linker_path));

    // Set linker flavor
    new_args.push("-Clinker-flavor=ld".into());

    // Set system root
    new_args.push(build_arg("-Clink-arg=--sysroot=", sysroot));

    // Add version specific libraries directory to search path
    new_args.push(build_arg("-Clink-arg=-L", &version_specific_libraries_path));

    // Add version independent libraries directory to search path
    new_args.push(build_arg(
        "-Clink-arg=-L",
        &version_independent_libraries_path,
    ));

    // Add path to folder containing libgcc.a to search path
    new_args.push(build_arg("-Clink-arg=-L", gcc_lib_path));

    // Strip symbols for release builds
    if !nostrip && profile == Profile::Release {
        new_args.push("-Clink-arg=-strip-all".into());
    }

    // Require position independent code
    new_args.push("-Crelocation-model=pic".into());

    // Create new command
    let mut cmd = cmd.clone();
    cmd.args_replace(&new_args);

    // Execute the command
    cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
        .map(drop)?;
    Ok(new_args)
}

/// Replace cmd with new arguments
pub fn new_quad_cmd_args(
    tool_root: std::path::PathBuf,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
) -> crate::error::Result<Vec<std::ffi::OsString>> {
    let mut new_args = super::linker_args(&tool_root)?;
    #[cfg(target_os = "windows")]
    let ext = ".cmd";
    #[cfg(not(target_os = "windows"))]
    let ext = "";
    let linker_path = tool_root.join("bin").join(format!(
        "{}{}-clang{}",
        build_target.rust_triple(),
        target_sdk_version,
        ext,
    ));
    new_args.push(build_arg("-Clinker=", linker_path));
    Ok(new_args)
}

/// Helper function to build arguments composed of concatenating two strings
fn build_arg(start: &str, end: impl AsRef<std::ffi::OsStr>) -> std::ffi::OsString {
    let mut new_arg = std::ffi::OsString::new();
    new_arg.push(start);
    new_arg.push(end.as_ref());
    new_arg
}
