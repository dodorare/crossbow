use crate::error::*;
use crate::tools::*;
use crate::types::*;

use anyhow::format_err;
use cargo::{
    core::{
        compiler::{CompileKind, CompileMode, CompileTarget, Executor},
        manifest::TargetSourcePath,
        resolver::CliFeatures,
        shell::Verbosity,
        {PackageId, TargetKind, Workspace},
    },
    ops::CompileOptions,
    util::{CargoResult, Config as CargoConfig},
};
use cargo_util::ProcessBuilder;
use std::{
    ffi::{OsStr, OsString},
    fs,
    fs::File,
    io::Write,
    path::Path,
    path::PathBuf,
    sync::Arc,
};
use tempfile::Builder;

/// Compile macroquuad rust code for android
pub fn compile_macroquad_rust_for_android(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    project_path: &Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    target_sdk_version: u32,
    lib_name: &str,
) -> Result<()> {
    let cargo_config = CargoConfig::default()?;
    let workspace = Workspace::new(&project_path.join("Cargo.toml"), &cargo_config)?;

    let build_target_dir = workspace
        .root()
        .join("target")
        .join(build_target.rust_triple())
        .join(profile);
    fs::create_dir_all(&build_target_dir).unwrap();

    let triple = build_target.rust_triple();
    // Set environment variables needed for use with the cc crate
    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;
    let ar = ndk.toolchain_bin("ar", build_target)?;

    std::env::set_var(format!("CC_{}", triple), &clang);
    std::env::set_var(format!("CXX_{}", triple), &clang_pp);
    std::env::set_var(format!("AR_{}", triple), &ar);

    // Use libc++. It is current default C++ runtime
    std::env::set_var("CXXSTDLIB", "c++");

    // Generate cmake toolchain and set environment variables to allow projects which use the
    // cmake crate to build correctly
    let cmake_toolchain_path = write_cmake_toolchain(
        target_sdk_version,
        ndk.ndk_path(),
        &build_target_dir,
        build_target,
    )?;
    std::env::set_var("CMAKE_TOOLCHAIN_FILE", cmake_toolchain_path);
    std::env::set_var("CMAKE_GENERATOR", r#"Unix Makefiles"#);
    std::env::set_var("CMAKE_MAKE_PROGRAM", make_path(ndk.ndk_path()));

    // Configure compilation options so that we will build the desired build_target
    let config = workspace.config();
    config.shell().set_verbosity(Verbosity::Normal);
    let mut opts = CompileOptions::new(config, CompileMode::Build)?;

    opts.build_config.requested_kinds = vec![CompileKind::Target(CompileTarget::new(
        build_target.rust_triple(),
    )?)];

    // Set features options
    opts.cli_features =
        CliFeatures::from_command_line(&features, all_features, no_default_features)?;

    // Set the file name for the generated shared library
    opts.target_rustc_args = Some(vec![format!(
        "--emit=link={}",
        build_target_dir
            .join(lib_name)
            .into_os_string()
            .into_string()
            .unwrap()
    )]);

    // Set profile
    if profile == Profile::Release {
        opts.build_config.requested_profile = "release".into();
    }

    // Create executor
    let executor: Arc<dyn Executor> = Arc::new(SharedLibraryExecutor {
        ndk: ndk.clone(),
        target_sdk_version,
        profile,
        build_target_dir,
        build_target,
        nostrip: false,
    });

    // Compile all targets for the requested build target
    cargo::ops::compile_with_exec(&workspace, &opts, &executor)?;
    Ok(())
}

/// Executor which builds binary and example targets as static libraries
struct SharedLibraryExecutor {
    ndk: AndroidNdk,
    target_sdk_version: u32,

    build_target_dir: PathBuf,
    build_target: AndroidTarget,

    profile: Profile,
    nostrip: bool,
}

impl Executor for SharedLibraryExecutor {
    fn exec(
        &self,
        cmd: &ProcessBuilder,
        _id: PackageId,
        target: &cargo::core::Target,
        mode: CompileMode,
        on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>,
        on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>,
    ) -> CargoResult<()> {
        if mode == CompileMode::Build
            && (target.kind() == &TargetKind::Bin || target.kind() == &TargetKind::ExampleBin)
        {
            let mut new_args = cmd.get_args().to_owned();

            // Determine source path
            let path = if let TargetSourcePath::Path(path) = target.src_path() {
                path.to_owned()
            } else {
                // Ignore other values
                return Ok(());
            };

            let original_src_filepath = path.canonicalize()?;

            // Generate source file that will be built
            //
            // Determine the name of the temporary file
            let tmp_lib_file_prefix = format!(
                "__cargo_apk_{}",
                original_src_filepath
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(String::new)
            );

            // Create the temporary file
            let mut tmp_file = Builder::new()
                .prefix(&tmp_lib_file_prefix)
                .tempfile_in(original_src_filepath.parent().unwrap())?;

            let extra_code = r##"
mod cargo_apk_glue_code {
    extern "C" {
        pub fn sapp_ANativeActivity_onCreate(
            activity: *mut std::ffi::c_void,
            saved_state: *mut std::ffi::c_void,
            saved_state_size: usize,
        );
    }
    #[no_mangle]
    pub unsafe extern "C" fn ANativeActivity_onCreate(
        activity: *mut std::ffi::c_void,
        saved_state: *mut std::ffi::c_void,
        saved_state_size: usize,
    ) {
        sapp_ANativeActivity_onCreate(activity, saved_state, saved_state_size as _);
    }
    #[no_mangle]
    pub unsafe extern "C" fn sokol_main() {
        let _ = super::main();
    }
    #[link(name = "android")]
    #[link(name = "log")]
    #[link(name = "EGL")]
    #[link(name = "GLESv3")]
    extern "C" {}
}"##;

            let original_contents = fs::read_to_string(original_src_filepath).unwrap();
            writeln!(tmp_file, "{}\n{}", original_contents, extra_code)?;

            // Replace source argument
            let filename = path.file_name().unwrap().to_owned();
            let source_arg = new_args.iter_mut().find_map(|arg| {
                let path_arg = Path::new(&arg);
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
                let path_arg = Path::new(&source_arg);
                let mut path_arg = path_arg.to_path_buf();
                path_arg.set_file_name(tmp_file.path().file_name().unwrap());
                *source_arg = path_arg.into_os_string();
            } else {
                return Err(format_err!(
                    "Unable to replace source argument when building target '{}'",
                    target.name()
                ));
            }

            // Create output directory inside the build target directory
            let build_path = self.build_target_dir.to_path_buf();
            fs::create_dir_all(&build_path).unwrap();

            // Change crate-type from bin to cdylib
            // Replace output directory with the directory we created
            let mut iter = new_args.iter_mut().rev().peekable();
            while let Some(arg) = iter.next() {
                if let Some(prev_arg) = iter.peek() {
                    if *prev_arg == "--crate-type" && arg == "bin" {
                        *arg = "cdylib".into();
                    } else if *prev_arg == "--out-dir" {
                        *arg = build_path.clone().into();
                    }
                }
            }

            // Helper function to build arguments composed of concatenating two strings
            fn build_arg(start: &str, end: impl AsRef<OsStr>) -> OsString {
                let mut new_arg = OsString::new();
                new_arg.push(start);
                new_arg.push(end.as_ref());
                new_arg
            }

            // Determine paths
            let tool_root = &self.ndk.toolchain_dir().unwrap();
            let linker_path = tool_root
                .join("bin")
                .join(format!("{}-ld.gold", &self.build_target.ndk_triple()));
            let sysroot = tool_root.join("sysroot");
            let version_independent_libraries_path = sysroot
                .join("usr")
                .join("lib")
                .join(&self.build_target.ndk_triple());
            let version_specific_libraries_path =
                AndroidNdk::find_ndk_path(self.target_sdk_version, |platform| {
                    version_independent_libraries_path.join(platform.to_string())
                })
                .map_err(|_| format_err!("unable to find NDK file"))?; // TODO: Fix this error
                                                                       // casting
            let gcc_lib_path = tool_root
                .join("lib/gcc")
                .join(&self.build_target.ndk_triple())
                .join("4.9.x");

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
            if !self.nostrip && self.profile == Profile::Release {
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
        } else if mode == CompileMode::Test {
            // This occurs when --all-targets is specified
            eprintln!("Ignoring CompileMode::Test for target: {}", target.name());
        } else if mode == CompileMode::Build {
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

/// Write a CMake toolchain which will remove references to the rustc build target before
/// including the NDK provided toolchain. The NDK provided android toolchain will set the
/// target appropriately Returns the path to the generated toolchain file
fn write_cmake_toolchain(
    min_sdk_version: u32,
    ndk_path: &Path,
    build_target_dir: &Path,
    build_target: AndroidTarget,
) -> CargoResult<PathBuf> {
    let toolchain_path = build_target_dir.join("cargo-apk.toolchain.cmake");
    let mut toolchain_file = File::create(&toolchain_path).unwrap();
    writeln!(
        toolchain_file,
        r#"set(ANDROID_PLATFORM android-{min_sdk_version})
set(ANDROID_ABI {abi})
string(REPLACE "--target={build_target}" "" CMAKE_C_FLAGS "${{CMAKE_C_FLAGS}}")
string(REPLACE "--target={build_target}" "" CMAKE_CXX_FLAGS "${{CMAKE_CXX_FLAGS}}")
unset(CMAKE_C_COMPILER CACHE)
unset(CMAKE_CXX_COMPILER CACHE)
include("{ndk_path}/build/cmake/android.toolchain.cmake")"#,
        min_sdk_version = min_sdk_version,
        ndk_path = ndk_path.to_string_lossy().replace("\\", "/"), /* Use forward slashes even on
                                                                   * windows to avoid path
                                                                   * escaping issues. */
        build_target = build_target.rust_triple(),
        abi = build_target.android_abi(),
    )?;
    Ok(toolchain_path)
}

/// Returns path to NDK provided make
fn make_path(ndk_path: &Path) -> PathBuf {
    ndk_path.join("prebuild").join(HOST_TAG).join("make")
}

#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
const HOST_TAG: &str = "windows-x86_64";

#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
const HOST_TAG: &str = "windows";

#[cfg(target_os = "linux")]
const HOST_TAG: &str = "linux-x86_64";

#[cfg(target_os = "macos")]
const HOST_TAG: &str = "darwin-x86_64";
