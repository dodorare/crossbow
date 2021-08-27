use crate::error::*;
use std::process::Command;

pub struct Aapt2Version {
    version: String,
    /// Displays this help menu
    help: bool,
}

impl Aapt2Version {
    pub fn new(version: String) -> Self {
        Self {
            version: version.to_owned(),
            help: false,
        }
    }

    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("version");
        aapt2.arg(&self.version);
        if self.help {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}
