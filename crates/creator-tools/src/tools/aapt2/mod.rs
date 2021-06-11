#![allow(dead_code)]

/// Android Asset Packaging Tool 2.0 (AAPT2).
/// https://developer.android.com/studio/command-line/aapt2
/// https://android.googlesource.com/platform/frameworks/base/+/master/tools/aapt2
///
/// The main idea behind AAPT2, apart from new features, is that it divides
/// the 'package' step into two: 'compile' and 'link'. It improves performance,
/// since if only one file changes, you only need to recompile that one file and
/// link all the intermediate files with the 'link' command.
mod compile;
mod convert;
mod dump;
mod link;
mod optimize;

pub use compile::*;
pub use convert::*;
pub use dump::*;
pub use link::*;
pub use optimize::*;

use std::path::{Path, PathBuf};

pub struct Aapt2;

impl Aapt2 {
    /// Compiles resources to be linked into an apk.
    pub fn compile(self, o: &Path, manifest: &Path) -> Aapt2Compile {
        Aapt2Compile::new(o, manifest)
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
    pub fn diff(self) {
        todo!();
    }

    /// Preforms resource optimizations on an apk.
    pub fn optimize(self) -> Aapt2Optimize {
        Aapt2Optimize
    }

    /// Converts an apk between binary and proto formats.
    pub fn convert(self, o: &Path, output_format: OutputFormat) -> Aapt2Convert {
        Aapt2Convert::new(o, output_format)
    }

    /// Prints the version of aapt.
    pub fn version(self) -> String {
        todo!();
    }

    /// Runs aapt in daemon mode. Each subsequent line is a single parameter to the
    /// command. The end of an invocation is signaled by providing an empty line.
    pub fn daemon(self) {
        // probably stream ...
        todo!();
    }
}
