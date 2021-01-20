mod android;
mod apple;

use crate::error::Result;
use clap::Clap;
use creator_tools::utils::Config;

#[derive(Clap, Clone, Debug)]
pub enum RunCommand {
    Android(android::AndroidRunCommand),
    Apple(apple::AppleRunCommand),
}

impl RunCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match &self {
            Self::Android(cmd) => cmd.run(config),
            Self::Apple(cmd) => cmd.run(config),
        }
    }
}
