use crate::commands::build::{android::AndroidBuildCommand, BuildContext};
use crate::*;
use clap::Clap;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub struct AndroidRunCommand {
    #[clap(flatten)]
    pub build_command: AndroidBuildCommand,
}

impl AndroidRunCommand {
    pub fn run(&self, config: &Config, current_dir: PathBuf) -> Result<()> {
        let build_context =
            BuildContext::init(&current_dir, self.build_command.shared.target_dir.clone())?;
        let (package_name, sdk, apk_path) = self.build_command.execute(config, &build_context)?;
        config.shell().status("Starting run process", "")?;
        config.shell().status("Installing APK file", "")?;
        install_apk(&sdk, &apk_path)?;
        config.shell().status("Starting APK file", "")?;
        start_apk(&sdk, &package_name)?;
        config.shell().status("Run finished successfully", "")?;
        Ok(())
    }
}
