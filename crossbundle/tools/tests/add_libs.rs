use crossbundle_tools::{
    commands::{
        android::{self, rust_compile},
    },
    tools::{AndroidNdk, AndroidSdk},
    types::{AndroidTarget, ApplicationWrapper, IntoRustTriple, Profile},
};

#[test]
fn add_libs_into_aapt2_test() {
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

    // Assign needed configuration to compile rust for android with bevy
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let profile = Profile::Debug;
    let target_sdk_version = 30;
    let bevy_lib_name = "bevy_test_lib.so";
    let app_wrapper_for_bevy = ApplicationWrapper::NdkGlue;

    rust_compile(
        &ndk,
        build_target,
        &bevy_project_path,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
        &bevy_lib_name,
        app_wrapper_for_bevy,
    )
    .unwrap();
    println!("rust was compiled for bevy example");

    // Specifies needed directories to manage library location
    let mut libs = Vec::new();
    let out_dir = dir
        .parent()
        .unwrap()
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let compiled_lib = out_dir.join(bevy_lib_name);
    libs.push((compiled_lib, build_target));

    // Adds libs into ./target/aarch64-linux-android/debug/
    for (compiled_lib, build_target) in libs {
        let lib = android::add_libs_into_aapt2(
            &ndk,
            &compiled_lib,
            build_target,
            profile,
            target_sdk_version,
            &out_dir,
            &dir.parent().unwrap().join("target"),
        )
        .unwrap();
        assert!(lib.exists());
        println!("library saved in {:?}", lib);
    }
}
