mod android;
mod apple;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::utils::Config;

#[derive(Parser, Clone, Debug)]
pub enum RunCommand {
    /// Executes `build` command and then deploy and launches the application on the Android device/emulator
    Android(android::AndroidRunCommand),
    /// Executes `build` command and then deploy and launches the application on the iOS device/emulator
    Ios(apple::IosRunCommand),
}

impl RunCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match &self {
            Self::Android(cmd) => cmd.run(config),
            Self::Ios(cmd) => cmd.run(config),
        }
    }
}
