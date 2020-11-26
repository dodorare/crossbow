use super::builder::ApkBuilder;
use crate::builder::android::error::*;
use crate::builder::android::metadata::AndroidMetadata;
use crate::builder::android::ndk_build::Ndk;
use crate::builder::android::target::AndroidTarget;
use crate::cargo::{artifact::Artifact, profile::Profile, CargoManifest};
use crate::cli::CliBuildAndroid;

use cargo_toml::Product;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ApkBuilderConfig {
    ndk: Ndk,
    artifacts: Option<Vec<Artifact>>,
    build_targets: Vec<AndroidTarget>,
    build_dir: Option<PathBuf>,
    version_name: Option<String>,
    version_code: Option<String>,
    profile: Profile,
    assets: Option<PathBuf>,
    res: Option<String>,
    metadata: Option<AndroidMetadata>,
}

impl ApkBuilderConfig {
    pub fn new() -> Result<Self, NdkError> {
        Ok(ApkBuilderConfig {
            ndk: Ndk::from_env()?,
            artifacts: None,
            build_targets: Vec::new(),
            build_dir: None,
            version_name: None,
            version_code: None,
            profile: Profile::Dev,
            assets: None,
            res: None,
            metadata: None,
        })
    }

    // pub fn cli(mut self, cli: CliBuildAndroid) -> Result<Self, NdkError> {
    //     let mut artifacts = Vec::new();
    //     artifacts.append(
    //         &mut cli
    //             .cargo
    //             .bin
    //             .into_iter()
    //             .map(|v| Artifact::Root(v))
    //             .collect(),
    //     );
    //     artifacts.append(
    //         &mut cli
    //             .cargo
    //             .example
    //             .into_iter()
    //             .map(|v| Artifact::Example(v))
    //             .collect(),
    //     );
    //     if !artifacts.is_empty() {
    //         self.artifacts = Some(artifacts);
    //     };
    //     let mut build_targets = Vec::new();
    //     for target in cli.cargo.target {
    //         let build_target = AndroidTarget::from_rust_triple(&target)?;
    //         build_targets.push(build_target);
    //     }
    //     self.build_targets = build_targets;
    //     if cli.cargo.release {
    //         self.profile = Profile::Release;
    //     };
    //     // self.build_dir = Some(
    //     //     dunce::simplified(cli.cargo.target_dir)
    //     //         .join(self.profile)
    //     //         .join("apk"),
    //     // );
    //     Ok(self)
    // }

    pub fn manifest(mut self, manifest: CargoManifest) -> Result<Self, NdkError> {
        if self.build_targets.is_empty() {
            if let Some(package) = manifest.package {
                if let Some(metadata) = package.metadata {
                    for target in metadata.android.build_targets {
                        self.build_targets.push(target);
                    }
                };
            };
        };
        // Todo: take only bin|examples from all `cargo_toml::Product`
        Ok(self)
    }

    // Todo: add more setters

    pub fn finish(self) -> ApkBuilder {
        // Todo: check if inited options enough for building apk file
        ApkBuilder::from_config(self)
    }
}
