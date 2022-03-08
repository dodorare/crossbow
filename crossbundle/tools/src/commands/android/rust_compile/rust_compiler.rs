use super::*;
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
    app_wrapper: ApplicationWrapper,
) -> Result<()> {
    // Specify path to workspace
    let rust_triple = build_target.rust_triple();

    // Set environment variables needed for use with the cc crate
    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;
    std::env::set_var(format!("CC_{}", rust_triple), &clang);
    std::env::set_var(format!("CXX_{}", rust_triple), &clang_pp);
    std::env::set_var(cargo_env_target_cfg("LINKER", rust_triple), &clang);
    let ar = ndk.toolchain_bin("ar", build_target)?;
    std::env::set_var(format!("AR_{}", rust_triple), &ar);

    let cargo_config = cargo::util::Config::default()?;
    let workspace = cargo::core::Workspace::new(&project_path.join("Cargo.toml"), &cargo_config)?;

    // Define directory to build project
    let build_target_dir = workspace
        .root()
        .join("target")
        .join(rust_triple)
        .join(profile);
    std::fs::create_dir_all(&build_target_dir).unwrap();

    set_cmake_vars(build_target, ndk, target_sdk_version, &build_target_dir)?;

    // Use libc++. It is current default C++ runtime
    std::env::set_var("CXXSTDLIB", "c++");

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

    let lib_path = project_path.join("src").join("main.rs");
    // Create the executor
    let executor: std::sync::Arc<dyn cargo::core::compiler::Executor> =
        std::sync::Arc::new(SharedLibraryExecutor {
            target_sdk_version,
            build_target_dir,
            build_target,
            ndk: ndk.clone(),
            profile,
            nostrip: false,
            app_wrapper,
            lib_path,
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
    app_wrapper: ApplicationWrapper,
    lib_path: std::path::PathBuf,
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
            let mut cmd = cmd.clone();
            // let ndk_glue_extra_code = super::consts::NDK_GLUE_EXTRA_CODE;
            // let tmp_file =
            //     super::gen_tmp_lib_file::generate_lib_file(&self.lib_path, ndk_glue_extra_code)?;

            let mut new_args = cmd.get_args().to_owned();

            let extra_code = match self.app_wrapper {
                ApplicationWrapper::Sokol => consts::SOKOL_EXTRA_CODE,
                ApplicationWrapper::NdkGlue => consts::NDK_GLUE_EXTRA_CODE,
            };

            let path =
                if let cargo::core::manifest::TargetSourcePath::Path(path) = target.src_path() {
                    path.to_owned()
                } else {
                    // Ignore other values
                    return Ok(());
                };

            // Generate tmp_file with bevy or quad extra code depending on either sokol or ndk glue
            // dependency
            let tmp_file = match self.app_wrapper {
                ApplicationWrapper::Sokol => {
                    gen_tmp_lib_file::generate_lib_file(&path, extra_code)?
                }
                ApplicationWrapper::NdkGlue => {
                    gen_tmp_lib_file::generate_lib_file(&path, extra_code)?
                }
            };
            println!("JJJJJJJJJJJJJJJJJJJJJJJJ");

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
            std::fs::create_dir_all(&self.build_target_dir).unwrap();

            // Change crate-type from bin to cdylib
            // Replace output directory with the directory we created
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
            // Workaround from https://github.com/rust-windowing/android-ndk-rs/issues/149:
            // Rust (1.56 as of writing) still requires libgcc during linking, but this does
            // not ship with the NDK anymore since NDK r23 beta 3.
            // See https://github.com/rust-lang/rust/pull/85806 for a discussion on why libgcc
            // is still required even after replacing it with libunwind in the source.
            // XXX: Add an upper-bound on the Rust version whenever this is not necessary anymore.
            println!("::::::::::::::::::::::::::::::::::::::::::::::");
            let build_tag = self.ndk.build_tag();
            let tool_root = self.ndk.toolchain_dir().map_err(|_| {
                anyhow::Error::msg(format!("Failed to get access to the toolchain directory"))
            })?;
            // if build_tag > 7272597 {
            //     let args = super::linker_args(&tool_root).map_err(|_| {
            //         anyhow::Error::msg("Failed to write content into libgcc.a file")
            //     })?;
            //     for arg in args.into_iter() {
            //         new_args.push(arg);
            //     }
            // }
            if build_tag > 7272597 {
                let error_msg = anyhow::Error::msg("Failed to write content into libgcc.a file");
                let mut args = match self.app_wrapper {
                    ApplicationWrapper::Sokol => {
                        new_ndk_quad_args(tool_root, &self.build_target, self.target_sdk_version)
                            .map_err(|_| error_msg)?
                    }
                    ApplicationWrapper::NdkGlue => {
                        linker_args(&tool_root).map_err(|_| error_msg)?
                    }
                };
                println!("args: {:?}", args);
                new_args.append(&mut args);
                if self.app_wrapper == ApplicationWrapper::NdkGlue {
                    cmd.args_replace(&new_args);
                    println!("cmd: {:?}", cmd);
                    cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                        .map(drop)?;
                }
                println!("new args after append: {:?}", new_args);
            } else if self.app_wrapper == ApplicationWrapper::Sokol {
                // Set linker arguments using in ndk =< 22
                let mut linker_args =
                    add_clinker_args(&self.ndk, &self.build_target, self.target_sdk_version)?;
                new_args.append(&mut linker_args);

                // Strip symbols for release builds
                if !self.nostrip && self.profile == Profile::Release {
                    new_args.push("-Clink-arg=-strip-all".into());
                }
            }
            if self.app_wrapper == ApplicationWrapper::Sokol {
                // Create new command
                let mut cmd = cmd.clone();
                cmd.args_replace(&new_args);

                // Execute the command
                cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                    .map(drop)?;
            }
            println!("FFFFFFFFFFF");
        } else if mode == cargo::core::compiler::CompileMode::Test {
            // This occurs when --all-targets is specified
            println!("LLLLLLLLLLLLLLLLLLLLLLLL");
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
            println!("IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII");
            let mut cmd = cmd.clone();
            cmd.args_replace(&new_args);
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)?
        } else {
            println!("jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj");
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)?
        }
        Ok(())
    }
}

