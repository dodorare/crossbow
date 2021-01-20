use crate::commands::build::{apple::AppleBuildCommand, BuildContext};
use crate::error::*;
use clap::Clap;
use creator_tools::{commands::apple, types::*, utils::Config};
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub struct AppleRunCommand {
    #[clap(flatten)]
    pub build_command: AppleBuildCommand,

    /// Simulator device name.
    #[clap(short, long, default_value = "iPhone 8", conflicts_with = "target")]
    pub simulator_name: String,
    /// Run in debug mode.
    #[clap(short, long)]
    pub debug: bool,
    /// Install and launch on the connected device.
    #[clap(short, long, conflicts_with = "target")]
    pub device: bool,
    /// Connected device id.
    #[clap(short = 'D', long, conflicts_with = "device_name")]
    pub device_id: Option<String>,
}

impl AppleRunCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let mut build_command = self.build_command.clone();
        if self.device && build_command.target.is_empty() {
            build_command.target = vec![AppleTarget::Aarch64AppleIos];
        } else if build_command.target.is_empty() {
            build_command.target = vec![AppleTarget::X86_64AppleIos];
        }
        let build_context = BuildContext::init(config, build_command.shared.target_dir.clone())?;
        let (metadata, app_paths) = build_command.execute(config, &build_context)?;
        config.shell().status("Starting run process")?;
        let bundle_id = &metadata.info_plist.identification.bundle_identifier;
        let app_path = self.get_app_path(&app_paths)?;
        if self.device {
            config.shell().status("Lounching app on connected device")?;
            apple::run_and_debug(&app_path, self.debug, false, false, self.device_id.as_ref())?;
        } else {
            config
                .shell()
                .status("Installing and launching application on simulator")?;
            apple::launch_apple_app(&app_path, &self.simulator_name, bundle_id, true)?;
            creator_tools::simctl::Simctl::new()
                .open()
                .map_err(|err| Error::CreatorTools(err.into()))?;
        }
        config.shell().status("Run finished successfully")?;
        Ok(())
    }

    fn get_app_path(&self, app_paths: &[PathBuf]) -> Result<PathBuf> {
        if self.device {
            Self::get_app_path_by_target(app_paths, AppleTarget::Aarch64AppleIos)
        } else {
            // TODO: Support apple silicon
            // if cfg!(target_arch = "aarch64") {
            //     Self::get_app_path_by_target(app_paths, AppleTarget::Aarch64AppleIos)
            // }
            Self::get_app_path_by_target(app_paths, AppleTarget::X86_64AppleIos)
        }
    }

    fn get_app_path_by_target(app_paths: &[PathBuf], target: AppleTarget) -> Result<PathBuf> {
        let mut iter = app_paths.iter();
        let res = iter.find(|&x| x.to_str().unwrap().contains(target.rust_triple()));
        Ok(res.ok_or(Error::CantFindTargetToRun)?.to_owned())
    }
}
