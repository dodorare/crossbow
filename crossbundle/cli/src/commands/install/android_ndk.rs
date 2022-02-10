use crate::error::*;
use clap::Parser;
use crossbundle_tools::error::CommandExt;

use super::sdkmanager::SdkManagerInstallCommand;

#[derive(Parser, Clone, Debug, Default)]
pub struct NdkInstallCommand {
    /// List installed and available packages. Use the channel option to include a package from a channel up to and including channel_id.
    /// For example, specify the canary channel to list packages from all channels.
    #[clap(long)]
    list: bool,
    /// Specify needed version for install NDK. By defaut, ndk/22.0.7026061 will be installed
    #[clap(long)]
    ndk: Option<String>,
    /// Specify version Android NDK that need to be uninstalled
    #[clap(long)]
    uninstall: Option<String>,
    /// Update all installed packages
    #[clap(long)]
    update: bool,
    /// Install all required tools for correct crossbundle work
    #[clap(long)]
    required_tools: bool,
}

impl NdkInstallCommand {
    pub fn install(&self) -> Result<()> {
        let sdkmanager_dir = SdkManagerInstallCommand::set_sdk_root(&SdkManagerInstallCommand {
            install_path: None,
        })?
        .join("cmdline-tools")
        .join("bin");

        #[cfg(target_os = "windows")]
        let sdkmanager = sdkmanager_dir.to_string_lossy().replace("\\", "/");
        #[cfg(not(target_os = "windows"))]
        let sdkmanager = sdkmanager_dir;

        let sdk_root = sdkmanager_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap();

        let sdkmanager_bat = format!("{}/sdkmanager.bat", sdkmanager);
        let mut sdkmanager = std::process::Command::new(sdkmanager_bat);
        sdkmanager.arg(format!("--sdk_root={}", sdk_root));
        if let Some(ndk) = &self.ndk {
            sdkmanager.arg(format!("ndk;{}", ndk));
        }
        if let Some(uninstall) = &self.uninstall {
            sdkmanager.arg("--uninstall").arg(uninstall);
        }
        if self.update {
            sdkmanager.arg("--update");
        }
        if self.list {
            sdkmanager.arg("--list");
        }
        if self.required_tools {
            sdkmanager
                .arg("build-tools;30.0.0")
                .arg("cmdline-tools;latest")
                .arg("extras;google;usb_driver")
                .arg("extras;google;Android_Emulator_Hypervisor_Driver")
                .arg("ndk;22.0.7026061")
                .arg("platforms;android-30")
                .arg("sources;android-30");
        }
        sdkmanager.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn download_test() {
        NdkInstallCommand::install(&NdkInstallCommand {
            ndk: None,
            uninstall: None,
            required_tools: true,
            list: false,
            update: false,
        })
        .unwrap();
    }
}
