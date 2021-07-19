use crate::error::{CommandExt, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Aapt2Convert {
    /// Output path
    o: PathBuf,
    /// Format of the output. Accepted values are 'proto' and 'binary'. When not set,
    /// defaults to 'binary'.
    output_format: Option<OutputFormat>,
    /// Enables encoding sparse entries using a binary search tree. This decreases APK
    /// size at the cost of resource retrieval performance.
    enable_sparse_encoding: bool,
    /// Preserve raw attribute values in xml files when using the 'binary' output format
    keep_raw_values: bool,
    /// Enables verbose logging`
    v: bool,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Convert {
    pub fn new(o: &Path) -> Self {
        Self {
            o: o.to_owned(),
            output_format: None,
            enable_sparse_encoding: false,
            keep_raw_values: false,
            v: false,
            h: false,
        }
    }

    pub fn enable_sparse_encoding(&mut self, enable_sparse_encoding: bool) -> &mut Self {
        self.enable_sparse_encoding = enable_sparse_encoding;
        self
    }

    pub fn keep_raw_values(&mut self, keep_raw_values: bool) -> &mut Self {
        self.keep_raw_values = keep_raw_values;
        self
    }

    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("convert");
        aapt2.arg("-o").arg(&self.o);
        if let Some(output_format) = &self.output_format {
            aapt2.arg("--output-format").arg(output_format.to_string());
        }
        if self.enable_sparse_encoding {
            aapt2.arg("--enable-sparse-encoding");
        }
        if self.keep_raw_values {
            aapt2.arg("--keep-raw-values");
        }
        if self.v {
            aapt2.arg("-v");
        }
        if self.h {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Proto,
    Binary,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Proto => write!(f, "proto"),
            Self::Binary => write!(f, "binary"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let _aapt2 = Aapt2Convert::new(&Path::new("res\\mipmap\\"))
            .keep_raw_values(true)
            .run();
    }
}
