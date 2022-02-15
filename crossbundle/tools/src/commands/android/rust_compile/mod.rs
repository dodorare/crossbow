mod compile_bevy;
mod compile_macroquad;
mod compile_options;
mod consts;
mod gen_tmp_lib_file;

use std::ffi::{OsStr, OsString};
use crate::{error::*, tools::*, types::*};
use compile_bevy::*;
use compile_macroquad::*;

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
            "libbevy-2d.so",
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
        let lib_name = "bevy_test_lib";

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
}
