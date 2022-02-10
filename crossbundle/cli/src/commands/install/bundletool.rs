use super::*;
use clap::Parser;
use crossbundle_tools::utils::Config;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, Default)]
pub struct BundletoolInstallCommand {
    /// Required. Version of download bundletool. For example:
    /// ```sh
    /// --version 1.8.2
    /// ```
    #[clap(long, short, default_value = "1.8.2")]
    version: String,
    /// Path to install bundletool. By default bundletool will be downloaded and saved in home directory
    #[clap(long, short)]
    path: Option<PathBuf>,
    /// Force install bundletool even if found.
    #[clap(long, short)]
    force: bool,
}

impl BundletoolInstallCommand {
    /// Download and install bundletool to provided or default path
    pub fn install(&self, config: &Config) -> crate::error::Result<()> {
        // TODO: Add status messages
        config.status("Installing bundletool")?;
        // TODO: Check if bundletool is already installed
        // TODO: Add force installation
        let download_url = format!(
            "https://github.com/google/bundletool/releases/download/{}/{}",
            self.version,
            self.file_name()
        );
        if let Some(install_path) = &self.path {
            let jar_path = install_path.join(self.file_name());
            download_to_file(&download_url, &jar_path)?;
        } else {
            // TODO: Replace $HOME with $HOME/.crossbow path for all installed commands and tmp files
            let default_jar_path = dirs::home_dir()
                .ok_or_else(|| crate::error::Error::HomeDirNotFound)?
                .join(self.file_name());
            download_to_file(&download_url, &default_jar_path)?;
        };
        Ok(())
    }

    /// Return bundletool jar file name with specified version
    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }
}
