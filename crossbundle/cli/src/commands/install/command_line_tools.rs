use super::*;
use android_tools::sdk_install_path;
use clap::Parser;
use crossbundle_tools::{
    commands::android::{self, remove},
    utils::Config,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Clone, Debug, Default)]
pub struct CommandLineToolsInstallCommand {
    /// Assign path to install command line tools
    #[clap(long, short)]
    pub install_path: Option<PathBuf>,
    /// Remove corrupted zip archive if installation was aborted
    #[clap(long, short)]
    pub remove_zip: bool,
}

impl CommandLineToolsInstallCommand {
    /// Download command line tools zip archive and extract it in specified sdk root directory
    pub fn install(&self, config: &Config) -> crate::error::Result<()> {
        if self.remove_zip {
            remove(vec![default_file_path(self.file_name())?])?;
        }

        let command_line_tools_download_url = COMMAND_LINE_TOOLS_DOWNLOAD_URL
            .parse::<PathBuf>()
            .ok()
            .unwrap()
            .join(self.file_name());

        let file_path = default_file_path(self.file_name())?;

        config.status_message(
            "Downloading command line tools zip archive into",
            &file_path.parent().unwrap().to_str().unwrap(),
        )?;
        self.download_and_save_file(command_line_tools_download_url, &file_path)?;

        let sdk_path = sdk_install_path()?;

        if let Some(path) = &self.install_path {
            config.status_message(
                "Extracting zip archive contents into",
                path.to_str().unwrap(),
            )?;
            android::extract_archive(&file_path, path)?;
        } else {
            config.status_message(
                "Extracting zip archive contents into",
                &sdk_path.to_str().unwrap(),
            )?;
            android::extract_archive(&file_path, Path::new(&sdk_path))?;
        }

        config.status("Deleting zip archive was left after installation")?;
        remove(vec![file_path])?;
        Ok(())
    }

    /// Return command line tools zip archive for defined operating system
    fn file_name(&self) -> String {
        format!("commandlinetools-{}-8092744_latest.zip", OS_TAG)
    }

    /// Check home directory for zip file. If it doesn't exists download zip file and save it in the directory
    pub fn download_and_save_file(
        &self,
        download_url: PathBuf,
        file_path: &Path,
    ) -> crate::error::Result<()> {
        for sdkmanager in std::fs::read_dir(file_path.parent().unwrap())? {
            let zip_path = sdkmanager?.path();
            if zip_path.ends_with(self.file_name()) {
                return Ok(());
            }
        }
        let url = download_url.to_str().unwrap();
        download_to_file(url, file_path)?;
        Ok(())
    }
}
