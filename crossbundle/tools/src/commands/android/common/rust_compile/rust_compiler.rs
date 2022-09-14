use super::*;
use crate::{error::*, types::*};

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
    app_wrapper: AppWrapper,
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
            };

            // Replace source argument
            let filename = path.file_name().unwrap().to_owned();
            let source_arg = new_args.iter_mut().find_map(|arg| {
                let tmp = std::path::Path::new(&arg).file_name().unwrap();
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

            // Determine paths to linker and libgcc using in ndk =< 22
            println!(" sdk version {}", self.target_sdk_version);
            let mut cmd = cmd.clone();
            let linker_path = self
                .ndk
                .linker_path(&self.build_target, self.target_sdk_version)?;
            if self.ndk.build_tag() > 7272597 {
                // let error_msg = anyhow::Error::msg("Failed to write content into libgcc.a file");
                // let mut args = match self.app_wrapper {
                //     AppWrapper::Quad => {
                //         new_ndk_quad_args(tool_root, &self.build_target, self.target_sdk_version)
                //             .map_err(|_| error_msg)?
                //     }
                //     AppWrapper::NdkGlue => linker_args(&tool_root).map_err(|_| error_msg)?,
                // };
                // new_args.append(&mut args);
                // cmd.args_replace(&new_args);
                // cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                //     .map(drop)?;
                // Add linker arguments
                // Specify linker
                new_args.push(build_arg("-Clinker=", linker_path));

                let mut args =
                    search_for_libgcc_and_libunwind(&self.build_target, build_path, &self.ndk)?;
                new_args.append(&mut args);
                // Set linker flavor
                // new_args.push("-Clinker-flavor=ld".into());

                // // Set system root
                // new_args.push(build_arg("-Clink-arg=--sysroot=", sysroot));

                // // Add version specific libraries directory to search path
                // new_args.push(build_arg("-Clink-arg=-L", &version_specific_libraries_path));

                // // Add version independent libraries directory to search path
                // new_args.push(build_arg("-Clink-arg=-L", &sysroot_lib_dir));

                // let link_dir = self.ndk.tool_root()?.join("libgcc");

                // std::fs::create_dir_all(&link_dir)?;
                // std::fs::write(link_dir.join("libgcc.a"), "INPUT(-lunwind)")?;
                // new_args.push(build_arg("-L", link_dir));
                // Strip symbols for release builds
                // if !self.nostrip && self.profile == Profile::Release {
                //     new_args.push("-Clink-arg=-strip-all".into());
                // }

                // // Require position independent code
                // new_args.push("-Crelocation-model=pic".into());
            } else {
                let mut args =
                    add_clinker_args(&self.ndk, &self.build_target, self.target_sdk_version)?;
                new_args.append(&mut args);
                // let mut args = match self.app_wrapper {
                //     AppWrapper::Quad => {
                //         add_clinker_args(&self.ndk, &self.build_target, self.target_sdk_version)?
                //     }
                //     AppWrapper::NdkGlue => linker_args(&tool_root, linker_path).map_err(|_| {
                //         anyhow::Error::msg("Failed to write content into libgcc.a file")
                //     })?,
                // };
                // new_args.append(&mut args);
                // new_args.push(build_arg("-Clinker=", linker_path));

                // // Set linker flavor
                // new_args.push("-Clinker-flavor=ld".into());

                // // Set system root
                // new_args.push(build_arg("-Clink-arg=--sysroot=", sysroot));

                // // Add version specific libraries directory to search path
                // new_args.push(build_arg("-Clink-arg=-L", &version_specific_libraries_path));

                // // Add version independent libraries directory to search path
                // new_args.push(build_arg("-Clink-arg=-L", &sysroot_lib_dir));

                // let link_dir = self.ndk.tool_root()?.join("libgcc");

                // std::fs::create_dir_all(&link_dir)?;
                // std::fs::write(link_dir.join("libgcc.a"), "INPUT(-lunwind)")?;
                // new_args.push(build_arg("-L", link_dir));
                // Add path containing libgcc.a and libunwind.a for linker to search.
                // See https://github.com/rust-lang/rust/pull/85806 for discussion on libgcc.
                // The workaround to get to NDK r23 or newer is to create a libgcc.a file with
                // the contents of 'INPUT(-lunwind)' to link in libunwind.a instead of libgcc.a
                // let libgcc_dir = build_path.join("_libgcc_");
                // std::fs::create_dir_all(&libgcc_dir)?;
                // let libgcc = libgcc_dir.join("libgcc.a");
                // std::fs::write(&libgcc, "INPUT(-lunwind)")?;
                // new_args.push(build_arg("-Clink-arg=-L", libgcc_dir));
                // let libunwind_dir = self.ndk.find_libunwind_dir(&self.build_target)?;
                // new_args.push(build_arg("-Clink-arg=-L", libunwind_dir));

                // Require position independent code
                // new_args.push("-Crelocation-model=pic".into());
                // Determine paths to linker and libgcc using in ndk =< 22
                // let tool_root = self.ndk.toolchain_dir().unwrap();
                // let gcc_lib_path = tool_root
                //     .join("lib/gcc")
                //     .join(self.build_target.ndk_triple())
                //     .join("4.9.x");
                // let sysroot = tool_root.join("sysroot");
                // let version_independent_libraries_path = sysroot
                //     .join("usr")
                //     .join("lib")
                //     .join(self.build_target.ndk_triple());
                // let version_specific_libraries_path =
                //     AndroidNdk::find_ndk_path(self.target_sdk_version, |platform| {
                //         version_independent_libraries_path.join(platform.to_string())
                //     })
                //     .map_err(|_| anyhow::Error::msg("Android SDK not found"))?;

                // // Add linker arguments
                // // Specify linker
                // new_args.push(build_arg("-Clinker=", linker_path));

                // // Set linker flavor
                // new_args.push("-Clinker-flavor=ld".into());

                // // Set system root
                // new_args.push(build_arg("-Clink-arg=--sysroot=", sysroot));

                // // Add version specific libraries directory to search path
                // new_args.push(build_arg("-Clink-arg=-L", &version_specific_libraries_path));

                // // Add version independent libraries directory to search path
                // new_args.push(build_arg(
                //     "-Clink-arg=-L",
                //     &version_independent_libraries_path,
                // ));

                // // Add path to folder containing libgcc.a to search path
                // new_args.push(build_arg("-Clink-arg=-L", gcc_lib_path));

                // // Strip symbols for release builds
                // if !self.nostrip && self.profile == Profile::Release {
                //     new_args.push("-Clink-arg=-strip-all".into());
                // }

                // // Require position independent code
                // new_args.push("-Crelocation-model=pic".into());
            }
            // Create new command
            let mut cmd = cmd.clone();
            cmd.args_replace(&new_args);

            //
            // Execute the command
            //
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)?;
            // // Workaround from https://github.com/rust-windowing/android-ndk-rs/issues/149:
            // // Rust (1.56 as of writing) still requires libgcc during linking, but this does
            // // not ship with the NDK anymore since NDK r23 beta 3.
            // // See https://github.com/rust-lang/rust/pull/85806 for a discussion on why libgcc
            // // is still required even after replacing it with libunwind in the source.
            // // XXX: Add an upper-bound on the Rust version whenever this is not necessary anymore.
            // let mut cmd = cmd.clone();
            // let build_tag = self.ndk.build_tag();
            // let tool_root = self.ndk.toolchain_dir().map_err(|_| {
            //     anyhow::Error::msg("Failed to get access to the toolchain directory")
            // })?;
            // let error_msg = anyhow::Error::msg("Failed to write content into libgcc.a file");
            // if build_tag > 7272597 {
            //     let mut args = match self.app_wrapper {
            //         AppWrapper::Quad => {
            //             new_ndk_quad_args(tool_root, &self.build_target, self.target_sdk_version)
            //                 .map_err(|_| error_msg)?
            //         }
            //         AppWrapper::NdkGlue => linker_args(&tool_root).map_err(|_| error_msg)?,
            //     };
            //     new_args.append(&mut args);
            //     cmd.args_replace(&new_args);
            //     cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            //         .map(drop)?;
            // } else if self.app_wrapper == AppWrapper::Quad {
            //     // Set linker arguments using in ndk =< 22
            //
            //     new_args.append(&mut linker_args);

            //     // Strip symbols for release builds
            //     if !self.nostrip && self.profile == Profile::Release {
            //         new_args.push("-Clink-arg=-strip-all".into());
            //     }

            //     // Create new command
            //     let mut cmd = cmd.clone();
            //     cmd.args_replace(&new_args);

            //     // Execute the command
            //     cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            //         .map(drop)?;
            // } else if self.app_wrapper == AppWrapper::NdkGlue {
            //     println!("I am here");
            //     let mut args = linker_args(&tool_root).map_err(|_| error_msg)?;
            //     new_args.append(&mut args);
            //     cmd.args_replace(&new_args);

            //     cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            //         .map(drop)?;
            // }
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

/// Helper function that allows to return environment argument with specified tool
pub fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace('-', "_");
    let env = format!("CARGO_TARGET_{}_{}", &utarget, tool);
    env.to_uppercase()
}
