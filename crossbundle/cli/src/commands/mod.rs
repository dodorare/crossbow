pub mod build;
pub mod install;
pub mod log;
pub mod new;
pub mod run;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::utils::Config;

#[derive(Parser, Clone, Debug)]
pub enum Commands {
    /// Starts the process of building/packaging/signing of the rust crate
    #[clap(subcommand)]
    Build(build::BuildCommand),
    /// Executes `build` command and then deploy and launches the application on the device/emulator
    #[clap(subcommand)]
    Run(run::RunCommand),
    /// Creates a new Cargo package in the given directory. Project will be ready to build with `crossbundle`
    New(new::NewCommand),
    /// Attach logger to device with running application
    #[clap(subcommand)]
    Log(log::LogCommand),
    /// Installs tools
    #[clap(subcommand)]
    Install(install::InstallCommand),
}

impl Commands {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(config),
            Commands::Run(cmd) => cmd.handle_command(config),
            Commands::New(cmd) => cmd.handle_command(config),
            Commands::Log(cmd) => cmd.handle_command(config),
            Commands::Install(cmd) => cmd.handle_command(config),
        }
    }
}
