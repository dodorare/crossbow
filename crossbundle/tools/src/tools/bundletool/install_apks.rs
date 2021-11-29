use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Deploy APKs to a connected device
/// 
/// After you generate a set of APKs, bundletool can deploy the right combination of APKs
/// from that set to a connected device.
/// 
/// For example, if you have a connected device running Android 5.0 (API level 21) 
/// or higher, bundletool pushes the base APK, feature module APKs, and configuration
/// APKs required to run your app on that device. Alternatively, if your connected device
/// is running Android 4.4 (API level 20) or lower, `bundletool` looks for a compatible
/// multi-APK and deploys it to your device. 
/// 
/// To deploy your app from an APK set, use theinstall-apks command and specify the path 
/// of the APK set using the `--apks=/path/to/apks` flag, as shown below. (If you have 
/// multiple devices connected, specify a target device by adding the 
/// `--device-id=serial-id` flag.)
#[derive(Debug, Default)]
pub struct InstallApks {
    apks: PathBuf,
    local_testing: bool,
    device_id: Option<String>,
}

impl InstallApks {
    /// Specifies path to set of apks
    pub fn new(apks: &Path) -> Self {
        Self {
            apks: apks.to_owned(),
            ..Default::default()
        }
    }

    /// If you're using the `--local-testing` flag with the `build-apks` command, for local
    /// testing to work correctly, you need to use `install-apks` to install your APKs
    pub fn local_testing(&mut self, local_testing: bool) -> &mut Self {
        self.local_testing = local_testing;
        self
    }

    /// If you have multiple devices connected, specify a target device by adding the
    /// `--device-id=serial-id` flag
    pub fn device_id(&mut self, device_id: String) -> &mut Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut install_apks = Command::new("java");
        install_apks.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            install_apks.arg(bundletool_path);
        } else {
            return Err(AndroidError::BundletoolNotFound.into());
        }
        install_apks.arg("install-apks");
        install_apks.arg("--apks");
        install_apks.arg(&self.apks);
        if self.local_testing {
            install_apks.arg("--local-testing");
        }
        if let Some(device_id) = &self.device_id {
            install_apks.arg("--device-id").arg(device_id);
        }
        install_apks.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{commands::android::{self, AabKey}, tools::{Aapt2Link, AndroidSdk}};

    use super::*;

    #[test]
    fn new() {
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

      let sign_key_path = Some(compiled_res_dir.join("aab.keystore"));
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
  let build_apk = android::build_apks(&aab_path, &apks, key).unwrap();
  assert!(build_apk.exists());
    InstallApks::new(&apks).run().unwrap();
    
    }
}
