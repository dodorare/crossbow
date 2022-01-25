#[cfg(target_os = "macos")]
use crossbundle_tools::commands::apple::*;
use crossbundle_tools::{
    commands::*,
    tools::{AndroidNdk, AndroidSdk},
    types::*,
};

#[test]
fn test_compile_android() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let package_name = gen_minimal_mq_project(&dir).unwrap();

    // Create dependencies
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let target_sdk_version = 30;
    let profile = Profile::Release;
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let lib_name = format!("lib{}.so", package_name.replace("-", "_"));

    // Compile rust code for android with macroquad engine
    android::compile_rust_for_android(
        &ndk,
        build_target,
        &dir,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
        &lib_name,
        ApplicationWrapper::Sokol,
    )
    .unwrap();
}

#[test]
#[cfg(target_os = "macos")]
fn test_compile_apple() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir).unwrap();

    compile_rust_for_ios(
        Target::Bin(name),
        AppleTarget::Aarch64AppleIos,
        dir,
        Profile::Release,
        vec![],
        false,
        false,
    )
    .unwrap();
}
