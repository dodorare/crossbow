use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// # Dump
/// Dump is used for printing information about the APK you generated using the link
/// command. For example, the following command prints content from the resource table of
/// the specified APK:
///
/// ```sh
/// aapt2 dump resources output.apk
/// ```
///
/// ## Dump syntax
/// The general syntax for using dump is as follows:
///
/// ```sh
/// aapt2 dump sub-command filename.apk [options]
/// ```
pub struct Aapt2Dump {
    subcommand: SubCommand,
    filename_apk: PathBuf,
    /// Suppresses the output of values when displaying resource.
    no_values: bool,
    /// Suppresses the output of values when displaying resource.
    file: Option<PathBuf>,
    /// Increases verbosity of the output.
    v: bool,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Dump {
    pub fn new(subcommand: SubCommand, filename_apk: &Path) -> Self {
        Self {
            subcommand,
            filename_apk: filename_apk.to_owned(),
            no_values: false,
            file: None,
            v: false,
            h: false,
        }
    }

    pub fn no_values(&mut self, no_values: bool) -> &mut Self {
        self.no_values = no_values;
        self
    }

    pub fn file(&mut self, file: &Path) -> &mut Self {
        self.file = Some(file.to_owned());
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
        aapt2.arg("dump");
        aapt2.arg(self.subcommand.to_string());
        aapt2.arg(&self.filename_apk);
        if self.no_values {
            aapt2.arg("--no-values");
        }
        if let Some(file) = &self.file {
            aapt2.arg("--file").arg(file);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test_one() {
        let mut aapt2 = Aapt2Dump::new(
            // Badging options:
            // --include-meta-data, Include meta-data information.
            //
            // Styleparents options:
            // --style arg, The name of the style to print
            //
            // Xmlstrings and Xmltree options:
            // --file arg,  A compiled xml file to print
            SubCommand::Configurations,
            &Path::new("C:/Users/den99/AndroidStudioProjects/creator_paint.apk"),
        );
        aapt2.no_values(true);
        aapt2.run();
    }
}
