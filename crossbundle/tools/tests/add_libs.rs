use crossbundle_tools::{
    commands::{
        android::{self, rust_compile},
        gen_minimal_project,
    },
    tools::{AndroidNdk, AndroidSdk},
    types::{AndroidTarget, ApplicationWrapper, IntoRustTriple, Profile},
};

#[test]
fn add_bevy_libs() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();

    // Assigns configuration for project
    let bevy_package_name = gen_minimal_project(&project_path, false).unwrap();

    // Assign needed configuration to compile rust for android with bevy
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let profile = Profile::Release;
    let target_sdk_version = 30;
    let bevy_lib_name = format!("lib{}.so", bevy_package_name.replace("-", "_"));
    let app_wrapper_for_bevy = ApplicationWrapper::NdkGlue;

    // Compile rust code for android with bevy engine
    rust_compile(
        &ndk,
        build_target,
        &project_path,
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
    let out_dir = project_path
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let bevy_compiled_lib = out_dir.join(bevy_lib_name);
    assert!(bevy_compiled_lib.exists());
    libs.push((bevy_compiled_lib, build_target));

    // Add libs into the directory ./target/aarch64-linux-android/debug/
    for (compiled_lib, build_target) in libs {
        let lib = android::add_libs_into_aapt2(
            &ndk,
            &compiled_lib,
            build_target,
            profile,
            target_sdk_version,
            &out_dir,
            &project_path.join("target"),
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

#[test]
fn add_quad_libs() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();

    // Assigns configuration for project
    let quad_package_name = gen_minimal_project(&project_path, true).unwrap();

    // Assign needed configuration to compile rust for android with bevy
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let profile = Profile::Release;
    let target_sdk_version = 30;
    let quad_lib_name = format!("lib{}.so", quad_package_name.replace("-", "_"));
    let app_wrapper_for_quad = ApplicationWrapper::Sokol;

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
        &quad_lib_name,
        app_wrapper_for_quad,
    )
    .unwrap();
    println!("rust was compiled for quad example");

    // Specifies needed directories to manage library location
    let mut libs = Vec::new();
    let out_dir = project_path
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let quad_compiled_lib = out_dir.join(quad_lib_name);
    assert!(quad_compiled_lib.exists());
    libs.push((quad_compiled_lib, build_target));

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
        )
        .unwrap();
        assert!(lib.exists());
        // std::thread::sleep(std::time::Duration::from_secs(11111));
        println!("library saved in {:?}", lib);

        // Check the size of the library to ensure it is not corrupted
        for entry in std::fs::read_dir(&lib).unwrap() {
            let library = entry.unwrap().path();
            let size = std::fs::metadata(&library).unwrap().len();
            println!("library size is {:?}", size);
        }
    }
}
