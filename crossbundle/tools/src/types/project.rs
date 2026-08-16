use crossbow::Permission;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(feature = "android")]
use crate::types::{AndroidRuntime, AndroidTarget, android_manifest::AndroidManifest};
#[cfg(feature = "apple")]
use crate::types::{IosTarget, apple_bundle::prelude::InfoPlist};

/// Typed Android plugin configuration shared by builds and cross-platform diagnostics.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AndroidGradlePlugins {
    /// Android Gradle local plugin archives.
    #[serde(default, rename = "plugins_local")]
    pub local: Vec<PathBuf>,
    /// Android Gradle remote plugin coordinates.
    #[serde(default, rename = "plugins_remote")]
    pub remote: Vec<String>,
    /// Android Gradle custom Maven repositories.
    #[serde(default, rename = "plugins_maven_repos")]
    pub maven_repos: Vec<String>,
    /// Android Gradle local plugin projects.
    #[serde(default, rename = "plugins_local_projects")]
    pub local_projects: Vec<GradleDependencyProject>,
}

impl AndroidGradlePlugins {
    pub fn is_empty(&self) -> bool {
        self.local.is_empty()
            && self.remote.is_empty()
            && self.maven_repos.is_empty()
            && self.local_projects.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GradleDependencyProject {
    pub(crate) include: String,
    #[serde(default)]
    pub(crate) dont_implement: bool,
    pub(crate) project_dir: Option<PathBuf>,
}

impl GradleDependencyProject {
    pub fn include(&self) -> &str {
        &self.include
    }

    pub fn project_dir(&self) -> Option<&Path> {
        self.project_dir.as_deref()
    }
}

/// Typed `package.metadata` model shared by builds and project diagnostics.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CrossbowMetadata {
    pub app_name: Option<String>,
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    #[serde(default)]
    pub permissions: Vec<Permission>,
    pub icon: Option<PathBuf>,
    #[cfg(feature = "android")]
    #[serde(default)]
    pub android: AndroidConfig,
    #[cfg(feature = "apple")]
    #[serde(default)]
    pub apple: AppleConfig,
}

impl CrossbowMetadata {
    #[cfg(feature = "android")]
    pub fn get_android_assets(&self) -> &[PathBuf] {
        if self.android.assets.is_empty() {
            &self.assets
        } else {
            &self.android.assets
        }
    }

    #[cfg(feature = "android")]
    pub fn get_android_resources(&self) -> &[PathBuf] {
        &self.android.resources
    }

    #[cfg(feature = "android")]
    pub fn android_uses_crossbow_bridge(&self) -> bool {
        !self.permissions.is_empty() || !self.android.plugins.is_empty()
    }

    #[cfg(feature = "apple")]
    pub fn get_apple_assets(&self) -> &[PathBuf] {
        if self.apple.assets.is_empty() {
            &self.assets
        } else {
            &self.apple.assets
        }
    }

    #[cfg(feature = "apple")]
    pub fn get_apple_resources(&self) -> &[PathBuf] {
        &self.apple.resources
    }
}

#[cfg(feature = "android")]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AndroidConfig {
    #[serde(default)]
    pub runtime: AndroidRuntime,
    pub manifest: Option<AndroidManifest>,
    pub manifest_path: Option<PathBuf>,
    #[serde(default)]
    pub resources: Vec<PathBuf>,
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    #[serde(default)]
    pub debug_build_targets: Vec<AndroidTarget>,
    #[serde(default)]
    pub release_build_targets: Vec<AndroidTarget>,
    #[serde(flatten)]
    pub plugins: AndroidGradlePlugins,
}

pub fn deserialize_crossbow_metadata(
    metadata: serde_json::Value,
) -> anyhow::Result<CrossbowMetadata> {
    if metadata.is_null() {
        return Ok(CrossbowMetadata::default());
    }
    #[cfg(feature = "android")]
    if let Some(android) = metadata.get("android") {
        if android.get("rust_compiler").is_some() {
            anyhow::bail!(
                "`package.metadata.android.rust_compiler` was removed in Crossbow 0.3; Cargo is now always used. Remove the key, or replace `rust_compiler = \"quad\"` with `runtime = \"miniquad\"`"
            );
        }
        if android.get("app_wrapper").is_some() {
            anyhow::bail!(
                "`package.metadata.android.app_wrapper` was removed in Crossbow 0.3; remove it and use `runtime = \"miniquad\"` only for Miniquad/Macroquad applications"
            );
        }
    }
    Ok(serde_json::from_value(metadata)?)
}

#[cfg(all(test, feature = "android"))]
mod android_config_tests {
    use super::*;

    #[test]
    fn rejects_removed_compiler_configuration_with_migration_help() {
        let error = deserialize_crossbow_metadata(serde_json::json!({
            "android": { "rust_compiler": "quad" }
        }))
        .unwrap_err()
        .to_string();
        assert!(error.contains("runtime = \"miniquad\""));

        let error = deserialize_crossbow_metadata(serde_json::json!({
            "android": { "app_wrapper": "ndk-glue" }
        }))
        .unwrap_err()
        .to_string();
        assert!(error.contains("app_wrapper` was removed"));
    }

    #[test]
    fn accepts_cargo_metadata_null_for_unconfigured_packages() {
        let metadata = deserialize_crossbow_metadata(serde_json::Value::Null).unwrap();
        assert!(metadata.app_name.is_none());
        assert_eq!(metadata.android.runtime, AndroidRuntime::NativeActivity);
    }
}

#[cfg(feature = "apple")]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AppleConfig {
    pub info_plist: Option<InfoPlist>,
    pub info_plist_path: Option<PathBuf>,
    #[serde(default)]
    pub resources: Vec<PathBuf>,
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    #[serde(default)]
    pub debug_build_targets: Vec<IosTarget>,
    #[serde(default)]
    pub release_build_targets: Vec<IosTarget>,
}
