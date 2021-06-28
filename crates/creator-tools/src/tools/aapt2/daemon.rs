use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Aapt2Daemon {
    trace_folder: PathBuf,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Daemon {
    pub fn new(trace_folder: &Path) -> Self {
        Self {
            trace_folder: trace_folder.to_owned(),
            h: false,
        }
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("daemon");
        aapt2.arg(&self.trace_folder);
        if self.h {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let _aapt2 = Aapt2Daemon::new(Path::new("C:\\Users\\adil_\\AppData\\Local\\test\\")).h(false).run();
    }
}
