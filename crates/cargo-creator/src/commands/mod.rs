mod build;

use crate::*;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub enum Commands {
    Build(build::BuildCommand),
}

impl Commands {
    pub fn handle_command(&self, current_dir: PathBuf) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(current_dir),
        }
    }
}
