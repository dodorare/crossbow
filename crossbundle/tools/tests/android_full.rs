use android_tools::java_tools::{android_dir, AabKey, KeyAlgorithm, Keytool};
use crossbundle_tools::{
    commands::{
        android::{self, remove, rust_compile},
        gen_minimal_project,
    },
    tools::{AndroidNdk, AndroidSdk},
    types::*,
};

#[test]
/// Tests all tools for creating apk
fn test_android_full() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();
    let macroquad_project = true;
    let quad_package_name = gen_minimal_project(&project_path, macroquad_project).unwrap();

    // Create dependencies
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let target_sdk_version = 30;
    let profile = Profile::Release;
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let quad_lib_name = format!("lib{}.so", quad_package_name.replace("-", "_"));
    let app_wrapper_for_quad = ApplicationWrapper::Sokol;

    // Compile rust code for android with quad engine
    rust_compile(
        &ndk,
        build_target,
        &project_path,
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

    // Create needed directories
    let out_dir = project_path
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let compiled_lib = out_dir.join(format!("lib{}.so", quad_package_name));
    // Check the size of the library to ensure it is not corrupted
    if compiled_lib.exists() {
        let size = std::fs::metadata(&compiled_lib).unwrap().len();
        println!("library size is {:?}", size);
    }
    assert!(compiled_lib.exists());

    // Gen android manifest
    let target_dir = project_path.join("target");
    let manifest = android::gen_minimal_android_manifest(
        None,
        &quad_package_name,
        None,
        "0.0.1".to_string(),
        None,
        None,
        target_sdk_version,
        None,
        None,
        true,
        None,
    );
    let apk_build_dir = target_dir.join(&profile).join("apk");
    let manifest_path = android::save_android_manifest(&apk_build_dir, &manifest).unwrap();
    assert!(manifest_path.exists());

    // Gen unaligned apk
    let unaligned_apk_path = android::gen_unaligned_apk(
        &sdk,
        &project_path,
        &apk_build_dir,
        &manifest_path,
        None,
        None,
        &manifest.application.label.unwrap().to_string(),
        manifest.uses_sdk.unwrap().target_sdk_version.unwrap(),
    )
    .unwrap();
    assert!(unaligned_apk_path.exists());

    // Add all needed libs into apk
    android::add_libs_into_apk(
        &sdk,
        &ndk,
        &unaligned_apk_path,
        &compiled_lib,
        build_target,
        profile,
        29,
        &apk_build_dir,
        &target_dir,
    )
    .unwrap();

    // Align apk
    let aligned_apk_path =
        android::align_apk(&sdk, &unaligned_apk_path, &manifest.package, &apk_build_dir).unwrap();
    assert!(aligned_apk_path.exists());

    // Removes old keystore if it exists
    let android_dir = android_dir().unwrap();
    let target = vec![android_dir.join("aab.keystore")];
    remove(target).unwrap();

    // Gen debug key for signing apk
    let key = AabKey::new_default().unwrap();
    Keytool::new()
        .genkeypair(true)
        .v(true)
        .keystore(&key.key_path)
        .alias(&key.key_alias)
        .keypass(&key.key_pass)
        .storepass(&key.key_pass)
        .dname(&["CN=Android Debug,O=Android,C=US".to_owned()])
        .keyalg(KeyAlgorithm::RSA)
        .keysize(2048)
        .validity(10000)
        .run()
        .unwrap();

    // Sign apk
    android::sign_apk(&sdk, &aligned_apk_path, key).unwrap();
}
