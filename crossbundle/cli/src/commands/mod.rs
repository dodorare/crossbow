pub mod build;
#[cfg(any(feature = "android", feature = "apple"))]
pub mod doctor;
pub mod install;
pub mod new;
pub mod run;
pub mod update;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::types::CliContext;

#[derive(Parser, Clone, Debug)]
pub enum Commands {
    /// Starts the process of building/packaging/signing of the rust crate
    #[clap(subcommand)]
    Build(build::BuildCommand),
    /// Checks host tools and, optionally, an explicit project without changing anything
    #[cfg(any(feature = "android", feature = "apple"))]
    Doctor(doctor::DoctorCommand),
    /// Executes `build` command and then deploy and launches the application on the
    /// device/emulator
    #[clap(subcommand)]
    Run(run::RunCommand),
    /// Creates a new Cargo package in the given directory. Project will be ready to build
    /// with `crossbundle`
    New(new::NewCommand),
    /// Installs bundletool and Android Studio's sdkmanager
    Install(install::InstallCommand),
    /// Updates or checks for new version of Crossbundle
    Update(update::UpdateCommand),
}

impl Commands {
    pub fn handle_command(&self, config: &CliContext) -> Result<()> {
        if self.requires_update_check() {
            crate::update::check::check_new_version(config)?;
        }
        match self {
            Commands::Build(cmd) => cmd.handle_command(config),
            #[cfg(any(feature = "android", feature = "apple"))]
            Commands::Doctor(cmd) => cmd.run(config),
            Commands::Run(cmd) => cmd.handle_command(config),
            Commands::New(cmd) => cmd.handle_command(config),
            Commands::Install(cmd) => cmd.handle_command(config),
            Commands::Update(cmd) => cmd.handle_command(config),
        }
    }

    fn requires_update_check(&self) -> bool {
        match self {
            Commands::Update(_) => false,
            #[cfg(any(feature = "android", feature = "apple"))]
            Commands::Doctor(_) => false,
            #[cfg(feature = "android")]
            Commands::Build(build::BuildCommand::Android(command)) => !command.dry_run,
            #[cfg(feature = "android")]
            Commands::Run(run::RunCommand::Android(command)) => !command.build_command.dry_run,
            _ => true,
        }
    }
}
