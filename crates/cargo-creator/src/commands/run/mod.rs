mod android;
mod apple;

use crate::*;
use clap::Clap;
use creator_tools::Config;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub enum RunCommand {
    Android(android::AndroidRunCommand),
    Apple(apple::AppleRunCommand),
}

impl RunCommand {
    pub fn handle_command(&self, config: &Config, current_dir: PathBuf) -> Result<()> {
        match &self {
            Self::Android(cmd) => cmd.run(config, current_dir),
            Self::Apple(cmd) => cmd.run(config, current_dir),
        }
    }
}
