use crate::commands::build::{apple::AppleBuildCommand, BuildContext};
use crate::*;
use clap::Clap;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap)]
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
        let build_context =
            BuildContext::init(&current_dir, self.build_command.target_dir.clone())?;
        let (metadata, app_dir) = self.build_command.execute(&build_context)?;
        log::info!("Starting run process");
        let bundle_id = &metadata.info_plist.identification.bundle_identifier;
        if self.simulator {
            log::info!("Installing and launching application on simulator");
            launch_apple_app(&app_dir, &self.device_name, bundle_id, false)?;
            creator_tools::simctl::Simctl::new()
                .open()
                .map_err(|err| Error::CreatorTools(err.into()))?;
        }
        log::info!("Run finished successfully");
        Ok(())
    }
}
