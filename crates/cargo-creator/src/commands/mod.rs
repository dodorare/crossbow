mod build;

use crate::error::*;
use clap::Clap;

#[derive(Clap)]
pub enum Commands {
    Build(build::BuildCommand),
}

impl Commands {
    pub fn handle_command(&self) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(),
        }
    }
}
