use super::create_file;
use clap::Parser;
use crossbundle_tools::utils::Config;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, Default)]
pub struct BundletoolInstallCommand {
    /// Required. Version of download bundletool. For example:
    /// ```sh
    /// --version 1.8.2
    /// ```
    #[clap(long, short)]
    version: String,
    /// Path to install bundletool. By default bundletool will be downloaded and saved in home directory
    #[clap(long, short)]
    path: Option<PathBuf>,
}

impl BundletoolInstallCommand {
    /// Download and install bundletool to provided or default path
    pub fn install(&self, _config: &Config) -> crate::error::Result<()> {
        let download_url = format!(
            "https://github.com/google/bundletool/releases/download/{}/{}",
            self.version,
            self.file_name()
        );
        if let Some(install_path) = &self.path {
            let jar_path = install_path.join(self.file_name());
            create_file(download_url, jar_path)?;
        } else {
            let default_jar_path = dirs::home_dir()
                .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
                .join(self.file_name());
            create_file(download_url, default_jar_path)?;
        };
        Ok(())
    }

    /// Return bundletool jar file name with specified version
    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }
}
