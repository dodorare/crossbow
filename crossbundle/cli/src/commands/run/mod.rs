#[cfg(feature = "android")]
mod android;
#[cfg(feature = "apple")]
mod apple;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::types::Config;

#[derive(Parser, Clone, Debug)]
pub enum RunCommand {
    /// Executes `build` command and then deploy and launches the application on the
    /// Android device/emulator
    #[cfg(feature = "android")]
    Android(android::AndroidRunCommand),
    /// Executes `build` command and then deploy and launches the application on the iOS
    /// device/emulator
    #[cfg(feature = "apple")]
    Ios(apple::IosRunCommand),
}

impl RunCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        #[cfg(any(feature = "android", feature = "apple"))]
        match &self {
            #[cfg(feature = "android")]
            Self::Android(cmd) => cmd.run(config)?,
            #[cfg(feature = "apple")]
            Self::Ios(cmd) => cmd.run(config)?,
        }
        Ok(())
    }
}
