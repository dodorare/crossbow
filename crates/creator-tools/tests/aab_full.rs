use creator_tools::{
    commands::{android, gen_minimal_project},
    tools::{AndroidNdk, AndroidSdk},
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
        let compiled_lib = build_dir.join(format!("lib{}.so", package_name));
        assert!(compiled_lib.exists());

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
            &[Path::new("creator\\crates\\creator-tools\\res\\mipmap").to_owned()],
            None,
            &build_dir,
            &sdk,
            &package_name,
            &manifest_path,
            target_sdk_version,
        )
        .unwrap();

        // Assign path to lib
        // android::add_lib_aapt2(Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\lib\\"),
        // Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\extracted_files\\")).unwrap();

        // android::add_libs_into_aapt2(
        //     &ndk,
        //     &compiled_lib,
        //     build_target,
        //     profile,
        //     30,
        //     &aab_build_dir,
        //     &target_dir,
        // )
        // .unwrap();

        // let base_aab_module = android::gen_base_aab_module().unwrap();

        // Gen aab from given list of modules (zip, zip, zip)
        // let aab_path = android::gen_aab_from_modules(&[base_aab_module], &aab_build_dir).unwrap();

        // Create keystore with keytool command
        android::gen_debug_key_aab(Path::new("res\\mipmap\\"), "devtools".to_string()).unwrap();
        android::jarsigner(
            Path::new("res\\mipmap\\keystore"),
            Path::new("res\\mipmap\\test.aab"),
            "devtools".to_string(),
        )
        .unwrap();
        android::verify_aab(Path::new("res\\mipmap\\test.aab")).unwrap();
    }
}
