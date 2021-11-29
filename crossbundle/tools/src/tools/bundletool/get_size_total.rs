use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Measure the estimated download sizes of APKs in an APK set
/// 
/// To measure the estimated download sizes of APKs in an APK set as they would be served
/// compressed over-the-wire, use the `get-size total` command:
///
/// ```xml
/// bundletool get-size total --apks=/MyApp/my_app.apks
/// ```
///
/// You can modify the behavior of the `get-size total` command using the following flags:
pub struct GetSizeTotal {
    apks: PathBuf,
    device_spec: Option<PathBuf>,
    dimensions: Option<String>,
    instant: bool,
    modules: Option<String>,
}

impl GetSizeTotal {
    /// (Required) Specifies the path to the existing APK set file whose download size is
    /// measured.
    pub fn new(apks: &Path) -> Self {
        Self {
            apks: apks.to_owned(),
            device_spec: None,
            dimensions: None,
            instant: false,
            modules: None,
        }
    }

    /// Specifies the path to the device spec file (from get-device-spec or constructed
    /// manually) to use for matching. You can specify a partial path to evaluate a set of
    /// configurations.
    pub fn device_spec(&mut self, device_spec: &Path) -> &mut Self {
        self.device_spec = Some(device_spec.to_owned());
        self
    }

    /// Specifies the dimensions used when computing the size estimates.
    /// Accepts a comma-separated list of: `SDK`, `ABI`, `SCREEN_DENSITY`, and `LANGUAGE`. To
    /// measure across all dimensions, specify `ALL`.
    pub fn dimensions(&mut self, dimensions: String) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Measures the download size of the instant-enabled APKs instead of the installable
    /// APKs. By default, `bundletool` measures the installable APK download sizes
    pub fn instant(&mut self, instant: bool) -> &mut Self {
        self.instant = instant;
        self
    }

    /// Specifies a comma-separated list of modules in the APK set to consider in the
    /// measurement. The `bundletool` command automatically includes any dependent modules
    /// for the specified set. By default, the command measures the download size of
    /// all modules installed during the first download.
    pub fn modules(&mut self, modules: String) -> &mut Self {
        self.modules = Some(modules);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut get_size_total = Command::new("java");
        get_size_total.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            get_size_total.arg(bundletool_path);
        } else {
            return Err(AndroidError::BundletoolNotFound.into());
        }
        get_size_total.arg("get-size");
        get_size_total.arg("total");
        get_size_total.arg("--apks");
        get_size_total.arg(&self.apks);
        if let Some(device_spec) = &self.device_spec {
            get_size_total.arg("--device-spec").arg(device_spec);
        }
        if let Some(dimensions) = &self.dimensions {
            get_size_total.arg("--dimensions").arg(dimensions);
        }
        if self.instant {
            get_size_total.arg("--instant");
        }
        if let Some(modules) = &self.modules {
            get_size_total.arg("--modules").arg(modules);
        }
        get_size_total.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{commands::android::{self, AabKey}, tools::{Aapt2, Aapt2Link, AndroidSdk}};

    use super::*;

    #[test]
    fn get_size_total_test() {
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
           let gen_zip_modules = android::gen_zip_modules(&dir, "test", &extracted_apk_path).unwrap();
           let aab_path = android::gen_aab_from_modules(
            "test",
            &[gen_zip_modules],
            &compiled_res_dir,
        ).unwrap();

        let sign_key_path = Some(dir.join("aab.keystore"));
        let sign_key_pass = Some("android");
        let sign_key_alias = Some("androiddebugkey");
        let key = if let Some(key_path) = sign_key_path {
            let aab_key = AabKey {
                key_path,
                key_pass: sign_key_pass.clone().unwrap().to_string(),
                key_alias: sign_key_alias.clone().unwrap().to_string(),
            };
            if aab_key.key_path.exists() {
                aab_key
            } else {
                android::gen_aab_key(aab_key).unwrap()
            }
        } else {
            let aab_key: AabKey = Default::default();
            if aab_key.key_path.exists() {
                aab_key
            } else {
                android::gen_aab_key(aab_key).unwrap()
            }
        };
    let apks = compiled_res_dir.join(format!("{}.apks", "test"));
    // Test build_apks
    let build_apks = android::build_apks(&aab_path, &apks, key).unwrap();
    GetSizeTotal::new(&build_apks).run().unwrap();
}
}
