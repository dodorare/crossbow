pub mod build;
pub mod new;
pub mod run;

use crate::error::Result;
use clap::Clap;
use creator_tools::utils::Config;

#[derive(Clap, Clone, Debug)]
pub enum Commands {
    Build(build::BuildCommand),
    Run(run::RunCommand),
    New(new::NewCommand),
}

impl Commands {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(config),
            Commands::Run(cmd) => cmd.handle_command(config),
            Commands::New(cmd) => cmd.handle_command(config),
        }
    }
}
