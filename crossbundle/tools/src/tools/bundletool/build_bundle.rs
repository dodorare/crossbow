use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Build your app bundle using bundletool
/// To build your app bundle, you use the `bundletool build-bundle` command, as shown
/// below:
///
/// ```xml
/// bundletool build-bundle --modules=base.zip --output=mybundle.aab
/// ```
///
/// ## Note
/// If you plan to publish the app bundle, you need to sign it using [`jarsigner`]. You
/// can not use apksigner to sign your app bundle.
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
    /// you would like the metadata file to be packaged, and `local-file-path` specifies
    /// the path to the local metadata file itself.
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

    use crate::{
        commands::android::{
            self, android_dir, gen_aab_key, gen_minimal_unsigned_aab, jarsigner, remove, AabKey,
        },
        tools::{AndroidSdk, BuildBundle},
    };

    #[test]
    fn build_bundle_test() {
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
                let extracted_files = android::extract_apk(&apk_path, &output_dir).unwrap();
                // Generates zip archive from extracted files
                let gen_zip_modules =
                    android::gen_zip_modules(&build_dir, "test", &extracted_files).unwrap();
                let aab = build_dir.join(format!("{}_unsigned.aab", package_name));
                // Builds app bundle
                BuildBundle::new(&[gen_zip_modules], &aab).run().unwrap();
            }
        }
    }
}
