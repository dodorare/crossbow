#[cfg(test)]
mod tests {
    use crossbundle_tools::{
        commands::android::{
            android_dir, extract_apk, gen_aab_key, gen_minimal_unsigned_aab, gen_zip_modules,
            jarsigner, remove, AabKey,
        },
        tools::{AndroidSdk, BuildApks, BuildBundle, GetSizeTotal},
    };

    #[test]
    fn test_build_apks() {
        // Creates a temporary directory
        let tempfile = tempfile::tempdir().unwrap();
        let build_dir = tempfile.path().to_path_buf();

        // Assigns configuratin to generate aab
        let sdk = AndroidSdk::from_env().unwrap();
        let package_name = "test";
        let target_sdk_version = 30;
        assert!(build_dir.exists());

        // Generates mininmal unsigned aab
        let aab_path =
            gen_minimal_unsigned_aab(sdk, "unsigned", target_sdk_version, &build_dir).unwrap();

        // Removes old keystore if it exists
        let android_dir = android_dir().unwrap();
        let target = vec![android_dir.join("aab.keystore")];
        remove(target).unwrap();

        // Creates new keystore to sign aab
        let aab_key = AabKey::default();
        let key_path = gen_aab_key(aab_key).unwrap();

        // Signs aab with key
        jarsigner(&aab_path, &key_path).unwrap();

        // Replace unsigned aab with signed aab
        let signed_aab = build_dir.join(format!("{}_signed.aab", package_name));
        std::fs::rename(&aab_path, &signed_aab).unwrap();

        // Test build_apks
        let apks_path = build_dir.join(format!("{}.apks", package_name));
        let apks = BuildApks::new(&signed_aab, &apks_path).run().unwrap();
        GetSizeTotal::new(&apks).run().unwrap();
    }

    #[test]
    fn build_bundle_test() {
        // Creates a temporary directory
        let tempfile = tempfile::tempdir().unwrap();
        let build_dir = tempfile.path().to_path_buf();
        assert!(build_dir.exists());

        // Assigns configuratin to generate aab
        let sdk = AndroidSdk::from_env().unwrap();
        let package_name = "test";
        let target_sdk_version = 30;

        // Generates mininmal unsigned aab
        let aab_path =
            gen_minimal_unsigned_aab(sdk, "unsigned", target_sdk_version, &build_dir).unwrap();
        assert!(aab_path.exists());

        // Removes old keystore if it exists
        let android_dir = android_dir().unwrap();
        let target = vec![android_dir.join("aab.keystore")];
        remove(target).unwrap();

        // Creates new keystore to sign aab
        let aab_key = AabKey::default();
        let key_path = gen_aab_key(aab_key).unwrap();

        // Signs aab with key
        let jarsigner = jarsigner(&aab_path, &key_path).unwrap();
        assert!(jarsigner.exists());

        // Replaces unsigned aab with signed aab
        let signed_aab = build_dir.join(format!("{}_signed.aab", package_name));
        std::fs::rename(&aab_path, &signed_aab).unwrap();

        // Defines apk path from build directory
        for apk in std::fs::read_dir(build_dir).unwrap() {
            let apk_path = apk.unwrap().path();
            if apk_path.ends_with("apk") {
                let build_dir = apk_path.parent().unwrap();
                let output_dir = build_dir.join("extracted_apk_files");

                // Extracts files from apk to defined path
                let extracted_files = extract_apk(&apk_path, &output_dir).unwrap();

                // Generates zip archive from extracted files
                let gen_zip_modules =
                    gen_zip_modules(&build_dir, "test", &extracted_files).unwrap();
                let aab = build_dir.join(format!("{}_unsigned.aab", package_name));

                // Builds app bundle
                BuildBundle::new(&[gen_zip_modules], &aab).run().unwrap();
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use crossbundle_tools::tools::GetDeviceSpec;

        #[test]
        fn build_apks_test() {
            // Creates a temporary directory
            let tempfile = tempfile::tempdir().unwrap();
            let build_dir = tempfile.path().to_path_buf();
            let package_name = "test";

            // Connect your device or emulator to generate device spec in `.json` format
            GetDeviceSpec::new(&build_dir.join(format!("{}.json", package_name)))
                .run()
                .unwrap();
        }
    }
}
