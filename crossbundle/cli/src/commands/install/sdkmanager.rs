use clap::Parser;
use crossbundle_tools::{error::CommandExt, tools::AndroidSdk, utils::Config};

#[derive(Parser, Clone, Debug, Default)]
pub struct SdkManagerInstallCommand {
    /// List installed and available packages. Use the channel option to include a package from a channel up to and including channel_id.
    /// For example, specify the canary channel to list packages from all channels.
    #[clap(long)]
    list: bool,
    /// Specify needed version for install NDK. By defaut, ndk/22.0.7026061 will be installed
    #[clap(long)]
    install: Option<String>,
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

impl SdkManagerInstallCommand {
    pub fn install(&self, _config: &Config) -> crate::error::Result<()> {
        // TODO: Fix logic. Perhaps replace with our AndroidSDK type
        let sdk = AndroidSdk::from_env()?;

        // TODO: Try to replace with library
        let sdkmanager = dunce::simplified(&sdk.sdk_path());

        let sdkmanager_bat = sdkmanager.join("sdkmanager.bat");
        let mut sdkmanager = std::process::Command::new(sdkmanager_bat);
        sdkmanager.arg(format!("--sdk_root={}", &sdk.sdk_path().to_str().unwrap()));
        if let Some(install) = &self.install {
            sdkmanager.arg(install);
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
                .arg("build-tools;29.0.0")
                .arg("ndk;22.0.7026061")
                .arg("platforms;android-30");
        }
        sdkmanager.output_err(true)?;
        Ok(())
    }
}
