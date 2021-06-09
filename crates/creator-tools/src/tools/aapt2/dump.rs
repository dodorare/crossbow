use std::path::PathBuf;
use std::process::Command;

/// # Dump
/// Dump is used for printing information about the APK you generated using the link command.
/// For example, the following command prints content from the resource table of the specified APK:
///
/// ```
/// aapt2 dump resources output.apk
/// ```
///
/// ## Dump syntax
/// The general syntax for using dump is as follows:
///
/// ```
/// aapt2 dump sub-command filename.apk [options]
/// ```
///
/// ## [Dump options](https://developer.android.com/studio/command-line/aapt2#dump_options)
pub struct Aapt2Dump {
    /// Suppresses the output of values when displaying resource.
    no_values: bool,
    /// Suppresses the output of values when displaying resource.
    file: Option<PathBuf>,
    /// Increases verbosity of the output.
    v: bool,
    filename_apk: SubCommand,
    file_name_apk: PathBuf,
}

pub struct Aapt2DumpBuilder {
    no_values: bool,
    file: Option<PathBuf>,
    v: bool,
    filename_apk: SubCommand,
    file_name_apk: PathBuf,
}

impl Aapt2DumpBuilder {
    pub fn new() -> Aapt2DumpBuilder {
        Aapt2DumpBuilder {
            no_values: false,
            file: None,
            v: false,
            filename_apk: SubCommand,
            file_name_apk: PathBuf,
    }

    pub fn no_values(&mut self, no_values: &bool) -> &mut Self {
        self.no_values = *no_values;
        self
    }

    pub fn file(&mut self, file: &Path) -> &mut Self {
        self.file = *file;
        self
    }

    pub fn v(&mut self, v: &bool) -> &mut Self {
        self.v = *v;
        self
    }

    pub fn filename_apk(&mut self, filename_apk: &bool) -> &mut Self {
        self.filename_apk = *filename_apk;
        self
    }

    pub fn file_name_apk(&mut self, file_name_apk: &bool) -> &mut Self {
        self.file_name_apk = *file_name_apk;
        self
    }

    pub fn run(&self) {
        let aapt2 = Command::new("aapt2");
        aapt2.arg("dump");
        aapt2.arg("filename_apk");
        if let no_values = &self.no_values {
            aapt2.arg("--no-values");
        }
        if let file = &self.file {
            aapt2.arg("--file");
        }
        if let no_values = &self.v {
            aapt2.arg("--v");
        }
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

