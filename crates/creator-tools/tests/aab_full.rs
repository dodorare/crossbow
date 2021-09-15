use creator_tools::{
    commands::{android, gen_minimal_project},
    tools::*,
    types::*,
};

#[cfg(test)]
mod tests {
    use super::*;
    use creator_tools::commands::android::gen_debug_key;

    #[test]
    fn test_aab() {
        // Create temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let project_path = tempdir.path();
        let package_name = gen_minimal_project(&project_path).unwrap();

        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        let target_sdk_version = 30;
        let profile = Profile::Debug;
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
        let target_dir = project_path.join("target");
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(profile.as_ref());
        let compiled_lib = out_dir.join(format!("lib{}.so", package_name));
        assert!(compiled_lib.exists());

        let android_build_dir = target_dir.join("android").join(profile.to_string());

        let android_abi = build_target.android_abi();
        let android_compiled_lib = android_build_dir
            .join("lib")
            .join(android_abi)
            .join(format!("lib{}.so", package_name));
        if !android_compiled_lib.exists() {
            std::fs::create_dir_all(&android_compiled_lib.parent().unwrap()).unwrap();
            fs_extra::file::copy(
                &compiled_lib,
                &android_compiled_lib,
                &fs_extra::file::CopyOptions::new(),
            )
            .unwrap();
        }

        // Generate manifest
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
        let aapt2_compile = Aapt2.compile_incremental(&res_path, &compiled_res_path);
        let compiled_res = aapt2_compile.run().unwrap();

        // Links all resources and creates .apk file
        let apk_path = android_build_dir.join(format!("{}_module.apk", package_name));
        let mut aapt2_link = Aapt2.link_compiled_res(Some(compiled_res), &apk_path, &manifest_path);
        aapt2_link
            .android_jar(sdk.android_jar(target_sdk_version).unwrap())
            .version_code(1)
            .proto_format(true)
            .auto_add_overlay(true);
        aapt2_link.run().unwrap();

        // Extracts files from .apk into /extracted_apk_files folder
        let output_dir = android_build_dir.join("extracted_apk_files");
        let extracted_apk_path = android::extract_apk(&apk_path, &output_dir).unwrap();

        let android_abi = build_target.android_abi();
        let android_compiled_lib = output_dir
            .join("lib")
            .join(android_abi)
            .join(format!("lib{}.so", package_name));
        if !android_compiled_lib.exists() {
            std::fs::create_dir_all(&android_compiled_lib.parent().unwrap()).unwrap();
            fs_extra::file::copy(
                &compiled_lib,
                &android_compiled_lib,
                &fs_extra::file::CopyOptions::new(),
            )
            .unwrap();
        }

        assert!(extracted_apk_path.exists());

        let gen_zip_modules =
            android::gen_zip_modules(&android_build_dir, &package_name, &extracted_apk_path)
                .unwrap();

        // Gen aab from given list of modules (zip, zip, zip)
        let aab_path =
            android::gen_aab_from_modules(&package_name, &[gen_zip_modules], &android_build_dir)
                .unwrap();
        for entry in std::fs::read_dir(&android_build_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.ends_with("extracted_apk_files") {
                std::fs::remove_dir_all(path.clone()).unwrap();
            }
            if path.ends_with("example_module.zip") {
                std::fs::remove_file(path).unwrap();
            }
        }

        // Create keystore with keytool command
        let key = gen_debug_key().unwrap();

        // Create keystore with keytool command
        let apks = android_build_dir.join(format!("{}.apks", package_name));
        let _build_apks = android::build_apks(&aab_path, &apks, key).unwrap();

        // println!("{}", project_path.to_string_lossy());
        std::thread::sleep(std::time::Duration::from_secs(60 * 20));
    }
}
