mod check;

use crate::error::Result;
use crate::update::check::check;
use clap::Parser;
use crossbundle_tools::types::Config;

#[derive(Parser, Clone, Debug)]
pub struct UpdateCommand {
    #[clap(long)]
    pub check: bool,
    #[clap(long)]
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
