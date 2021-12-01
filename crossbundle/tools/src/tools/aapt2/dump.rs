use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// # Dump
/// Dump is used for printing information about the APK you generated using the link
/// command. For example, the following command prints content from the resource table of
/// the specified APK:
///
/// ```sh
/// `aapt2 dump resources output.apk`
/// ```
///
/// ## Dump syntax
/// The general syntax for using dump is as follows:
///
/// ```sh
/// `aapt2 dump sub-command filename.apk [options]`
/// ```
pub struct Aapt2Dump {
    subcommand: SubCommand,
    filename_apk: PathBuf,
    no_values: bool,
    dumped_file: Option<PathBuf>,
    verbose: bool,
    help: bool,
}

impl Aapt2Dump {
    /// Initialize aapt2 dump then specifies subcommand and apk file
    pub fn new(subcommand: SubCommand, filename_apk: &Path) -> Self {
        Self {
            subcommand,
            filename_apk: filename_apk.to_owned(),
            no_values: false,
            dumped_file: None,
            verbose: false,
            help: false,
        }
    }

    /// Suppresses the output of values when displaying resource
    pub fn no_values(&mut self, no_values: bool) -> &mut Self {
        self.no_values = no_values;
        self
    }

    /// Specifies a file as an argument to be dumped from the APK
    pub fn dumped_file(&mut self, dumped_file: &Path) -> &mut Self {
        self.dumped_file = Some(dumped_file.to_owned());
        self
    }

    /// Increases verbosity of the output
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Executes aapt2 dump with arguments
    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("dump");
        aapt2.arg(self.subcommand.to_string());
        aapt2.arg(&self.filename_apk);
        if self.no_values {
            aapt2.arg("--no-values");
        }
        if let Some(dumped_file) = &self.dumped_file {
            aapt2.arg("--file").arg(dumped_file);
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

pub enum SubCommand {
    /// Print the contents of the AAPT2 Container (APC) generated during compilation.
    Apc,
    /// Print information extracted from the APK's manifest.
    Badging,
    /// Print every configuration used by a resource in the APK.
    Configurations,
    /// Print the APK's package name.
    Packagename,
    /// Print the permissions extracted from the APK's manifest.
    Permissions,
    /// Print the contents of the APK's resource table string pool.
    Strings,
    /// Print the parents of styles used in the APK.
    Styleparents,
    /// Print the contents of the APK's resource table.
    Resources,
    /// Print strings from the APK's compiled xml.
    Xmlstrings,
    /// Print a tree of the APK's compiled xml.
    Xmltree,
}

impl std::fmt::Display for SubCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Apc => write!(f, "apc"),
            Self::Badging => write!(f, "badging"),
            Self::Configurations => write!(f, "configurations"),
            Self::Packagename => write!(f, "packagename"),
            Self::Permissions => write!(f, "permissions"),
            Self::Strings => write!(f, "strings"),
            Self::Styleparents => write!(f, "styleparents"),
            Self::Resources => write!(f, "resources"),
            Self::Xmlstrings => write!(f, "xmlstrings"),
            Self::Xmltree => write!(f, "xmltree"),
        }
    }
}
