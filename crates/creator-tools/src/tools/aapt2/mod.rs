//! Android Asset Packaging Tool 2.0 (AAPT2).
//! https://developer.android.com/studio/command-line/aapt2
//! https://android.googlesource.com/platform/frameworks/base/+/master/tools/aapt2
//!
//! The main idea behind AAPT2, apart from new features, is that it divides
//! the 'package' step into two: 'compile' and 'link'. It improves performance,
//! since if only one file changes, you only need to recompile that one file and
//! link all the intermediate files with the 'link' command.

mod compile;
mod convert;
mod daemon;
mod diff;
mod dump;
mod link;
mod optimize;
mod version;
mod gen_apk_aapt2;

pub use compile::*;
pub use convert::*;
pub use dump::*;
pub use link::*;
pub use optimize::*;

use std::path::{Path, PathBuf};

use self::{daemon::Aapt2Daemon, diff::Aapt2Diff, version::Aapt2Version};

pub struct Aapt2;

impl Aapt2 {
    /// Compiles resources to be linked into an apk.
    pub fn compile(self, o: &Path, manifest: &Path, visibility: Visibility) -> Aapt2Compile {
        Aapt2Compile::new(o, manifest, visibility)
    }

    /// Links resources into an apk.
    pub fn link(self, inputs: &[PathBuf], o: &Path, manifest: &Path) -> Aapt2Link {
        Aapt2Link::new(inputs, o, manifest)
    }

    /// Used for printing information about the APK you generated using the link command.
    pub fn dump(self, subcommand: SubCommand, filename_apk: &Path) -> Aapt2Dump {
        Aapt2Dump::new(subcommand, filename_apk)
    }

    /// Prints the differences in resources of two apks.
    /// https://gerrit.pixelexperience.org/plugins/gitiles/frameworks_base/+/refs/tags/android-10.0.0_r2/tools/aapt2/cmd/Diff.cpp
    pub fn diff(self, file: &[PathBuf]) -> Aapt2Diff {
        Aapt2Diff::new(file)
    }

    /// Preforms resource optimizations on an apk.
    pub fn optimize(self, o: &PathBuf, d: &PathBuf, x: &PathBuf) -> Aapt2Optimize {
        Aapt2Optimize::new(o, d, x)
    }

    /// Converts an apk between binary and proto formats.
    pub fn convert(self, o: &Path, output_format: OutputFormat) -> Aapt2Convert {
        Aapt2Convert::new(o, output_format)
    }

    /// Prints the version of aapt.
    pub fn version(self, version: String) -> Aapt2Version {
        Aapt2Version::new(version.to_string())
    }

    /// Runs aapt in daemon mode. Each subsequent line is a single parameter to the
    /// command. The end of an invocation is signaled by providing an empty line.
    pub fn daemon(self, trace_folder: &Path) -> Aapt2Daemon {
        Aapt2Daemon::new(trace_folder)
    }
}
