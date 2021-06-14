use crate::error::*;
use clap::Clap;
use creator_tools::{commands::android, tools::AndroidSdk, utils::Config};

#[derive(Clap, Clone, Debug)]
pub struct AndroidLogCommand;

impl AndroidLogCommand {
    pub fn run(&self, _config: &Config) -> Result<()> {
        let sdk = AndroidSdk::from_env()?;
        android::attach_logger_only_rust(&sdk)?;
        Ok(())
    }
}
