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

const SDKMANAGER_DOWNLOAD_URL: &'static str = "https://dl.google.com/android/repository/";

#[derive(Parser, Clone, Debug, Default)]
pub struct SdkManagerInstallCommand {
    #[clap(long, short)]
    install_path: Option<PathBuf>,
}

impl SdkManagerInstallCommand {
    /// Download command line tools zip archive and extract it in specified sdk root directory
    pub fn install(&self, _config: &Config) -> crate::error::Result<()> {
        let sdkmanager_download_url = SDKMANAGER_DOWNLOAD_URL
            .parse::<PathBuf>()
            .ok()
            .unwrap()
            .join(format!("{}", self.sdk_file_name()));

        let file_path = Self::default_file_path(&self)?;
        let sdk_root = Self::set_sdk_root(&self)?;

        Self::create_file(&self, sdkmanager_download_url, &file_path)?;

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
            .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
            .join(self.sdk_file_name());
        Ok(default_file_path)
    }

    /// Set sdk root for sdkmanager storing
    pub fn set_sdk_root(&self) -> crate::error::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        let root = Path::new("AppData")
            .join("Local")
            .join("Android")
            .join("Sdk");

        #[cfg(not(target_os = "windows"))]
        let root = Path::new("Android").join("Sdk");

        if !root.exists() {
            std::fs::create_dir_all(&root)?
        }
        let sdk_root = Self::default_file_path(&self)?.parent().unwrap().join(root);
        Ok(sdk_root)
    }

    /// Check home directory for zip file. If it doesn't exists download zip file and save it in the directory
    pub fn create_file(&self, download_url: PathBuf, file_path: &Path) -> crate::error::Result<()> {
        // TODO: check the archive for corruption
        for sdkmanager in std::fs::read_dir(file_path.parent().unwrap())? {
            let zip_path = sdkmanager?.path();
            if zip_path.ends_with(Self::sdk_file_name(&self)) {
                return Ok(());
            }
        }
        let url = download_url.to_str().unwrap();
        let response = ureq::get(url)
            .call()
            .map_err(crate::error::Error::DownloadFailed)?;

        let mut out = std::fs::File::create(&file_path).map_err(|cause| {
            crate::error::Error::JarFileCreationFailed {
                path: file_path.clone().to_path_buf(),
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
}
