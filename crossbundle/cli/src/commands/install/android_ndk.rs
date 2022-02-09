use crate::error::*;
use clap::Parser;
use crossbundle_tools::{error::CommandExt, utils::Config};
use std::{path::PathBuf, process::Command};

use super::sdkmanager::SdkManagerInstallCommand;

#[derive(Parser, Clone, Debug, Default)]
pub struct NdkInstallCommand {
    /// Specify needed version for install NDK. By defaut, ndk/22.0.7026061 will be installed
    #[clap(long)]
    ndk: Option<String>,
    /// Specify version Android NDK that need to be uninstalled
    #[clap(long)]
    uninstall: Option<String>,
    /// Install all required tools for correct crossbundle work
    #[clap(long)]
    default: bool,
}

impl NdkInstallCommand {
    pub fn install(&self) -> Result<()> {
        let sdk_root = SdkManagerInstallCommand::set_sdk_root(&SdkManagerInstallCommand {
            install_path: None,
        })?
        .join("cmdline-tools")
        .join("bin");
        println!("sdk_root: {:?}", sdk_root.parent().unwrap());
        let sdk = sdk_root.to_string_lossy().replace("\\", "/");

        let sdkmanager = format!("{}/sdkmanager.bat", sdk);
        let mut sdkmanager = std::process::Command::new(sdkmanager);
        if let Some(ndk) = &self.ndk {
            sdkmanager.arg(format!("ndk;{}", ndk));
        } else {
            sdkmanager.arg("ndk;22.0.7026061");
        }
        sdkmanager.arg(format!("--sdk_root={}", sdk));
        // if let Some(uninstall) = &self.uninstall {
        //     sdkmanager
        //         .arg("--uninstall")
        //         .arg(format!("ndk;{}", uninstall));
        // } else {
        //     sdkmanager.arg("--uninstall").arg("ndk;22.0.7026061");
        // }
        println!("sdkmanager: {:?}", sdkmanager);
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
            default: false,
        })
        .unwrap();
    }
}
