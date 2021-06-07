use std::path::PathBuf;

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
    filename_apk: SubCommands,
    file_name_apk: PathBuf,
}

pub enum SubCommands {
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

impl Aapt2Dump {
    pub fn run(self) {
        todo!();
    }
}
