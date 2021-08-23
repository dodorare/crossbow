use creator_tools::{
    commands::{android, gen_minimal_project},
    tools::*,
    types::*,
};
use std::path::Path;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_aab() {
        // Create temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let project_path = tempdir.path();
        let package_name = gen_minimal_project(&project_path).unwrap();

        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        let target_sdk_version = 30;
        let profile = Profile::Release;
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        android::compile_rust_for_android(
            &ndk,
            Target::Lib,
            build_target,
            &project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
        )
        .unwrap();
        let build_dir = project_path.join("target").join("android").join("debug");

        // Generate manifest
        let manifest = android::gen_minimal_android_manifest(
            &package_name,
            None,
            "0.0.1".to_string(),
            None,
            None,
            target_sdk_version,
            None,
            None,
        );
        let manifest_path = android::save_android_manifest(&build_dir, &manifest).unwrap();
        assert!(manifest_path.exists());

        // Gen apks and prepare modules (zip, zip, zip)
        let base_apk_path = android::gen_base_aab_module(
            &[Path::new("res\\mipmap\\Screenshot_2.png").to_owned()],
            None,
            &build_dir,
            &sdk,
            &package_name,
            &manifest_path,
            target_sdk_version,
        )
        .unwrap();
        assert!(base_apk_path.exists());

        let abi = build_target.android_abi();
        let compiled_lib = build_dir
            .join("lib")
            .join(abi)
            .join(format!("lib{}.so", package_name));
        if !compiled_lib.exists() {
            std::fs::create_dir_all(&compiled_lib).unwrap();
        }

        // let abi = build_target.android_abi();
        // let lib_build_dir = base_apk_path.join("lib").join(abi);
        let target_dir = project_path.join("target");
        // Assign path to lib
        let add_lib = android::add_libs_into_aapt2(
            &ndk,
            &compiled_lib,
            build_target,
            profile,
            30,
            &base_apk_path,
            &target_dir,
        )
        .unwrap();
        assert!(add_lib.exists());

        let gen_zip_modules =
            android::gen_zip_modules(&build_dir, &package_name, &base_apk_path).unwrap();
        assert!(gen_zip_modules.exists());

        // Gen aab from given list of modules (zip, zip, zip)
        let aab_path =
            android::gen_aab_from_modules(&package_name, &[gen_zip_modules], &build_dir).unwrap();
        assert!(aab_path.exists());

        // Create keystore with keytool command
        let build_apks = android::build_apks(&aab_path, &build_dir, &package_name).unwrap();
        assert!(build_apks.exists());

        // android::jarsigner(
        //     Path::new("res\\mipmap\\keystore"),
        //     Path::new("res\\mipmap\\test.aab"),
        //     "devtools".to_string(),
        // )
        // .unwrap();
        // android::verify_aab(Path::new("res\\mipmap\\test.aab")).unwrap();
    }
}
