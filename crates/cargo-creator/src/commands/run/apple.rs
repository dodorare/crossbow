use crate::commands::build::{apple::AppleBuildCommand, BuildContext};
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub struct AppleRunCommand {
    #[clap(flatten)]
    pub build_command: AppleBuildCommand,

    /// Install and launch on the simulator
    #[clap(short, long)]
    pub simulator: bool,
    /// Simulator device name
    #[clap(short, long, default_value = "iPhone 8")]
    pub device_name: String,
}

impl AppleRunCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let mut build_command = self.build_command.clone();
        if self.simulator {
            // TODO: Support apple silicon
            build_command.target = vec![AppleTarget::X86_64AppleIos];
        }
        let build_context =
            BuildContext::init(&current_dir, build_command.shared.target_dir.clone())?;
        let (metadata, app_paths) = build_command.execute(&build_context)?;
        log::info!("Starting run process");
        let bundle_id = &metadata.info_plist.identification.bundle_identifier;
        let app_path = self.get_app_path(app_paths)?;
        if self.simulator {
            log::info!("Installing and launching application on simulator");
            launch_apple_app(&app_path, &self.device_name, bundle_id, false)?;
            creator_tools::simctl::Simctl::new()
                .open()
                .map_err(|err| Error::CreatorTools(err.into()))?;
        }
        log::info!("Run finished successfully");
        Ok(())
    }

    fn get_app_path(&self, app_paths: Vec<PathBuf>) -> Result<PathBuf> {
        if self.simulator
            && cfg!(all(
                target_os = "macos",
                target_arch = "aarch64-apple-darwin"
            ))
        {
            // TODO: Test on apple silicon simulator
            Self::get_app_path_by_target(app_paths, AppleTarget::Aarch64AppleIos)
        } else if self.simulator && cfg!(target_os = "macos") {
            Self::get_app_path_by_target(app_paths, AppleTarget::X86_64AppleIos)
        } else {
            // TODO: Support run on device
            Err(Error::UnsupportedFeature)
        }
    }

    fn get_app_path_by_target(app_paths: Vec<PathBuf>, target: AppleTarget) -> Result<PathBuf> {
        let mut iter = app_paths.iter();
        let res = iter.find(|&x| x.to_str().unwrap().contains(target.rust_triple()));
        Ok(res.ok_or(Error::CantFindTargetToRun)?.to_owned())
    }
}
