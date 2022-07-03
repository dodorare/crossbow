use crossbundle_tools::{
    commands::{
        android::{self, rust_compile, GenAndroidManifest},
        gen_minimal_project,
    },
    tools::{AndroidNdk, AndroidSdk},
    types::{AndroidTarget, ApplicationWrapper, IntoRustTriple, Profile},
};

#[test]
pub fn test_gen_gradle_project() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();

    // Assigns configuration for project
    let package_name = gen_minimal_project(&project_path, true).unwrap();

    // Assign needed configuration to compile rust for android with bevy
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let profile = Profile::Release;
    let target_sdk_version = 30;
    let version_code = 1_u32;
    let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
    let app_wrapper = ApplicationWrapper::Sokol;

    let android_build_dir = project_path
        .join("target")
        .join("android")
        .join(&package_name);
    std::fs::create_dir_all(&android_build_dir).unwrap();

    // Generate gradle project
    let gradle_project_path = android::gen_gradle_project(&android_build_dir, None, None).unwrap();

    // Generate manifest
    let manifest = GenAndroidManifest {
        package_name: package_name.to_string(),
        version_code,
        ..Default::default()
    };
    let android_manifest = manifest.gen_min_android_manifest();
    let manifest_path =
        android::save_android_manifest(&gradle_project_path, &android_manifest).unwrap();
    assert!(manifest_path.exists());

    // Compile rust code for android with bevy engine
    rust_compile(
        &ndk,
        build_target,
        project_path,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
        &lib_name,
        app_wrapper,
    )
    .unwrap();
    println!("rust was compiled for example");

    // Specifies needed directories to manage library location
    let mut libs = Vec::new();
    let out_dir = project_path
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let compiled_lib = out_dir.join(lib_name);
    assert!(compiled_lib.exists());
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
            &project_path.join("target"),
            &package_name,
        )
        .unwrap();
        assert!(lib.exists());
        println!("library saved in {:?}", lib);

        // Check the size of the library to ensure it is not corrupted
        for entry in std::fs::read_dir(&lib).unwrap() {
            let library = entry.unwrap().path();
            let size = std::fs::metadata(&library).unwrap().len();
            println!("library size is {:?}", size);
        }
    }
}
