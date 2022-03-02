mod compile_bevy;
mod compile_macroquad;
mod compile_options;
mod consts;
mod gen_tmp_lib_file;
mod rust_compile;

use crate::{error::*, tools::*, types::*};
use compile_bevy::*;
use compile_macroquad::*;
use rust_compile::*;
use std::{
    ffi::{OsStr, OsString},
    io::Write,
};

/// Compiles rust code for android with macroquad engine
pub fn compile_rust_for_android(
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
    if app_wrapper == ApplicationWrapper::Sokol {
        compile_rust_for_android_with_mq(
            ndk,
            build_target,
            project_path,
            profile,
            features,
            all_features,
            no_default_features,
            target_sdk_version,
            lib_name,
        )
    } else {
        compile_rust_for_android_with_bevy(
            ndk,
            build_target,
            project_path,
            profile,
            features,
            all_features,
            no_default_features,
            target_sdk_version,
            lib_name,
        )
    }
}

/// Helper function to build arguments composed of concatenating two strings
fn build_arg(start: &str, end: impl AsRef<OsStr>) -> OsString {
    let mut new_arg = OsString::new();
    new_arg.push(start);
    new_arg.push(end.as_ref());
    new_arg
}

/// Helper function that allows to return environment argument with specified tool
pub fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace('-', "_");
    let env = format!("CARGO_TARGET_{}_{}", &utarget, tool);
    env.to_uppercase()
}

/// Replace libgcc file with unwind. libgcc was removed in ndk versions >=23.
/// This is workaround for gcc not found issue.
pub fn linker_args(tool_root: &std::path::Path) -> crate::error::Result<Vec<OsString>> {
    let mut new_args = Vec::new();
    let link_dir = tool_root.join("libgcc");

    std::fs::create_dir_all(&link_dir)?;
    std::fs::write(link_dir.join("libgcc.a"), "INPUT(-lunwind)")?;
    new_args.push(build_arg("-L", link_dir));

    Ok(new_args)
}

/// Sets needed environment variables
pub fn set_cmake_vars(
    build_target: AndroidTarget,
    ndk: &AndroidNdk,
    target_sdk_version: u32,
    build_target_dir: &std::path::Path,
) -> cargo::CargoResult<()> {
    // Return path to toolchain cmake file
    let cmake_toolchain_path = write_cmake_toolchain(
        target_sdk_version,
        ndk.ndk_path(),
        build_target_dir,
        build_target,
    )?;

    // Set cmake environment variables
    std::env::set_var("CMAKE_TOOLCHAIN_FILE", cmake_toolchain_path);
    std::env::set_var("CMAKE_GENERATOR", r#"Unix Makefiles"#);
    std::env::set_var("CMAKE_MAKE_PROGRAM", make_path(ndk.ndk_path()));
    Ok(())
}

/// Returns path to NDK provided make
pub fn make_path(ndk_path: &std::path::Path) -> std::path::PathBuf {
    ndk_path
        .join("prebuild")
        .join(self::consts::HOST_TAG)
        .join("make")
}

/// Write a CMake toolchain which will remove references to the rustc build target before
/// including the NDK provided toolchain. The NDK provided android toolchain will set the
/// target appropriately Returns the path to the generated toolchain file
pub fn write_cmake_toolchain(
    min_sdk_version: u32,
    ndk_path: &std::path::Path,
    build_target_dir: &std::path::Path,
    build_target: AndroidTarget,
) -> cargo::util::CargoResult<std::path::PathBuf> {
    let toolchain_path = build_target_dir.join("cargo-apk.toolchain.cmake");
    let mut toolchain_file = std::fs::File::create(&toolchain_path).unwrap();
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
        ndk_path = dunce::simplified(ndk_path).to_string_lossy(),
        build_target = build_target.rust_triple(),
        abi = build_target.android_abi(),
    )?;
    Ok(toolchain_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_rust_with_macroquad() {
        // Specify path to user directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_path = user_dirs.parent().unwrap().parent().unwrap();

        // Specify path to macroquad project example
        let project_path = project_path.join("examples").join("macroquad-3d");

        // Provide path to Android SDK and Android NDK
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        compile_rust_for_android_with_mq(
            &ndk,
            AndroidTarget::Aarch64LinuxAndroid,
            &project_path,
            Profile::Debug,
            vec![],
            false,
            false,
            30,
            "macroquad_test_lib.so",
        )
        .unwrap();
    }

    #[test]
    fn test_compile_rust_with_bevy() {
        // Specify path to users directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_path = user_dirs.parent().unwrap().parent().unwrap();

        // Specify path to bevy project example
        let project_path = project_path.join("examples").join("bevy-2d");

        // Assign needed configuration to compile rust for android with bevy
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        let profile = Profile::Debug;
        let target_sdk_version = 30;
        let lib_name = "bevy_test_lib.so";

        // Compile rust code for android with bevy engine
        compile_rust_for_android_with_bevy(
            &ndk,
            build_target,
            &project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
            lib_name,
        )
        .unwrap();
    }

    #[test]
    fn test_compile_rust() {
        // Specify path to users directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dir = user_dirs
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples");

        // Specify path to bevy project example
        let bevy_project_path = dir.join("bevy-2d");
        let quad_project_path = dir.join("macroquad-3d");

        // Assign needed configuration to compile rust for android with bevy
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        let profile = Profile::Debug;
        let target_sdk_version = 30;
        let bevy_lib_name = "bevy_test_lib.so";
        let quad_lib_name = "quad_test_lib.so";

        // Compile rust code for android with bevy engine
        rust_compile(
            &ndk,
            build_target,
            &bevy_project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
            bevy_lib_name,
            false,
        )
        .unwrap();
        println!("rust was compiled for bevy example");

        // Compile rust code for android with quad engine
        rust_compile(
            &ndk,
            build_target,
            &quad_project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
            quad_lib_name,
            true,
        )
        .unwrap();
        println!("rust was compiled for quad example");
    }
}
