use super::*;
use clap::Parser;
use crossbundle_tools::{
    commands::android::{self, remove},
    utils::Config,
};
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
const OS_TAG: &str = "win";

#[cfg(target_os = "macos")]
const OS_TAG: &str = "mac";

#[cfg(target_os = "linux")]
const OS_TAG: &str = "linux";

const COMMAND_LINE_TOOLS_DOWNLOAD_URL: &'static str = "https://dl.google.com/android/repository/";

#[derive(Parser, Clone, Debug, Default)]
pub struct CommandLineToolsInstallCommand {
    /// Assign path to install command line tools
    #[clap(long, short)]
    pub install_path: Option<PathBuf>,
}

impl CommandLineToolsInstallCommand {
    /// Download command line tools zip archive and extract it in specified sdk root directory
    pub fn install(&self, _config: &Config) -> crate::error::Result<()> {
        let command_line_tools_download_url = COMMAND_LINE_TOOLS_DOWNLOAD_URL
            .parse::<PathBuf>()
            .ok()
            .unwrap()
            .join(format!("{}", self.sdk_file_name()));

        let file_path = Self::default_file_path(&self)?;
        let sdk_root = Self::set_sdk_root(&self)?;

        Self::download_and_save_file(&self, command_line_tools_download_url, &file_path)?;

        if let Some(path) = &self.install_path {
            android::extract_archive(&file_path, path)?;
        } else {
            android::extract_archive(&file_path, &sdk_root)?;
        }

        remove(vec![file_path])?;
        Ok(())
    }

    /// Return command line tools zip archive for defined operating system
    fn sdk_file_name(&self) -> String {
        format!("commandlinetools-{}-8092744_latest.zip", OS_TAG)
    }

    /// Make default file path and return it
    pub fn default_file_path(&self) -> crate::error::Result<PathBuf> {
        let default_file_path = dirs::home_dir()
            .ok_or_else(|| crate::error::Error::HomeDirNotFound)?
            .join(self.sdk_file_name());
        Ok(default_file_path)
    }

    // TODO: Rethink this stuff
    /// Set sdk root for sdkmanager storing
    pub fn set_sdk_root(&self) -> crate::error::Result<PathBuf> {
        // TODO: Replace paths with $HOME based ones

        #[cfg(target_os = "windows")]
        let root = Path::new("AppData")
            .join("Local")
            .join("Android")
            .join("Sdk");

        #[cfg(not(target_os = "windows"))]
        let root = Path::new("Android").join("Sdk");

        let sdk_root = Self::default_file_path(&self)?.parent().unwrap().join(root);
        if !sdk_root.exists() {
            std::fs::create_dir_all(&sdk_root)?
        }
        Ok(sdk_root)
    }

    /// Check home directory for zip file. If it doesn't exists download zip file and save it in the directory
    pub fn download_and_save_file(
        &self,
        download_url: PathBuf,
        file_path: &Path,
    ) -> crate::error::Result<()> {
        for sdkmanager in std::fs::read_dir(file_path.parent().unwrap())? {
            let zip_path = sdkmanager?.path();
            if zip_path.ends_with(Self::sdk_file_name(&self)) {
                return Ok(());
            }
        }
        let url = download_url.to_str().unwrap();
        download_to_file(url, file_path)?;
        Ok(())
    }
}
