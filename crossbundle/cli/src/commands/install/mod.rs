pub mod bundletool;
pub mod command_line_tools;
pub mod sdkmanager;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::utils::Config;

use self::{
    bundletool::BundletoolInstallCommand, command_line_tools::CommandLineToolsInstallCommand,
    sdkmanager::SdkManagerInstallCommand,
};

#[derive(Parser, Clone, Debug)]
pub enum InstallCommand {
    /// Install bundletool. You can specify version of bundletool. By default, we have 1.8.2 bundletool version in usage
    Bundletool(BundletoolInstallCommand),
    /// Download the basic Android command line tools below. You can use the included sdkmanager to download other SDK packages.
    /// These tools are included in Android Studio
    CommandLineTools(CommandLineToolsInstallCommand),
    /// Allows you to view, install, update, and uninstall packages for the Android SDK
    SdkManager(SdkManagerInstallCommand),
}

impl InstallCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            InstallCommand::Bundletool(cmd) => cmd.install(config),
            InstallCommand::CommandLineTools(cmd) => cmd.install(config),
            InstallCommand::SdkManager(cmd) => cmd.install(config),
        }
    }
}

/// Download from url and saves it in specified file
pub fn download_to_file(
    download_url: &str,
    file_path: &std::path::Path,
) -> crate::error::Result<()> {
    let response = ureq::get(download_url)
        .call()
        .map_err(crate::error::Error::DownloadFailed)?;
    let mut out = std::fs::File::create(file_path).map_err(|cause| {
        crate::error::Error::JarFileCreationFailed {
            path: file_path.to_path_buf(),
            cause,
        }
    })?;
    std::io::copy(&mut response.into_reader(), &mut out).map_err(|cause| {
        crate::error::Error::CopyToFileFailed {
            path: file_path.to_path_buf(),
            cause,
        }
    })?;
    Ok(())
}

/// Using default file path related on $HOME path for all installed commands
pub fn default_file_path(file_name: String) -> crate::error::Result<std::path::PathBuf> {
    let default_file_path = dirs::home_dir()
        .ok_or_else(|| crate::error::Error::HomeDirNotFound)?
        .join(file_name);
    Ok(default_file_path)
}
