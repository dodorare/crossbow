mod android;
mod apple;

use crate::*;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub enum RunCommand {
    Android(android::AndroidRunCommand),
    Apple(apple::AppleRunCommand),
}

impl RunCommand {
    pub fn handle_command(&self, current_dir: PathBuf) -> Result<()> {
        match &self {
            Self::Android(cmd) => cmd.run(current_dir),
            Self::Apple(cmd) => cmd.run(current_dir),
        }
    }
}
