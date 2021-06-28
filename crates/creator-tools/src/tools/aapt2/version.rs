use crate::error::*;
use std::process::Command;

pub struct Aapt2Version {
    version: String,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Version {
    pub fn new(version: String) -> Self {
        Self {
            version: version.to_owned(),
            h: false,
        }
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("version");
        aapt2.arg(&self.version);
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
        let _aapt2 = Aapt2Version::new("1.0.0".to_string()).h(false).run();
    }
}
