pub mod build;
pub mod new;
pub mod run;

use crate::*;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub enum Commands {
    Build(build::BuildCommand),
    Run(run::RunCommand),
    New(new::NewCommand),
}

impl Commands {
    pub fn handle_command(&self, config: &Config, current_dir: PathBuf) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(config, current_dir),
            Commands::Run(cmd) => cmd.handle_command(config, current_dir),
            Commands::New(cmd) => cmd.handle_command(config, current_dir),
        }
    }
}
