pub mod build;
pub mod run;

use crate::*;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub enum Commands {
    Build(build::BuildCommand),
    Run(run::RunCommand),
}

impl Commands {
    pub fn handle_command(&self, current_dir: PathBuf) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(current_dir),
            Commands::Run(cmd) => cmd.handle_command(current_dir),
        }
    }
}
