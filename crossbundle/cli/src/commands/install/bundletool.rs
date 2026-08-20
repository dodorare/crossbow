use super::*;
use crate::error::Result;
use clap::Parser;
use crossbundle_tools::types::CliContext;
use std::path::PathBuf;

const BUNDLETOOL_JAR_FILE_DOWNLOAD_URL: &str =
    "https://github.com/google/bundletool/releases/download";

#[derive(Parser, Clone, Debug, Default)]
pub struct BundletoolInstallCommand {
    /// Required. Version of download bundletool. For example:
    /// --version 1.18.3
    #[clap(long, short, default_value = "1.18.3")]
    pub version: String,
    /// Path to install bundletool. By default bundletool will be downloaded and saved in
    /// home directory
    #[clap(long, short)]
    pub path: Option<PathBuf>,
    /// Force install bundletool even if found.
    #[clap(long, short)]
    pub force: bool,
}

impl BundletoolInstallCommand {
    /// Download and install bundletool to provided or default path
    pub fn install(&self, config: &CliContext) -> Result<()> {
        let home_dir = home::home_dir().ok_or(Error::HomeDirNotFound)?;
        if !self.force {
            for bundletool in std::fs::read_dir(&home_dir)? {
                let installed_bundletool = bundletool?.path();
                if installed_bundletool.ends_with(self.file_name()) {
                    config.status("You have installed bundletool on your system already. Use `--force` command to overwrite.")?;
                    return Ok(());
                }
            }
        }
        let download_url = format!(
            "{BUNDLETOOL_JAR_FILE_DOWNLOAD_URL}/{}/{}",
            self.version,
            self.file_name()
        );

        if let Some(install_path) = &self.path {
            config.status_message(
                format!("{} installing into", self.file_name()),
                install_path.to_string_lossy(),
            )?;
            let jar_path = install_path.join(self.file_name());
            download_to_file(&download_url, &jar_path)?;
        } else {
            config.status_message(
                format!("{} installing into", self.file_name()),
                home_dir.to_string_lossy(),
            )?;
            let default_jar_path = default_file_path(self.file_name())?;
            download_to_file(&download_url, &default_jar_path)?;
        };
        config.status("Bundletool was installed successfully")?;
        Ok(())
    }

    /// Return bundletool jar file name with specified version
    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }
}
