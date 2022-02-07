pub mod bundletool;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::utils::Config;

use self::bundletool::AndroidInstallCommand;

#[derive(Parser, Clone, Debug)]
pub enum InstallCommand {
    Bundletool(AndroidInstallCommand),
}

impl InstallCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            InstallCommand::Bundletool(cmd) => cmd.install(config),
        }
    }
}
