use crate::error::{CommandExt, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Default)]
pub struct Aapt2Convert {
    output_path: PathBuf,
    output_format: Option<OutputFormat>,
    enable_sparse_encoding: bool,
    keep_raw_values: bool,
    verbose: bool,
    help: bool,
}

impl Aapt2Convert {
    /// Initialize aapt2 convert and then specifies output path to convert
    pub fn new(output_path: &Path) -> Self {
        Self {
            output_path: output_path.to_owned(),
            ..Default::default()
        }
    }

    /// Format of the output. Accepted values are `proto` and `binary`. When not set,
    /// defaults to `binary`
    pub fn output_format(&mut self, output_format: OutputFormat) -> &mut Self {
        self.output_format = Some(output_format);
        self
    }

    /// Enables encoding sparse entries using a binary search tree. This decreases APK
    /// size at the cost of resource retrieval performance
    pub fn enable_sparse_encoding(&mut self, enable_sparse_encoding: bool) -> &mut Self {
        self.enable_sparse_encoding = enable_sparse_encoding;
        self
    }

    /// Preserve raw attribute values in xml files when using the 'binary' output format
    pub fn keep_raw_values(&mut self, keep_raw_values: bool) -> &mut Self {
        self.keep_raw_values = keep_raw_values;
        self
    }

    /// Enables verbose logging
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Executes aapt2 convert with arguments
    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("convert");
        aapt2.arg("-o").arg(&self.output_path);
        if let Some(output_format) = &self.output_format {
            aapt2.arg("--output-format").arg(output_format.to_string());
        }
        if self.enable_sparse_encoding {
            aapt2.arg("--enable-sparse-encoding");
        }
        if self.keep_raw_values {
            aapt2.arg("--keep-raw-values");
        }
        if self.verbose {
            aapt2.arg("-v");
        }
        if self.help {
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
