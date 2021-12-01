use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Aapt2Daemon {
    trace_folder: PathBuf,
    help: bool,
}

impl Aapt2Daemon {
    /// Initialize aapt2 daemon and then specifies path to trace folder
    pub fn new(trace_folder: &Path) -> Self {
        Self {
            trace_folder: trace_folder.to_owned(),
            help: false,
        }
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Executes aapt2 daemon with arguments
    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("daemon");
        aapt2.arg(&self.trace_folder);
        if self.help {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}
