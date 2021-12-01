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
