use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Extract device-specific APKs from an existing APK set
/// 
/// If you have an existing APK set and you want to extract from it a subset of APKs
/// that target a specific device configuration, you can use the `extract-apk` command
/// and specify a device specification JSON, as follows:
/// 
/// ```xml
/// bundletool extract-apks
/// --apks=/MyApp/my_existing_APK_set.apks
/// --output-dir=/MyApp/my_pixel2_APK_set.apks
/// --device-spec=/MyApp/bundletool/pixel2.json
/// ``` 
pub struct ExtractApks {
    apks: PathBuf,
    device_spec: PathBuf,
    output_dir: PathBuf,
}

impl ExtractApks {
    /// Specifies the path to the device spec file (from get-device-spec or constructed
    /// manually) to use for matching.
    pub fn new(apks: &Path, device_spec: &Path, output_dir: &Path) -> Self {
        Self {
            apks: apks.to_owned(),
            device_spec: device_spec.to_owned(),
            output_dir: output_dir.to_owned(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let mut extract_apks = Command::new("extract-apks");
        extract_apks.arg("--apks");
        extract_apks.arg(&self.apks);
        extract_apks.arg("--device-spec");
        extract_apks.arg(&self.device_spec);
        extract_apks.arg("--output-dir");
        extract_apks.arg(&self.output_dir);
        extract_apks.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use itertools::Itertools;

    use crate::{commands::android::{self, AabKey}, tools::{Aapt2, Aapt2Compile, Aapt2Link, AndroidSdk, BuildBundle}};

    use super::*;

    #[test]
    fn extract_apks_test() {
           // Creates a temporary directory and specify resources
            let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let dir = user_dirs.parent().unwrap().parent().unwrap().to_path_buf();
            let res_path = dir.join("examples\\bevy-3d\\assets\\models\\helmet");
            res_path.canonicalize().unwrap();
            let sdk = AndroidSdk::from_env().unwrap();
            let tempfile = tempfile::tempdir().unwrap();
            let compiled_res_dir = tempfile.path().to_path_buf();
            assert!(compiled_res_dir.exists());
 
            // Compiles resources for aapt2 link
            let compiled_res = sdk
             .aapt2()
             .unwrap()
             .compile_dir(&res_path, &compiled_res_dir)
             .run()
             .unwrap();
         assert!(compiled_res.exists());
         
            // Generates minimal android manifest
            let manifest = android::gen_minimal_android_manifest(
                None,
                "example",
                None,
                "0.0.1".to_string(),
                None,
                None,
                30,
                None,
                None,
                false,
            );
    
            // Saves android manifest into temporary directory
            let manifest_path = android::save_android_manifest(&compiled_res_dir, &manifest).unwrap();
            assert!(manifest_path.exists());
    
            // Generates apk file
            let sdk = AndroidSdk::from_env().unwrap();
            let target_sdk_version = 30;
            let apk_path = dir.join("test_apk");
            let mut aapt2_link = Aapt2Link::new(&[compiled_res], &apk_path, &manifest_path);
            aapt2_link
                .android_jar(sdk.android_jar(target_sdk_version).unwrap())
                .proto_format(true)
                .auto_add_overlay(true)
                .verbose(true)
                .version_code(1);
            aapt2_link.run().unwrap();

        let extracted_apk_path = android::extract_apk(&apk_path, &compiled_res_dir).unwrap();
        let filename = Path::new(&apk_path);
        let file = fs::File::open(&filename).unwrap();
        let mut apk = zip::ZipArchive::new(file).unwrap();
        apk.extract(dir.clone()).unwrap();
        let gen_zip_modules = android::gen_zip_modules(&dir.clone(), "test", &extracted_apk_path).unwrap();
        let aab = dir.join(format!("{}_unsigned.aab", "test"));
        BuildBundle::new(&[gen_zip_modules], &aab).run().unwrap();
        }
}

