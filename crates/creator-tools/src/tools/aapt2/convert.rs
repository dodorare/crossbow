use std::{path::{Path, PathBuf}, process::Command};
use crate::error::{CommandExt, Result};

pub struct Aapt2Convert {
    /// Output path
    o: PathBuf,
    /// Format of the output. Accepted values are 'proto' and 'binary'. When not set,
    /// defaults to 'binary'.
    output_format: OutputFormat,
    /// Enables encoding sparse entries using a binary search tree. This decreases APK
    /// size at the cost of resource retrieval performance.
    enable_sparse_encoding: bool,
    /// Preserve raw attribute values in xml files when using the 'binary' output format
    keep_raw_values: bool,
    /// Enables verbose logging`
    v: bool,
}

impl Aapt2Convert {
    pub fn new(o: &Path, output_format: OutputFormat) -> Self{
       Self {
        o: o.to_owned(),
        output_format,
        enable_sparse_encoding: false,
        keep_raw_values: false,
        v: false,
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

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("convert");
        aapt2.arg(&self.o);
        aapt2.arg(&self.output_format.to_string());
        if self.enable_sparse_encoding{
            aapt2.arg("--enable-sparse-encoding ");
        }
        if self.keep_raw_values {
            aapt2.arg("--keep-raw-values");
        }
        if self.v {
            aapt2.arg("-v");
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
