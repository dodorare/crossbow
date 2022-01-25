use android_tools::java_tools::{android_dir, AabKey, JarSigner, KeyAlgorithm, Keytool};
use crossbundle_tools::commands::android::remove;
use crossbundle_tools::commands::gen_minimal_mq_project;
use crossbundle_tools::{commands::android, tools::*, types::*};

#[test]
/// Tests all tools for creating aab
fn test_aab_full() {
    // Creates temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();

    // Assigns configuration for project
    let package_name = gen_minimal_mq_project(&project_path).unwrap();
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let target_sdk_version = 30;
    let profile = Profile::Debug;
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
    let target_dir = project_path.join("target");
    let android_build_dir = target_dir.join("android").join(profile.to_string());

    // Сompile rust code for android with macroquad engine
    android::compile_rust_for_android(
        &ndk,
        build_target,
        &project_path,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
        &lib_name,
        ApplicationWrapper::Sokol,
    )
    .unwrap();

    // Generates manifest
    let manifest = android::gen_minimal_android_manifest(
        None,
        &package_name,
        None,
        "0.0.1".to_string(),
        None,
        None,
        target_sdk_version,
        None,
        None,
        false,
    );
    let manifest_path = android::save_android_manifest(&android_build_dir, &manifest).unwrap();
    assert!(manifest_path.exists());

    // Compiles resources
    let compiled_res_path = android_build_dir.join("compiled_res");
    if !compiled_res_path.exists() {
        std::fs::create_dir_all(&compiled_res_path).unwrap();
    }
    let res_path = project_path.join("res");
    let aapt2_compile = sdk
        .aapt2()
        .unwrap()
        .compile_incremental(&res_path, &compiled_res_path);
    let compiled_res = aapt2_compile.run().unwrap();

    // Links all resources and creates .apk file
    let apk_path = android_build_dir.join(format!("{}_module.apk", package_name));
    let mut aapt2_link =
        sdk.aapt2()
            .unwrap()
            .link_compiled_res(Some(compiled_res), &apk_path, &manifest_path);
    aapt2_link
        .android_jar(sdk.android_jar(target_sdk_version).unwrap())
        .version_code(1)
        .proto_format(true)
        .auto_add_overlay(true);
    aapt2_link.run().unwrap();

    // Extracts files from .apk into /extracted_apk_files folder
    let output_dir = android_build_dir.join("extracted_apk_files");
    let extracted_apk_path = android::extract_apk(&apk_path, &output_dir).unwrap();
    assert!(extracted_apk_path.exists());

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
            &extracted_apk_path,
            &target_dir,
        )
        .unwrap();
        assert!(lib.exists());
    }

    // Generates zip archive
    let gen_zip_modules =
        android::gen_zip_modules(&android_build_dir, &package_name, &extracted_apk_path).unwrap();

    // Genenerates aab from given list of modules (zip, zip, zip)
    let aab_path = android::gen_aab_from_modules(
        &package_name,
        &[gen_zip_modules.clone()],
        &android_build_dir,
    )
    .unwrap();

    // Removes unnecessary files
    remove(vec![extracted_apk_path, gen_zip_modules]).unwrap();

    // Removes old keystore if it exists
    let android_dir = android_dir().unwrap();
    let target = vec![android_dir.join("aab.keystore")];
    remove(target).unwrap();

    // Create keystore with deafault configuration
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

    // Sign AAB with created keystore
    JarSigner::new(&aab_path, &key.key_alias)
        .keystore(&key.key_path)
        .storepass(key.key_pass.to_string())
        .verbose(true)
        .sigalg("SHA256withRSA".to_string())
        .digestalg("SHA-256".to_string())
        .run()
        .unwrap();

    // Creates apks from generated aab
    let apks = android_build_dir.join(format!("{}.apks", package_name));
    let _build_apks = BuildApks::new(&aab_path, &apks)
        .overwrite(true)
        .ks(&key.key_path)
        .ks_pass_pass(key.key_pass)
        .ks_key_alias(key.key_alias)
        .run()
        .unwrap();
}
