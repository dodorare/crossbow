// use clap::Parser;
// use crossbundle_tools::utils::Config;
// use std::{path::PathBuf, process::Command};

// #[cfg(target_os = "windows")]
// const OS_TAG: &str = "win";

// #[cfg(target_os = "macos")]
// const OS_TAG: &str = "mac";

// #[cfg(target_os = "linux")]
// const OS_TAG: &str = "linux";

// #[derive(Parser, Clone, Debug, Default)]
// pub struct NdkInstallCommand {
//     #[clap(long, short)]
//     download_sdkmanager: bool,
// }

// impl NdkInstallCommand {
//     pub fn download_cmd_line_tools(&self, _config: &Config) -> crate::error::Result<()> {
//         let download_url = format!(
//             "https://dl.google.com/android/repository/{}",
//             self.file_name()
//         );

//         let default_jar_path = dirs::home_dir()
//             .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
//             .join(self.file_name());
//         create_zip_file(download_url, Self::default_file_path(&self, false)?)?;

//         Ok(())
//     }

//     pub fn install_tools(&self, config: &Config) -> crate::error::Result<()> {
//         if self.download_sdkmanager {
//             self.download_cmd_line_tools(config)?;
//         }
//         let set_sdk_path = Command::new("sdkmanager").arg(format!(
//             "sdk_root={:?}",
//             Self::default_file_path(&self, false)?
//         ));
//         Ok(())
//     }

//     fn file_name(&self) -> String {
//         format!("commandlinetools-{}-8092744_latest.zip", OS_TAG)
//     }

//     pub fn default_file_path(&self, sdk_root: bool) -> crate::error::Result<PathBuf> {
//         let default_file_path = dirs::home_dir()
//             .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
//             .join(self.file_name());
//         if sdk_root {
//             default_file_path
//         }
//         Ok(default_file_path)
//     }
// }

// /// Download jar file and save it in directory
// pub fn create_zip_file(
//     download_url: String,
//     jar_path: std::path::PathBuf,
// ) -> crate::error::Result<()> {
//     let response = ureq::get(&download_url)
//         .call()
//         .map_err(crate::error::Error::DownloadFailed)?;

//     let mut out = std::fs::File::create(&jar_path).map_err(|cause| {
//         crate::error::Error::JarFileCreationFailed {
//             path: jar_path.clone(),
//             cause,
//         }
//     })?;
//     std::io::copy(&mut response.into_reader(), &mut out).map_err(|cause| {
//         crate::error::Error::CopyToFileFailed {
//             path: jar_path,
//             cause,
//         }
//     })?;
//     Ok(())
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     #[test]
// //     fn download_test() {
// //         NdkInstallCommand::install(&NdkInstallCommand {}).unwrap();
// //     }
// // }
