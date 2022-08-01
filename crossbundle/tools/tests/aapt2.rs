use crossbundle_tools::{commands::android::save_android_manifest, tools::AndroidSdk, types::*};

#[test]
fn test_aapt2_compile() {
    // Creates a temporary directory
    let tempfile = tempfile::tempdir().unwrap();
    let compiled_res_dir = tempfile.path().to_path_buf();
    assert!(compiled_res_dir.exists());

    // Specifies path to resources
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = user_dirs.parent().unwrap().parent().unwrap().to_path_buf();
    let res_path = dir
        .join("examples")
        .join("bevy-2d")
        .join("res")
        .join("android")
        .join("mipmap-hdpi");
    assert!(res_path.exists());

    // Specifies path to AndroidSdk
    let sdk = AndroidSdk::from_env().unwrap();

    // Compiles resources
    let compiled_res = sdk
        .aapt2()
        .unwrap()
        .compile_incremental(&res_path, &compiled_res_dir)
        .run()
        .unwrap();
    assert!(compiled_res.exists());
}

#[test]
fn test_aapt2_link() {
    // Creates a temporary directory
    let tempfile = tempfile::tempdir().unwrap();
    let tempdir = tempfile.path().to_path_buf();
    assert!(tempdir.exists());

    // Specifies path to needed resources
    let sdk = AndroidSdk::from_env().unwrap();
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = user_dirs.parent().unwrap().parent().unwrap().to_path_buf();
    let res_path = dir
        .join("examples")
        .join("bevy-2d")
        .join("res")
        .join("android")
        .join("mipmap-hdpi");
    assert!(res_path.exists());

    // Compiles resources for aapt2 link
    let aapt2_compile = sdk
        .aapt2()
        .unwrap()
        .compile_incremental(dunce::simplified(&res_path), dunce::simplified(&tempdir));
    let compiled_res = aapt2_compile.run().unwrap();
    assert!(compiled_res.exists());

    // Generates minimal android manifest
    let mut android_manifest = android_manifest::AndroidManifest {
        package: "com.crossbow.example".to_owned(),
        ..Default::default()
    };
    update_android_manifest_with_default(
        &mut android_manifest,
        Some("Example".to_owned()),
        "example",
        false,
    );

    // Saves android manifest into temporary directory
    let manifest_path = save_android_manifest(&tempdir, &android_manifest).unwrap();
    assert!(manifest_path.exists());

    // Link files and generates apk file
    let apk_path = tempdir.join("test.apk");
    let target_sdk_version = 30;
    let mut aapt2_link =
        sdk.aapt2()
            .unwrap()
            .link_inputs(&[compiled_res], &apk_path, &manifest_path);
    aapt2_link
        .android_jar(sdk.android_jar(target_sdk_version).unwrap())
        .proto_format(true)
        .auto_add_overlay(true)
        .verbose(true);
    aapt2_link.run().unwrap();
}
