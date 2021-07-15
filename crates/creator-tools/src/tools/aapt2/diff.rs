use crate::error::*;
use std::{path::PathBuf, process::Command};

pub struct Aapt2Diff {
    file: Vec<PathBuf>,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Diff {
    pub fn new(file: &[PathBuf]) -> Self {
        Self {
            file: file.to_vec(),
            h: false,
        }
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("diff");
        self.file.iter().for_each(|file| {
            aapt2.arg(file);
        });
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
        let _aapt2 = Aapt2Diff::new(&Vec::new()).h(false).run();
    }
}
