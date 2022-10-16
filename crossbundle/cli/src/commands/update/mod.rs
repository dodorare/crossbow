pub mod check;
pub mod self_update;

use crate::error::Result;
use crate::update::{check::check, self_update::self_update};
use clap::Parser;
use crossbundle_tools::types::Config;

#[derive(Parser, Clone, Debug)]
pub struct UpdateCommand {
    #[clap(long)]
    /// Check the crossbundle package version used by the user and compare it with the
    /// version in `crates.io`
    pub check: bool,
    #[clap(long)]
    /// Update crossbunlde if new version was found in `crates.io`
    pub update: bool,
}

impl UpdateCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        if self.check {
            check(config)?;
        }
        if self.update {
            self_update(config).unwrap();
        }
        Ok(())
    }
}
