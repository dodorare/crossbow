use crate::{
    cargo_manifest::Metadata,
    error::{Error, Result},
};
use crossbundle_tools::{
    commands::*,
    tools::*,
    types::{
        android_manifest::AndroidManifest, apple_bundle::prelude::InfoPlist, AndroidTarget,
        AppleTarget,
    },
    utils::*,
};
use std::path::PathBuf;

pub struct BuildContext {
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub manifest: cargo::core::Manifest,
    pub metadata: Metadata,
    pub target_dir: PathBuf,
}

impl BuildContext {
    pub fn new(config: &Config, target_dir: Option<PathBuf>) -> Result<Self> {
        let workspace_manifest_path = find_workspace_cargo_manifest_path(config.current_dir())?;
        let package_manifest_path = find_package_cargo_manifest_path(config.current_dir())?;
        let project_path = package_manifest_path.parent().unwrap().to_owned();
        let target_dir =
            target_dir.unwrap_or_else(|| workspace_manifest_path.parent().unwrap().join("target"));
        info!("Parsing Cargo.toml");
        let manifest = parse_manifest(&package_manifest_path)?;
        let custom_metadata = manifest
            .custom_metadata()
            .ok_or(Error::InvalidManifestMetadata)?
            .to_owned();
        println!("custom_metadataa is {:?}", custom_metadata);
        let metadata = custom_metadata
            .try_into::<Metadata>()
            .map_err(|_| Error::InvalidManifestMetadata)?;
        println!("metadata is {:?}", metadata);
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            manifest,
            project_path,
            metadata,
            target_dir,
        })
    }

    pub fn package_name(&self) -> String {
        if let Some(package_name) = self.metadata.android_package_name.clone() {
            return package_name;
        };
        self.manifest.summary().name().to_string()
    }

    pub fn package_version(&self) -> String {
        self.manifest.summary().version().to_string()
    }

    pub fn target_sdk_version(&self, sdk: &AndroidSdk) -> u32 {
        if let Some(target_sdk_version) = self.metadata.target_sdk_version {
            return target_sdk_version;
        };
        sdk.default_platform()
    }

    pub fn android_build_targets(&self, build_targets: &Vec<AndroidTarget>) -> Vec<AndroidTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        };
        if self.metadata.android_build_targets.is_none() {
            return vec![AndroidTarget::Aarch64LinuxAndroid];
        };
        let targets = self.metadata.android_build_targets.clone();
        if targets.is_some() && !targets.as_ref().unwrap().is_empty() {
            return targets.unwrap();
        };
        vec![AndroidTarget::Aarch64LinuxAndroid]
    }

    pub fn android_res(&self) -> Option<PathBuf> {
        self.metadata.android_res.clone()
    }

    pub fn android_assets(&self) -> Option<PathBuf> {
        self.metadata.android_assets.clone()
    }

    pub fn gen_android_manifest(
        &self,
        sdk: &AndroidSdk,
        package_name: &str,
        debuggable: bool,
    ) -> Result<AndroidManifest> {
        if self.metadata.use_android_manifest {
            let path = self
                .metadata
                .android_manifest_path
                .clone()
                .unwrap_or_else(|| self.project_path.join("AndroidManifest.xml"));
            Ok(android::read_android_manifest(&path)?)
        } else if !self.metadata.use_android_manifest {
            let manifest = android::gen_minimal_android_manifest(
                self.metadata.android_package_name.clone(),
                package_name,
                self.metadata.app_name.clone(),
                self.metadata
                    .version_name
                    .clone()
                    .unwrap_or(self.package_version()),
                self.metadata.version_code.clone(),
                self.metadata.min_sdk_version,
                self.metadata
                    .target_sdk_version
                    .unwrap_or_else(|| sdk.default_platform()),
                self.metadata.max_sdk_version,
                self.metadata.icon.clone(),
                debuggable,
                self.metadata.android_permissions_sdk_23.clone(),
                self.metadata.android_permissions.clone(),
                self.metadata.android_features.clone(),
                self.metadata.android_service.clone(),
            );
            Ok(manifest)
        } else {
            let target_sdk_version = sdk.default_platform();
            Ok(android::gen_minimal_android_manifest(
                None,
                package_name,
                None,
                self.package_version(),
                None,
                None,
                target_sdk_version,
                None,
                None,
                debuggable,
                None,
                None,
                None,
                None,
            ))
        }
    }

    pub fn gen_info_plist(&self, package_name: &String) -> Result<InfoPlist> {
        if self.metadata.use_info_plist {
            let path = self
                .metadata
                .info_plist_path
                .clone()
                .unwrap_or_else(|| self.project_path.join("Info.plist"));
            Ok(apple::read_info_plist(&path)?)
        } else if !self.metadata.use_info_plist {
            Ok(apple::gen_minimal_info_plist(
                package_name,
                self.metadata.app_name.clone(),
                self.metadata
                    .version_name
                    .clone()
                    .unwrap_or(self.package_version()),
            ))
        } else {
            Ok(apple::gen_minimal_info_plist(
                package_name,
                None,
                self.package_version(),
            ))
        }
    }

    pub fn apple_build_targets(&self, build_targets: &Vec<AppleTarget>) -> Vec<AppleTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        };
        if self.metadata.apple_build_targets.is_none() {
            return vec![AppleTarget::X86_64AppleIos];
        };
        let targets = self.metadata.clone().apple_build_targets;
        if targets.is_some() && !targets.as_ref().unwrap().is_empty() {
            return targets.unwrap();
        };
        vec![AppleTarget::X86_64AppleIos]
    }

    pub fn apple_res(&self) -> Option<PathBuf> {
        self.metadata.apple_res.clone()
    }

    pub fn apple_assets(&self) -> Option<PathBuf> {
        self.metadata.apple_assets.clone()
    }
}
