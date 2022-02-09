pub mod bundletool;
pub mod ndk;
pub mod sdkmanager;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::utils::Config;

use self::{bundletool::BundletoolInstallCommand, sdkmanager::SdkManagerInstallCommand};

#[derive(Parser, Clone, Debug)]
pub enum InstallCommand {
    /// Install bundletool. You can specify version of bundletool. By default, we have 1.8.2 bundletool version in usage
    Bundletool(BundletoolInstallCommand),
    /// Install Android Studio's command line tools including sdkmanager command line tool inside.
    /// Sdkmanager allows to install Android SDK and Android NDK.
    SdkManager(SdkManagerInstallCommand),
}

impl InstallCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            InstallCommand::Bundletool(cmd) => cmd.install(config),
            InstallCommand::SdkManager(cmd) => cmd.install(config),
        }
    }
}

/// Download jar file and save it in directory
pub fn create_file(
    download_url: String,
    file_path: std::path::PathBuf,
) -> crate::error::Result<()> {
    let response = ureq::get(&download_url)
        .call()
        .map_err(crate::error::Error::DownloadFailed)?;

    let mut out = std::fs::File::create(&file_path).map_err(|cause| {
        crate::error::Error::JarFileCreationFailed {
            path: file_path.clone(),
            cause,
        }
    })?;
    std::io::copy(&mut response.into_reader(), &mut out).map_err(|cause| {
        crate::error::Error::CopyToFileFailed {
            path: file_path,
            cause,
        }
    })?;
    Ok(())
}
