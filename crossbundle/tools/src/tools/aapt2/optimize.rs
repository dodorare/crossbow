use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Default)]
pub struct Aapt2Optimize {
    output_apk: PathBuf,
    output_dir: PathBuf,
    config_xml_file: Option<PathBuf>,
    apk_artifacts: bool,
    target_densities: Option<String>,
    resources_config_path: Option<PathBuf>,
    configs_to_include: Option<String>,
    split: Option<PathBuf>,
    keep_artifacts: Option<String>,
    enable_sparse_encoding: bool,
    collapse_resource_name: bool,
    shorten_resource_paths: bool,
    resource_path_shortening_map: Option<PathBuf>,
    verbose: bool,
    help: bool,
}

impl Aapt2Optimize {
    /// Initialize aapt2 optimize then specifies path to the aapt2 link output APK and
    /// Path to the output directory (for splits)
    pub fn new(output_apk: &Path, output_dir: &Path) -> Self {
        Self {
            output_apk: output_apk.to_owned(),
            output_dir: output_dir.to_owned(),
            ..Default::default()
        }
    }

    /// Specifies path to XML configuration file
    pub fn config_xml_file(&mut self, config_xml_file: &Path) -> &mut Self {
        self.config_xml_file = Some(config_xml_file.to_owned());
        self
    }

    /// Print the multi APK artifacts and exit
    pub fn apk_artifacts(&mut self, apk_artifacts: bool) -> &mut Self {
        self.apk_artifacts = apk_artifacts;
        self
    }

    /// Comma separated list of the screen densities that the APK will be optimized for.
    /// All the resources that would be unused on devices of the given densities will be
    /// removed from the APK
    pub fn target_densities(&mut self, target_densities: &str) -> &mut Self {
        self.target_densities = Some(target_densities.to_owned());
        self
    }

    /// Path to the `resources.cfg` file containing the list of resources and directives
    /// to each resource. ```Format: type/resource_name#[directive][,directive]```
    pub fn resources_config_path(&mut self, resources_config_path: &Path) -> &mut Self {
        self.resources_config_path = Some(resources_config_path.to_owned());
        self
    }

    /// Comma separated list of configurations to include. The default is all
    /// configurations
    pub fn configs_to_include(&mut self, configs_to_include: &str) -> &mut Self {
        self.configs_to_include = Some(configs_to_include.to_owned());
        self
    }

    /// Split resources matching a set of configs out to a Split APK.
    /// ```Syntax: path/to/output.apk;<config>[,<config>[...]].```
    /// On Windows, use a semicolon `;` separator instead
    pub fn split(&mut self, split: &Path) -> &mut Self {
        self.split = Some(split.to_owned());
        self
    }

    /// Comma separated list of artifacts to keep.
    /// If none are specified, all artifacts will be kept
    pub fn keep_artifacts(&mut self, keep_artifacts: &str) -> &mut Self {
        self.keep_artifacts = Some(keep_artifacts.to_owned());
        self
    }

    /// Enables encoding sparse entries using a binary search tree. This decreases APK
    /// size at the cost of resource retrieval performance
    pub fn enable_sparse_encoding(&mut self, enable_sparse_encoding: bool) -> &mut Self {
        self.enable_sparse_encoding = enable_sparse_encoding;
        self
    }

    /// Collapses resource names to a single value in the key string pool.
    /// Resources can be exempted using the `no_collapse` directive in a file specified by
    /// `--resources-config-path`
    pub fn collapse_resource_name(&mut self, collapse_resource_name: bool) -> &mut Self {
        self.collapse_resource_name = collapse_resource_name;
        self
    }

    /// Shortens the paths of resources inside the APK
    pub fn shorten_resource_paths(&mut self, shorten_resource_paths: bool) -> &mut Self {
        self.shorten_resource_paths = shorten_resource_paths;
        self
    }

    /// Path to output the map of old resource paths to shortened paths
    pub fn resource_path_shortening_map(
        &mut self,
        resource_path_shortening_map: &Path,
    ) -> &mut Self {
        self.resource_path_shortening_map = Some(resource_path_shortening_map.to_owned());
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

    /// Executes aapt2 optimize with arguments
    pub fn run(self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("optimize");
        aapt2.arg("-o").arg(&self.output_apk);
        aapt2.arg("-d").arg(&self.output_dir);
        if let Some(config_xml_file) = self.config_xml_file {
            aapt2.arg("-x").arg(config_xml_file);
        }
        if self.apk_artifacts {
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
        if let Some(configs_to_include) = self.configs_to_include {
            aapt2.arg("-c").arg(configs_to_include);
        }
        if let Some(split) = self.split {
            aapt2.arg("--split").arg(split);
        }
        if let Some(keep_artifacts) = self.keep_artifacts {
            aapt2.arg("--keep-artifacts").arg(keep_artifacts);
        }
        if self.enable_sparse_encoding {
            aapt2.arg("--enable-sparse-encoding");
        }
        if self.collapse_resource_name {
            aapt2.arg("--collapse-resource-name");
        }
        if self.shorten_resource_paths {
            aapt2.arg("--shorten-resource-paths");
        }
        if let Some(resource_path_shortening_map) = self.resource_path_shortening_map {
            aapt2
                .arg("--resource-path-shortening-map")
                .arg(resource_path_shortening_map);
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
