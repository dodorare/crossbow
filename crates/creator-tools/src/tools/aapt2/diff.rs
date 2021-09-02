use crate::error::*;
use std::{path::PathBuf, process::Command};

pub struct Aapt2Diff {
    input_apks: Vec<PathBuf>,
    help: bool,
}

impl Aapt2Diff {
    /// Initialize struct Aapt2Diff and then specifies paths to input apks
    pub fn new(input_apks: &[PathBuf]) -> Self {
        Self {
            input_apks: input_apks.to_vec(),
            help: false,
        }
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Opens the command line and launches aapt2 diff with arguments
    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("diff");
        self.input_apks.iter().for_each(|input_apks| {
            aapt2.arg(input_apks);
        });
        if self.help {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}
