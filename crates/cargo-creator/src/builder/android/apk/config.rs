use super::builder::ApkBuilder;
use crate::builder::android::error::*;
use crate::builder::android::target::AndroidTarget;
use crate::cargo::AndroidCargoManifest;
use crate::cli::CliBuildAndroid;

use cargo_toml::{Product, Profiles};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ApkBuilderConfig {
    artifacts: Option<Vec<Product>>,
    build_targets: Option<Vec<AndroidTarget>>,
    build_dir: Option<PathBuf>,
    version_name: Option<String>,
    version_code: Option<String>,
    profiles: Option<Profiles>,
    assets: Option<PathBuf>,
    res: Option<String>,
}

impl ApkBuilderConfig {
    pub fn cli_cmd(mut self, cli_cmd: CliBuildAndroid) -> ApkBuilderConfig {
        // Todo: init all options that u can take from cli command
        // Todo: should we init `cargo_toml::Product` from cli target?
        self
    }

    pub fn manifest(mut self, manifest: AndroidCargoManifest) -> ApkBuilderConfig {
        // Todo: init all options that u can take `Cargo.toml` file
        // Todo: take only bin|examples from all `cargo_toml::Product`
        self
    }

    // Todo: add more setters

    pub fn finish(self) -> ApkBuilder {
        // Todo: check if inited options enough for building apk file
        ApkBuilder::from_config(self)
    }
}
