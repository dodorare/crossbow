use crate::error::*;
use std::path::PathBuf;
use std::process::Command;

pub struct Aapt2Optimize {
    /// Path to the output APK.
    o: PathBuf,
    /// Path to the output directory (for splits).
    d: PathBuf,
    /// Path to XML configuration file.
    x: PathBuf,
    /// Print the multi APK artifacts and exit.
    p: bool,
    /// Comma separated list of the screen densities that the APK will be optimized for.
    /// All the resources that would be unused on devices of the given densities will be
    /// removed from the APK.
    target_densities: Option<String>,
    /// Path to the resources.cfg file containing the list of resources and directives to
    /// each resource. ```Format: type/resource_name#[directive][,directive]```
    resources_config_path: Option<PathBuf>,
    /// Comma separated list of configurations to include. The default is all
    /// configurations.
    c: Option<String>,
    /// Split resources matching a set of configs out to a Split APK.
    /// ```Syntax: path/to/output.apk;<config>[,<config>[...]].```
    /// On Windows, use a semicolon ';' separator instead.
    split: Option<PathBuf>,
    /// Comma separated list of artifacts to keep.
    /// If none are specified, all artifacts will be kept.
    keep_artifacts: Option<String>,
    /// Enables encoding sparse entries using a binary search tree. This decreases APK
    /// size at the cost of resource retrieval performance.
    enable_sparse_encoding: bool,
    /// Collapses resource names to a single value in the key string pool.
    /// Resources can be exempted using the "no_collapse" directive in a file specified by
    /// --resources-config-path.
    collapse_resource_name: bool,
    /// Shortens the paths of resources inside the APK.
    shorten_resource_paths: bool,
    /// Path to output the map of old resource paths to shortened paths.
    resource_path_shortening_map: Option<PathBuf>,
    /// Enables verbose logging
    v: bool,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Optimize {
    pub fn new(o: &Path, d: &Path, x: &Path) -> Self {
        Self {
            o: o.to_owned(),
            d: d.to_owned(),
            x: x.to_owned(),
            p: false,
            target_densities: None,
            resources_config_path: None,
            c: None,
            split: None,
            keep_artifacts: None,
            enable_sparse_encoding: false,
            collapse_resource_name: false,
            shorten_resource_paths: false,
            resource_path_shortening_map: None,
            v: false,
            h: false,
        }
    }

    fn o(&self, o: &Path) -> &mut Self {
        self.o
    }

    pub fn run(self) {
        todo!();
    }
}
