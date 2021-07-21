use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// To measure the estimated download sizes of APKs in an APK set as they would be served
/// compressed over-the-wire, use the get-size total command: ```
/// bundletool get-size total --apks=/MyApp/my_app.apks
/// ```
/// You can modify the behavior of the get-size total command using the following flags:
pub struct ExtractApks {
    /// (Required)
    apks: PathBuf,
    /// Specifies the path to the device spec file (from get-device-spec or constructed
    /// manually) to use for matching.
    device_spec: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    /// Specifies the path for bundletool
    bundletool_path: Option<PathBuf>,
}

impl ExtractApks {
    pub fn new(apks: &Path) -> Self {
        Self {
            apks: apks.to_owned(),
            device_spec: None,
            output_dir: None,
            bundletool_path: None,
        }
    }

    pub fn device_spec(&mut self, device_spec: &Path) -> &mut Self {
        self.device_spec = Some(device_spec.to_owned());
        self
    }

    pub fn output_dir(&mut self, output_dir: &Path) -> &mut Self {
        self.output_dir = Some(output_dir.to_owned());
        self
    }

    pub fn bundletool_path(&mut self, bundletool_path: &Path) -> &mut Self {
        self.bundletool_path = Some(bundletool_path.to_owned());
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut extract_apks = Command::new("extract-apks");
        extract_apks.arg("--apks=");
        extract_apks.arg(&self.apks);
        if let Some(device_spec) = &self.device_spec {
            extract_apks.arg("--device-spec=").arg(device_spec);
        }
        if let Some(output_dir) = &self.output_dir {
            extract_apks.arg("--dimensions=").arg(output_dir);
        }
        extract_apks.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]

    fn extract_apks_test() {
        let _extract_apks = ExtractApks::new(&Path::new("res\\mipmap\\")).run();
    }
}
