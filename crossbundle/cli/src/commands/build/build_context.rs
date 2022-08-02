use crate::{error::*, types::*};
use crossbundle_tools::{
    commands::*,
    tools::*,
    types::{
        android_manifest::AndroidManifest, apple_bundle::prelude::InfoPlist,
        update_android_manifest_with_default, AndroidTarget, IosTarget, Profile,
    },
    utils::*,
};
use std::path::PathBuf;

pub struct BuildContext {
    // Paths
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub target_dir: PathBuf,
    // Configurations
    pub manifest: cargo::core::Manifest,
    pub config: CrossbowMetadata,
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
        let crossbow_metadata = if let Some(cargo_metadata) = manifest.custom_metadata() {
            cargo_metadata
                .clone()
                .try_into::<CrossbowMetadata>()
                .map_err(|e| Error::InvalidMetadata(e.into()))?
        } else {
            CrossbowMetadata::default()
        };
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            project_path,
            target_dir,
            config: crossbow_metadata,
            manifest,
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
        if let Some(target_sdk_version) = self
            .config
            .android
            .manifest
            .as_ref()
            .and_then(|x| x.uses_sdk.as_ref().and_then(|u| u.target_sdk_version))
        {
            return target_sdk_version;
        };
        sdk.default_platform()
    }

    /// Get android build targets from cargo manifest
    pub fn android_build_targets(
        &self,
        profile: Profile,
        build_targets: &Vec<AndroidTarget>,
    ) -> Vec<AndroidTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        };
        if profile == Profile::Debug && !self.config.android.debug_build_targets.is_empty() {
            return self.config.android.debug_build_targets.clone();
        };
        if profile == Profile::Release && !self.config.android.release_build_targets.is_empty() {
            return self.config.android.release_build_targets.clone();
        };
        vec![AndroidTarget::Aarch64]
    }

    /// Get android manifest from the path in cargo manifest or generate it with the given configuration
    pub fn gen_android_manifest(
        &self,
        package_name: &str,
        gradle: bool,
    ) -> Result<AndroidManifest> {
        if let Some(manifest_path) = &self.config.android.manifest_path {
            return Ok(android::read_android_manifest(manifest_path)?);
        }
        let mut manifest = if let Some(manifest) = &self.config.android.manifest {
            manifest.clone()
        } else {
            AndroidManifest::default()
        };
        update_android_manifest_with_default(
            &mut manifest,
            self.config.app_name.clone(),
            package_name,
            gradle,
        );
        Ok(manifest)
    }

    /// Get apple build targets from cargo manifest
    pub fn apple_build_targets(
        &self,
        profile: Profile,
        build_targets: &Vec<IosTarget>,
    ) -> Vec<IosTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        }
        if profile == Profile::Debug && self.config.apple.debug_build_targets.is_empty() {
            return self.config.apple.debug_build_targets.clone();
        }
        if profile == Profile::Release && self.config.apple.release_build_targets.is_empty() {
            return self.config.apple.release_build_targets.clone();
        }
        vec![IosTarget::Aarch64Sim]
    }

    /// Get info plist from the path in cargo manifest or generate it with the given configuration
    pub fn gen_info_plist(&self, package_name: &str) -> Result<InfoPlist> {
        if let Some(info_plist_path) = &self.config.apple.info_plist_path {
            return Ok(apple::read_info_plist(info_plist_path)?);
        }
        let info_plist = if let Some(info_plist) = &self.config.apple.info_plist {
            info_plist.clone()
        } else {
            InfoPlist::default()
        };
        // FIXME
        apple::gen_minimal_info_plist(
            package_name,
            self.config.app_name.clone(),
            self.package_version(),
        );
        Ok(info_plist)
    }
}
