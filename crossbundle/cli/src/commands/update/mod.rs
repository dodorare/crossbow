mod check;

use crate::error::Result;
use crate::update::check::check;
use clap::Parser;
use crossbundle_tools::types::Config;

#[derive(Parser, Clone, Debug)]
pub struct UpdateCommand {
    #[clap(long)]
    /// Check the crossbundle package version used by the user and compare it with the
    /// version in `crates.io`
    pub check: bool,
    #[clap(long)]
    /// TODO
    pub update: bool,
}

impl UpdateCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        if self.check {
            check(config)?;
        }
        Ok(())
    }
}
