#[cfg(feature = "android")]
mod android;
#[cfg(feature = "apple")]
mod apple;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::types::CliContext;

#[derive(Parser, Clone, Debug)]
pub enum RunCommand {
    /// Executes `build` command and then deploy and launches the application on the
    /// Android device/emulator
    #[cfg(feature = "android")]
    Android(android::AndroidRunCommand),
    /// Builds, deploys, and launches the application on an iOS device or Simulator
    #[cfg(feature = "apple")]
    Ios(apple::IosRunCommand),
}

impl RunCommand {
    pub fn handle_command(&self, _context: &CliContext) -> Result<()> {
        #[cfg(any(feature = "android", feature = "apple"))]
        match &self {
            #[cfg(feature = "android")]
            Self::Android(cmd) => cmd.run(_context)?,
            #[cfg(feature = "apple")]
            Self::Ios(cmd) => cmd.run(_context)?,
        }
        Ok(())
    }
}
