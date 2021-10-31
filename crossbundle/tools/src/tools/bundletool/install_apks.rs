use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Deploy APKs to a connected device
/// After you generate a set of APKs, bundletool can deploy the right combination of APKs
/// from that set to a connected device For example, if you have a connected device
/// running Android 5.0 (API level 21) or higher, bundletool pushes the base APK,
/// feature module APKs, and configuration APKs required to run your app on that device.
/// Alternatively, if your connected device is running Android 4.4 (API level 20) or
/// lower, bundletool looks for a compatible multi-APK and deploys it to your device.
/// To deploy your app from an APK set, use the install-apks command and specify the path
/// of the APK set using the --apks=/path/to/apks flag, as shown below. (If you have
/// multiple devices connected, specify a target device by adding the
/// --device-id=serial-id flag.)
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

    /// If you're using the --local-testing flag with the build-apks command, for local
    /// testing to work correctly, you need to use install-apks to install your APKs
    pub fn local_testing(&mut self, local_testing: bool) -> &mut Self {
        self.local_testing = local_testing;
        self
    }

    /// If you have multiple devices connected, specify a target device by adding the
    /// --device-id=serial-id flag
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

    use super::*;
    #[test]
    fn new() {
        InstallApks::new(Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\threed.apks")).run().unwrap();
    }
}
