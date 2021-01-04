mod android;
mod apple;

use crate::*;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct RunCommand {
    #[clap(subcommand)]
    pub cmd: RunCommandInner,
}

impl RunCommand {
    pub fn handle_command(&self, current_dir: PathBuf) -> Result<()> {
        match &self.cmd {
            RunCommandInner::Android(cmd) => cmd.run(current_dir),
            RunCommandInner::Apple(cmd) => cmd.run(current_dir),
        }
    }
}

#[derive(Clap)]
pub enum RunCommandInner {
    Android(android::AndroidRunCommand),
    Apple(apple::AppleRunCommand),
}
