use crate::error::*;
use std::path::{Path, PathBuf};
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

    fn p(&mut self, p: bool) -> &mut Self {
        self.p = p;
        self
    }

    fn target_densities(&mut self, target_densities: &str) -> &mut Self {
        self.target_densities = Some(target_densities.to_owned());
        self
    }

    fn c(&mut self, c: &str) -> &mut Self {
        self.c = Some(c.to_owned());
        self
    }

    fn split(&mut self, split: &Path) -> &mut Self {
        self.split = Some(split.to_owned());
        self
    }

    fn keep_artifacts(&mut self, keep_artifacts: &str) -> &mut Self {
        self.keep_artifacts = Some(keep_artifacts.to_owned());
        self
    }

    fn enable_sparse_encoding(&mut self, enable_sparse_encoding: bool) -> &mut Self {
        self.enable_sparse_encoding = enable_sparse_encoding;
        self
    }

    fn collapse_resource_name(&mut self, collapse_resource_name: bool) -> &mut Self {
        self.collapse_resource_name = collapse_resource_name;
        self
    }

    fn shorten_resource_paths(&mut self, shorten_resource_paths: bool) -> &mut Self {
        self.shorten_resource_paths = shorten_resource_paths;
        self
    }

    fn resource_path_shortening_map(&mut self, resource_path_shortening_map: &Path) -> &mut Self {
        self.resource_path_shortening_map = Some(resource_path_shortening_map.to_owned());
        self
    }

    fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(self) {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("optimize");
        aapt2.arg("-o").arg(&self.o);
        aapt2.arg("-d").arg(&self.d);
        aapt2.arg("-x").arg(&self.x);
        if self.p {
            aapt2.arg("-p");
        }
        if let Some(target_densities) = self.target_densities {
            aapt2.arg("--target_densities").arg(target_densities);
        }
        if let Some(resources_config_path) = self.resources_config_path {
            aapt2
                .arg("--resources_config_path")
                .arg(resources_config_path);
        }
        if let Some(c) = self.c {
            aapt2.arg("-c").arg(c);
        }
        if let Some(split) = self.split {
            aapt2.arg("--split").arg(split);
        }
        if let Some(keep_artifacts) = self.keep_artifacts {
            aapt2.arg("--keep_artifacts").arg(keep_artifacts);
        }
        if self.enable_sparse_encoding {
            aapt2.arg("--enable_sparse_encoding");
        }
        if self.collapse_resource_name {
            aapt2.arg("--collapse_resource_name");
        }
        if self.shorten_resource_paths {
            aapt2.arg("--shorten_resource_paths");
        }
        if let Some(resource_path_shortening_map) = self.resource_path_shortening_map {
            aapt2
                .arg("--resource_path_shortening_map")
                .arg(resource_path_shortening_map);
        }
        if self.v {
            aapt2.arg("-v");
        }
        if self.h {
            aapt2.arg("-h");
        }
    }
}
