use crate::error::*;
use clap::Parser;
use crossbundle_tools::{
    commands::android::common::attach_logger_only_rust, tools::AndroidSdk, utils::Config,
};

#[derive(Parser, Clone, Debug)]
pub struct AndroidLogCommand;

impl AndroidLogCommand {
    pub fn run(&self, _config: &Config) -> Result<()> {
        let sdk = AndroidSdk::from_env()?;
        attach_logger_only_rust(&sdk)?;
        Ok(())
    }
}
