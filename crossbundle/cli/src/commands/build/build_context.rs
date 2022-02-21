use crate::{
    cargo_manifest::{CargoManifest, CargoPackage, Metadata},
    error::{Error, Result},
};
use crossbundle_tools::{
    commands::{
        android, apple, find_package_cargo_manifest_path, find_workspace_cargo_manifest_path,
    },
    tools::AndroidSdk,
    types::{
        android_manifest::AndroidManifest, apple_bundle::prelude::InfoPlist, AndroidTarget,
        AppleTarget,
    },
    utils::Config,
};
use std::path::PathBuf;

pub struct BuildContext {
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub cargo_package: CargoPackage<Metadata>,
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
        let cargo_manifest = CargoManifest::from_path_with_metadata(&package_manifest_path)?;
        let cargo_package = cargo_manifest.package.ok_or(Error::InvalidManifest)?;
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            project_path,
            cargo_package,
            target_dir,
        })
    }

    pub fn package_name(&self) -> String {
        if let Some(metadata) = &self.cargo_package.metadata {
            if let Some(package_name) = metadata.android_package_name.clone() {
                return package_name;
            };
        };
        self.cargo_package.name.clone()
    }

    pub fn package_version(&self) -> String {
        self.cargo_package.version.clone()
    }

    pub fn target_sdk_version(&self, sdk: &AndroidSdk) -> u32 {
        if let Some(metadata) = &self.cargo_package.metadata {
            if let Some(target_sdk_version) = metadata.target_sdk_version {
                return target_sdk_version;
            };
        };
        sdk.default_platform()
    }

    pub fn android_build_targets(&self, build_targets: &Vec<AndroidTarget>) -> Vec<AndroidTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        };
        if self.cargo_package.metadata.is_none() {
            return vec![AndroidTarget::Aarch64LinuxAndroid];
        };
        let targets = self
            .cargo_package
            .metadata
            .clone()
            .unwrap()
            .android_build_targets;
        if targets.is_some() && !targets.as_ref().unwrap().is_empty() {
            return targets.unwrap();
        };
        vec![AndroidTarget::Aarch64LinuxAndroid]
    }

    pub fn android_res(&self) -> Option<PathBuf> {
        self.cargo_package
            .metadata
            .as_ref()
            .map(|m| m.android_res.clone())
            .unwrap_or_default()
    }

    pub fn android_assets(&self) -> Option<PathBuf> {
        self.cargo_package
            .metadata
            .as_ref()
            .map(|m| m.android_assets.clone())
            .unwrap_or_default()
    }

    pub fn gen_android_manifest(
        &self,
        sdk: &AndroidSdk,
        package_name: &String,
        debuggable: bool,
    ) -> Result<AndroidManifest> {
        if let Some(metadata) = &self.cargo_package.metadata {
            if metadata.use_android_manifest {
                let path = metadata
                    .android_manifest_path
                    .clone()
                    .unwrap_or_else(|| self.project_path.join("AndroidManifest.xml"));
                Ok(android::read_android_manifest(&path)?)
            } else {
                let mut manifest = android::gen_minimal_android_manifest(
                    metadata.android_package_name.clone(),
                    package_name,
                    metadata.app_name.clone(),
                    metadata
                        .version_name
                        .clone()
                        .unwrap_or(self.package_version()),
                    metadata.version_code.clone(),
                    metadata.min_sdk_version,
                    metadata
                        .target_sdk_version
                        .unwrap_or_else(|| sdk.default_platform()),
                    metadata.max_sdk_version,
                    metadata.icon.clone(),
                    debuggable,
                );
                if !metadata.android_permissions.is_empty() {
                    manifest.uses_permission = metadata.android_permissions.clone();
                }
                Ok(manifest)
            }
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
            ))
        }
    }

    pub fn gen_info_plist(&self, package_name: &String) -> Result<InfoPlist> {
        if let Some(metadata) = &self.cargo_package.metadata {
            if metadata.use_info_plist {
                let path = metadata
                    .info_plist_path
                    .clone()
                    .unwrap_or_else(|| self.project_path.join("Info.plist"));
                Ok(apple::read_info_plist(&path)?)
            } else {
                Ok(apple::gen_minimal_info_plist(
                    package_name,
                    metadata.app_name.clone(),
                    metadata
                        .version_name
                        .clone()
                        .unwrap_or(self.package_version()),
                ))
            }
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
        if self.cargo_package.metadata.is_none() {
            return vec![AppleTarget::X86_64AppleIos];
        };
        let targets = self
            .cargo_package
            .metadata
            .clone()
            .unwrap()
            .apple_build_targets;
        if targets.is_some() && !targets.as_ref().unwrap().is_empty() {
            return targets.unwrap();
        };
        vec![AppleTarget::X86_64AppleIos]
    }

    pub fn apple_res(&self) -> Option<PathBuf> {
        self.cargo_package
            .metadata
            .as_ref()
            .map(|m| m.apple_res.clone())
            .unwrap_or_default()
    }

    pub fn apple_assets(&self) -> Option<PathBuf> {
        self.cargo_package
            .metadata
            .as_ref()
            .map(|m| m.apple_assets.clone())
            .unwrap_or_default()
    }
}
