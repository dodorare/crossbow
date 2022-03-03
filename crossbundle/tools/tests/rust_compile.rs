#[cfg(target_os = "macos")]
use crossbundle_tools::commands::{apple::*, gen_minimal_project};
use crossbundle_tools::{
    commands::android::rust_compile,
    tools::{AndroidNdk, AndroidSdk},
    types::*,
};

#[test]
fn test_rust_compile() {
    // Specify path to example directory
    let current_dir = std::env::current_dir().unwrap();
    let dir = current_dir
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

    // TODO: Implement drop trait
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

#[test]
#[cfg(target_os = "macos")]
fn test_compile_apple() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir, false).unwrap();

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
