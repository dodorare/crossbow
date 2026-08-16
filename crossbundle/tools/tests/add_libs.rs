#![cfg(feature = "android")]

use crossbundle_tools::{
    commands::{android::*, gen_minimal_project},
    types::*,
};

fn compile_and_package(macroquad: bool) {
    let project = tempfile::tempdir().unwrap();
    let package = gen_minimal_project(project.path(), macroquad).unwrap();
    let target_dir = project.path().join("target");
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(sdk.sdk_path()).unwrap();
    let target = AndroidTarget::Aarch64;
    let profile = Profile::Release;
    let library = standard_cargo_compile(
        &ndk,
        target,
        &project.path().join("Cargo.toml"),
        &package,
        &package,
        profile,
        &[],
        false,
        false,
        23,
        &target_dir,
    )
    .unwrap();

    let output = add_libs_into_aapt2(
        &ndk,
        &library,
        target,
        profile,
        23,
        project.path(),
        &target_dir,
        &package,
    )
    .unwrap();
    assert!(output.join(library.file_name().unwrap()).is_file());
}

#[test]
fn packages_standard_native_activity_library() {
    compile_and_package(false);
}

#[test]
fn packages_standard_miniquad_library() {
    compile_and_package(true);
}
