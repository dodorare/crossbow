use crate::{error::*, types::*};
use crossbundle_tools::{
    commands::{android::gen_manifest, *},
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
    pub android_config: AndroidConfig,
    pub apple_config: AppleConfig,
    pub target_dir: PathBuf,
}

impl BuildContext {
    /// Create new instance of build context
    pub fn new(config: &Config, target_dir: Option<PathBuf>) -> Result<Self> {
        let workspace_manifest_path = find_workspace_cargo_manifest_path(config.current_dir())?;
        let package_manifest_path = find_package_cargo_manifest_path(config.current_dir())?;
        let project_path = package_manifest_path.parent().unwrap().to_owned();
        let target_dir =
            target_dir.unwrap_or_else(|| workspace_manifest_path.parent().unwrap().join("target"));
        info!("Parsing Cargo.toml");
        let manifest = parse_manifest(&package_manifest_path)?;
        let cargo_metadata = manifest
            .custom_metadata()
            .ok_or(Error::InvalidCargoMetadata)?
            .to_owned();
        let metadata = cargo_metadata
            .clone()
            .try_into::<Metadata>()
            .map_err(|_| Error::InvalidAndroidConfigMetadata)?;
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            manifest,
            project_path,
            android_config: metadata.android,
            apple_config: metadata.apple,
            target_dir,
        })
    }

    /// Get package name from cargo manifest
    pub fn package_name(&self) -> String {
        self.manifest.summary().name().to_string()
    }

    /// Get package version from cargo manifest
    pub fn package_version(&self) -> String {
        self.manifest.summary().version().to_string()
    }

    /// Get target sdk version from cargo manifest
    pub fn target_sdk_version(&self, sdk: &AndroidSdk) -> u32 {
        if let Some(target_sdk_version) = self.android_config.target_sdk_version {
            return target_sdk_version;
        };
        sdk.default_platform()
    }

    /// Get android build targets from cargo manifest
    pub fn android_build_targets(&self, build_targets: &Vec<AndroidTarget>) -> Vec<AndroidTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        };
        if self.android_config.build_targets.is_none() {
            return vec![AndroidTarget::Aarch64LinuxAndroid];
        };
        let targets = self.android_config.build_targets.clone();
        if targets.is_some() && !targets.as_ref().unwrap().is_empty() {
            return targets.unwrap();
        };
        vec![AndroidTarget::Aarch64LinuxAndroid]
    }

    /// Get android resources from cargo manifest
    pub fn android_res(&self) -> Option<PathBuf> {
        self.android_config.res.clone()
    }

    /// Get android assets from cargo manifest
    pub fn android_assets(&self) -> Option<PathBuf> {
        self.android_config.assets.clone()
    }

    /// Get android package id from cargo manifest
    pub fn android_package(&self, package_name: &str) -> String {
        self.android_config
            .package
            .clone()
            .unwrap_or(format!("com.rust.{}", package_name))
            .replace('-', "_")
    }

    /// Get android manifest from the path in cargo manifest or generate it with the given configuration
    pub fn gen_android_manifest(
        &self,
        sdk: &AndroidSdk,
        package_name: &str,
        debuggable: bool,
        gradle: bool,
    ) -> Result<AndroidManifest> {
        let android_manifest = AndroidConfig {
            app_name: self.android_config.app_name.clone(),
            version_name: Some(
                self.android_config
                    .version_name
                    .clone()
                    .unwrap_or_else(|| self.package_version()),
            ),
            version_code: self.android_config.version_code,
            min_sdk_version: Some(
                self.android_config
                    .min_sdk_version
                    .unwrap_or(MIN_SDK_VERSION),
            ),
            target_sdk_version: Some(
                self.android_config
                    .target_sdk_version
                    .unwrap_or_else(|| sdk.default_platform()),
            ),
            max_sdk_version: self.android_config.max_sdk_version,
            icon: self.android_config.icon.clone(),
            permissions_sdk_23: self.android_config.permissions_sdk_23.clone(),
            permissions: self.android_config.permissions.clone(),
            features: self.android_config.features.clone(),
            service: self.android_config.service.clone(),
            meta_data: self.android_config.meta_data.clone(),
            queries: self.android_config.queries.clone(),
            ..Default::default()
        };
        if let Some(manifest_path) = &self.android_config.manifest_path {
            Ok(android::read_android_manifest(manifest_path)?)
        } else {
            let manifest = gen_manifest::gen_android_manifest(
                Some(format!("com.rust.{}", package_name).replace('-', "_")),
                package_name.to_string(),
                android_manifest.app_name,
                android_manifest
                    .version_name
                    .unwrap_or_else(|| self.package_version()),
                android_manifest.version_code.unwrap_or(1),
                Some(android_manifest.min_sdk_version.unwrap_or(MIN_SDK_VERSION)),
                android_manifest
                    .target_sdk_version
                    .unwrap_or_else(|| sdk.default_platform()),
                android_manifest.max_sdk_version,
                android_manifest.icon,
                debuggable,
                android_manifest.permissions_sdk_23,
                android_manifest.permissions,
                android_manifest.features,
                android_manifest.service,
                android_manifest.meta_data,
                android_manifest.queries,
                gradle,
            );
            Ok(manifest)
        }
    }

    /// Get info plist from the path in cargo manifest or generate it with the given configuration
    pub fn gen_info_plist(&self, package_name: &str) -> Result<InfoPlist> {
        if let Some(info_plist_path) = &self.apple_config.info_plist_path {
            Ok(apple::read_info_plist(info_plist_path)?)
        } else {
            Ok(apple::gen_minimal_info_plist(
                package_name,
                self.apple_config.app_name.clone(),
                self.apple_config
                    .version_name
                    .clone()
                    .unwrap_or_else(|| self.package_version()),
            ))
        }
    }

    /// Get apple build targets from cargo manifest
    pub fn apple_build_targets(&self, build_targets: &Vec<AppleTarget>) -> Vec<AppleTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        };
        if self.apple_config.build_targets.is_none() {
            return vec![AppleTarget::X86_64AppleIos];
        };
        let targets = self.apple_config.clone().build_targets;
        if targets.is_some() && !targets.as_ref().unwrap().is_empty() {
            return targets.unwrap();
        };
        vec![AppleTarget::X86_64AppleIos]
    }

    /// Get apple resources from cargo manifest
    pub fn apple_res(&self) -> Option<PathBuf> {
        self.apple_config.res.clone()
    }

    /// Get apple assets from cargo manifest
    pub fn apple_assets(&self) -> Option<PathBuf> {
        self.apple_config.assets.clone()
    }
}
