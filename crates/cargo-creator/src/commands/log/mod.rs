mod android;

use crate::error::Result;
use android::AndroidLogCommand;
use clap::Parser;
use creator_tools::utils::Config;

#[derive(Parser, Clone, Debug)]
pub enum LogCommand {
    Android(AndroidLogCommand),
}

impl LogCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            LogCommand::Android(cmd) => cmd.run(config),
        }
    }
}
