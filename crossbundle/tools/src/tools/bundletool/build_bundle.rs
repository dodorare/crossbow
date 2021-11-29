use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Build your app bundle using bundletool
/// To build your app bundle, you use the `bundletool build-bundle` command, as shown below:
///
/// ```xml
/// bundletool build-bundle --modules=base.zip --output=mybundle.aab
/// ```
///
/// ## Note
/// If you plan to publish the app bundle, you need to sign it using [`jarsigner`]. You can
/// not use apksigner to sign your app bundle.
/// 
/// [`jarsigner`]::https://docs.oracle.com/javase/8/docs/technotes/tools/windows/jarsigner.html
#[derive(Debug, PartialEq, PartialOrd)]
pub struct BuildBundle {
    modules: Vec<PathBuf>,
    output: PathBuf,
    config: Option<PathBuf>,
    metadata_file: Option<PathBuf>,
}

impl BuildBundle {
    /// Specifies the list of module ZIP files `bundletool` should use to build your app
    /// bundle.
    ///
    /// Specifies the path and filename for the output `*.aab` file.
    pub fn new(modules: &[PathBuf], output: &Path) -> Self {
        Self {
            modules: modules.to_vec(),
            output: output.to_owned(),
            config: None,
            metadata_file: None,
        }
    }

    /// Specifies the path to an optional configuration file you can use to customize the
    /// build process. To learn more, see the section about [`customizing downstream APK
    /// generation`].
    ///
    /// [`customizing downstream APK generation`]::https://developer.android.com/studio/build/building-cmdline#bundleconfig
    pub fn config(&mut self, config: &Path) -> &mut Self {
        self.config = Some(config.to_owned());
        self
    }

    /// Instructs bundletool to package an optional metadata file inside your app bundle.
    /// You can use this file to include data, such as ProGuard mappings or the complete
    /// list of your app's DEX files, that may be useful to other steps in your toolchain
    /// or an app store.
    ///
    /// `target-bundle-path` specifies a path relative to the root of the app bundle where
    /// you would like the metadata file to be packaged, and `local-file-path` specifies the
    /// path to the local metadata file itself.
    pub fn metadata_file(&mut self, metadata_file: &Path) -> &mut Self {
        self.metadata_file = Some(metadata_file.to_owned());
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut build_bundle = Command::new("java");
        build_bundle.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            build_bundle.arg(bundletool_path);
        } else {
            return Err(AndroidError::BundletoolNotFound.into());
        }
        build_bundle.arg("build-bundle");
        build_bundle.arg("--modules");
        build_bundle.arg(
            self.modules
                .iter()
                .map(|v| v.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        build_bundle.arg("--output").arg(&self.output);
        if let Some(config) = &self.config {
            build_bundle.arg("--config").arg(config);
        }
        if let Some(metadata_file) = &self.metadata_file {
            build_bundle.arg("--metadata-file").arg(metadata_file);
        }
        build_bundle.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{commands::android::{self, AabKey}, tools::{Aapt2, Aapt2Compile, Aapt2Link, AndroidSdk, BuildBundle}};

    use super::*;

    #[test]
    fn build_bundle_test() {
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
           let aab = dir.join(format!("{}_unsigned.aab", "test"));
            BuildBundle::new(&[gen_zip_modules], &aab).run().unwrap();
        }
}