/// Get the program arguments and execute program with it
pub fn add_cmd_arguments(
    mut new_args: Vec<std::ffi::OsString>,
    build_tag: u32,
    tool_root: std::path::PathBuf,
    ndk: &AndroidNdk,
    cmd: &cargo_util::ProcessBuilder,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
    nostrip: bool,
    profile: Profile,
    app_wrapper: ApplicationWrapper,
    on_stdout_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
    on_stderr_line: &mut dyn FnMut(&str) -> cargo::util::errors::CargoResult<()>,
) -> cargo::util::CargoResult<Vec<std::ffi::OsString>> {
    if build_tag > 7272597 {
        let error_msg = anyhow::Error::msg("Failed to write content into libgcc.a file");
        let mut args = match app_wrapper {
            ApplicationWrapper::Sokol => {
                new_ndk_quad_args(tool_root, build_target, target_sdk_version)
                    .map_err(|_| error_msg)?
            }
            ApplicationWrapper::NdkGlue => linker_args(&tool_root).map_err(|_| error_msg)?,
        };
        println!("args: {:?}", args);
        new_args.append(&mut args);
        println!("new args after append: {:?}", new_args);
    } else if app_wrapper == ApplicationWrapper::Sokol {
        // Set linker arguments using in ndk =< 22
        let mut linker_args = vec![
            build_arg("-Clinker=", ndk.linker_path(&build_target)?),
            "-Clinker-flavor=ld".into(),
            build_arg("-Clink-arg=--sysroot=", ndk.sysroot()?),
            build_arg(
                "-Clink-arg=-L",
                ndk.version_specific_libraries_path(target_sdk_version, &build_target)?,
            ),
            build_arg(
                "-Clink-arg=-L",
                ndk.sysroot_lib_dir(&build_target).map_err(|_| {
                    anyhow::Error::msg(format!(
                        "Failed to get access to the {:?}",
                        ndk.sysroot_lib_dir(&build_target)
                    ))
                })?,
            ),
            build_arg("-Clink-arg=-L", ndk.gcc_lib_path(&build_target)?),
            "-Crelocation-model=pic".into(),
        ];
        new_args.append(&mut linker_args);

        // Strip symbols for release builds
        if !nostrip && profile == Profile::Release {
            new_args.push("-Clink-arg=-strip-all".into());
        }
    }
    if app_wrapper == ApplicationWrapper::Sokol {
        // Create new command
        let mut cmd = cmd.clone();
        cmd.args_replace(&new_args);

        // Execute the command
        cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            .map(drop)?;
    }
    Ok(new_args)
}
