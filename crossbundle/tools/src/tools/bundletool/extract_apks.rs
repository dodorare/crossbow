use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// To measure the estimated download sizes of APKs in an APK set as they would be served
/// compressed over-the-wire, use the get-size total command: ```
/// bundletool get-size total --apks=/MyApp/my_app.apks ```
/// You can modify the behavior of the get-size total command using the following flags:
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
