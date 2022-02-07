use clap::Parser;
use crossbundle_tools::utils::Config;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, Default)]
pub struct AndroidInstallCommand {
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

impl AndroidInstallCommand {
    /// Download and install bundletool to provided or default path
    pub fn install(&self, _config: &Config) -> crate::error::Result<()> {
        let download_url = format!(
            "https://github.com/google/bundletool/releases/download/{}/{}",
            self.version,
            self.file_name()
        );
        if let Some(install_path) = &self.path {
            let jar_path = install_path.join(self.file_name());
            create_jar_file(download_url, jar_path)?;
        } else {
            let default_jar_path = dirs::home_dir()
                .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
                .join(self.file_name());
            create_jar_file(download_url, default_jar_path)?;
        };
        Ok(())
    }

    /// Return bundletool jar file name with specified version
    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }
}

/// Download jar file and save it in directory
fn create_jar_file(download_url: String, jar_path: PathBuf) -> crate::error::Result<()> {
    let response = ureq::get(&download_url)
        .call()
        .map_err(crate::error::Error::DownloadFailed)?;

    let mut out = std::fs::File::create(&jar_path).map_err(|cause| {
        crate::error::Error::JarFileCreationFailed {
            path: jar_path.clone(),
            cause,
        }
    })?;
    std::io::copy(&mut response.into_reader(), &mut out).map_err(|cause| {
        crate::error::Error::CopyToFileFailed {
            path: jar_path,
            cause,
        }
    })?;
    Ok(())
}
