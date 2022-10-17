pub mod check;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::{error::CommandExt, types::Config};

#[derive(Parser, Clone, Debug)]
pub struct UpdateCommand {
    #[clap(long)]
    /// Check the crossbundle package version used by the user and compare it with the
    /// version in `crates.io`
    pub check: bool,
    #[clap(long)]
    /// Update crossbundle if a new version was found in `crates.io`
    pub update: bool,
}

impl UpdateCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        if self.check {
            check::check(config)?;
        }
        if self.update {
            self_update(config)?;
        }
        Ok(())
    }
}

/// Self-update crossbundle project and output update status
pub(crate) fn self_update(config: &Config) -> Result<()> {
    config.status("Running `cargo install crossbundle --force` command")?;
    let mut cargo_cmd = std::process::Command::new("cargo");
    cargo_cmd.arg("install").arg("crossbundle").arg("--force");
    cargo_cmd.output_err(true)?;
    Ok(())
}
