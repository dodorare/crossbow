use creator_tools::types::*;
use creator_tools::*;

pub fn get_minimal_android_manifest(
    project_name: &str,
    target_sdk_version: u32,
) -> AndroidManifest {
    AndroidManifest {
        package_name: format!("rust.{}", project_name.replace("-", "_")),
        package_label: project_name.to_owned(),
        version_name: "1.2.3".to_owned(),
        version_code: VersionCode::from_semver("1.2.3").unwrap().to_code(1),
        split: None,
        target_name: project_name.replace("-", "_"),
        debuggable: false,
        target_sdk_version,
        min_sdk_version: 23,
        opengles_version: (3, 1),
        features: vec![],
        permissions: vec![],
        intent_filters: vec![],
        icon: None,
        fullscreen: false,
        orientation: None,
        application_metadatas: vec![],
        activity_metadatas: vec![],
    }
}

#[test]
fn test_android_full() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir).unwrap();

    // Create dependencies
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

    // Compile rust lib for android
    let target_sdk_version = 30;
    let profile = Profile::Release;
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    compile_rust_for_android(
        &ndk,
        Target::Lib,
        build_target,
        dir,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
    )
    .unwrap();
    let out_dir = dir
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let compiled_lib = out_dir.join(format!("lib{}.so", name));
    assert!(compiled_lib.exists());

    // Gen android manifest
    let target_dir = dir.join("target");
    let manifest = get_minimal_android_manifest(&name, target_sdk_version);
    let apk_build_dir = target_dir.join(&profile).join("apk");
    let manifest_path = gen_android_manifest(&apk_build_dir, &manifest).unwrap();
    assert!(manifest_path.exists());

    // Gen unaligned apk
    let unaligned_apk_path =
        gen_unaligned_apk(&sdk, &apk_build_dir, &manifest_path, None, None, &manifest).unwrap();
    assert!(unaligned_apk_path.exists());

    // Add all needed libs into apk
    add_libs_into_apk(
        &sdk,
        &ndk,
        &unaligned_apk_path,
        &compiled_lib,
        build_target,
        profile,
        23,
        &apk_build_dir,
        &target_dir,
    )
    .unwrap();

    // Align apk
    let aligned_apk_path = align_apk(
        &sdk,
        &unaligned_apk_path,
        &manifest.package_label,
        &apk_build_dir,
    )
    .unwrap();
    assert!(aligned_apk_path.exists());

    // Gen debug key for signing apk
    let key = gen_debug_key().unwrap();
    println!("{:?}", key);

    // Sign apk
    sign_apk(&sdk, &aligned_apk_path, key).unwrap();
}
