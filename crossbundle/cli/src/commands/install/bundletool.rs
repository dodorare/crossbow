use clap::Parser;
use crossbundle_tools::utils::Config;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, Default)]
pub struct AndroidInstallCommand {
    #[clap(long, short)]
    version: String,
    // TODO: impl install_path flag
}

impl AndroidInstallCommand {
    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }

    pub fn install(&self, _config: &Config) -> crate::error::Result<()> {
        let default_jar_path = dirs::home_dir()
            .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
            .join(self.file_name());

        let download_url = format!(
            "https://github.com/google/bundletool/releases/download/{}/{}",
            self.version,
            self.file_name()
        );

        if !default_jar_path.exists() {
            let response = ureq::get(&download_url)
                .call()
                .map_err(crate::error::Error::DownloadFailed)?;

            let mut out = std::fs::File::create(&default_jar_path).map_err(|cause| {
                crate::error::Error::JarFileCreationFailed {
                    path: default_jar_path.clone(),
                    cause,
                }
            })?;
            std::io::copy(&mut response.into_reader(), &mut out).map_err(|cause| {
                crate::error::Error::CopyToFileFailed {
                    path: default_jar_path,
                    cause,
                }
            })?;
        }
        Ok(())
    }
}
