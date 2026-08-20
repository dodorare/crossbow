use super::*;
use crate::error::Result;
use clap::Parser;
use crossbundle_tools::{
    commands::android::*,
    types::{CliContext, android_sdk_path},
};
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
const OS_TAG: &str = "win";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const OS_TAG: &str = "mac_arm64";
#[cfg(all(target_os = "macos", not(target_arch = "aarch64")))]
const OS_TAG: &str = "mac_x86_64";
#[cfg(target_os = "linux")]
const OS_TAG: &str = "linux";

const COMMAND_LINE_TOOLS_DOWNLOAD_URL: &str = "https://dl.google.com/android/repository/";

#[derive(Parser, Clone, Debug, Default)]
pub struct CommandLineToolsInstallCommand {
    /// Assign path to install command line tools
    #[clap(long, short)]
    pub install_path: Option<PathBuf>,
    /// Force install command line tools even if found or corrupted.
    #[clap(long, short)]
    pub force: bool,
}

impl CommandLineToolsInstallCommand {
    /// Download command line tools zip archive and extract it in specified sdk root
    /// directory
    pub fn install(&self, config: &CliContext) -> Result<()> {
        let sdk_path = self
            .install_path
            .clone()
            .map_or_else(android_sdk_path, Ok)?;
        let cmdline_tools_path = sdk_path.join("cmdline-tools");
        if cmdline_tools_path.exists() {
            return Ok(());
        }
        if self.force {
            remove(vec![default_file_path(self.file_name())?])?;
        }

        let download_url = format!("{COMMAND_LINE_TOOLS_DOWNLOAD_URL}{}", self.file_name());
        let file_path = default_file_path(self.file_name())?;
        let parent = file_path
            .parent()
            .ok_or_else(|| Error::PathNotFound(file_path.clone()))?;

        config.status_message(
            format!("Downloading {} into", self.file_name()),
            parent.to_string_lossy(),
        )?;
        self.download_and_save_file(&download_url, &file_path)?;

        config.status_message(
            "Extracting zip archive contents into",
            sdk_path.to_string_lossy(),
        )?;
        extract_archive(&file_path, Path::new(&sdk_path))?;

        config.status("Deleting zip archive that was left after installation")?;
        remove(vec![file_path])?;
        Ok(())
    }

    /// Return command line tools zip archive for defined operating system
    fn file_name(&self) -> String {
        format!("commandlinetools-{}-15859902_latest.zip", OS_TAG)
    }

    /// Check home directory for zip file. If it doesn't exists download zip file and save
    /// it in the directory
    pub fn download_and_save_file(&self, download_url: &str, file_path: &Path) -> Result<()> {
        remove(vec![file_path.to_path_buf()])?;
        let parent = file_path
            .parent()
            .ok_or_else(|| Error::PathNotFound(file_path.to_owned()))?;
        for dir in std::fs::read_dir(parent)? {
            let zip_path = dir?.path();
            if zip_path.ends_with(self.file_name()) {
                return Ok(());
            }
        }
        download_to_file(download_url, file_path)?;
        Ok(())
    }
}
