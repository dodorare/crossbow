mod cmake_toolchain;
mod compile_bevy;
mod compile_macroquad;
mod compile_options;
mod consts;
mod gen_tmp_lib_file;
mod rust_compiler;

use crate::{error::*, tools::*, types::*};
pub use cmake_toolchain::*;
pub use compile_bevy::*;
use compile_macroquad::*;
pub use rust_compiler::*;
use std::ffi::{OsStr, OsString};

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

pub fn add_clinker_args(
    ndk: &AndroidNdk,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
) -> cargo::CargoResult<Vec<OsString>> {
    let linker_args = vec![
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
    Ok(linker_args)
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

/// Replace cmd with new arguments
pub fn new_ndk_quad_args(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_compile() {
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
        let app_wrapper_for_quad = ApplicationWrapper::Sokol;
        let app_wrapper_for_bevy = ApplicationWrapper::NdkGlue;

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
            app_wrapper_for_bevy,
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
            app_wrapper_for_quad,
        )
        .unwrap();
        println!("rust was compiled for quad example");
    }
}
