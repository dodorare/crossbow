use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Bundletool is capable of generating an APK set that targets a device configuration 
/// specified by a JSON file. To first generate a JSON file for a connected device, run 
/// the following command:
#[derive(Debug, PartialEq, PartialOrd)]
pub struct GetDeviceSpec{
    output: PathBuf,
}
impl GetDeviceSpec {
    pub fn new(output: &Path) -> Self {
        Self {
            output: output.to_owned(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let mut get_device_spec = Command::new("java");
        get_device_spec.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            get_device_spec.arg(bundletool_path);
        } else {
            return Err(AndroidError::BundletoolNotFound.into());
        }
        get_device_spec.arg("get-device-spec");
        get_device_spec.arg("--output").arg(&self.output);
        get_device_spec.output_err(true)?;
        Ok(())
    }
}
