use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Generate and use device specification JSON files
///
/// `Bundletool` is capable of generating an APK set that targets a device configuration
/// specified by a JSON file. To first generate a JSON file for a connected device, run
/// the following command:
///
/// ```xml
/// bundletool get-device-spec --output=/tmp/device-spec.json
/// ```
///
/// `bundletool` creates a JSON file for your device in the directory the tool is located.
/// You can then pass it to `bundletool` to generate a set of APKs that target only the
/// configuration described in that JSON file as follows:
///
/// ```xml
/// bundletool build-apks --device-spec=/MyApp/pixel2.json
/// --bundle=/MyApp/my_app.aab --output=/MyApp/my_app.apks
/// ```
#[derive(Debug, PartialEq, PartialOrd)]
pub struct GetDeviceSpec {
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
#[cfg(test)]
mod tests {

    use super::*;

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
