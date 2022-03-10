use crossbundle_tools::{
    commands::{
        android::{self, compile_rust_for_android},
        gen_minimal_project,
    },
    tools::{AndroidNdk, AndroidSdk},
    types::{AndroidTarget, ApplicationWrapper, IntoRustTriple, Profile},
};

#[test]
fn add_libs_into_aapt2_test() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();

    // Assigns configuration for project
    let macroquad_project = false;
    let package_name = gen_minimal_project(project_path, macroquad_project).unwrap();

    // Assigns configuration for project
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let profile = Profile::Debug;
    let target_sdk_version = 30;
    let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
    let target_dir = project_path.join("target");
    let android_build_dir = target_dir.join("android").join(profile.to_string());

    // Compile rust code for android with bevy engine
    compile_rust_for_android(
        &ndk,
        build_target,
        project_path,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
        &lib_name,
        ApplicationWrapper::NdkGlue,
    )
    .unwrap();

    // Specifies needed directories to manage library location
    let mut libs = Vec::new();
    let out_dir = target_dir
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let compiled_lib = out_dir.join(lib_name);
    libs.push((compiled_lib, build_target));

    // Adds libs into specified directory
    for (compiled_lib, build_target) in libs {
        let lib = android::add_libs_into_aapt2(
            &ndk,
            &compiled_lib,
            build_target,
            profile,
            target_sdk_version,
            &android_build_dir,
            &target_dir,
        )
        .unwrap();
        assert!(lib.exists());
        println!("library saved in {:?}", lib);
    }
}
