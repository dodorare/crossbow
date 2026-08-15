use crossbow::Permission;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(feature = "android")]
use crate::types::{AndroidRustCompiler, AndroidTarget, android_manifest::AndroidManifest};
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
    #[serde(default, alias = "app_wrapper")]
    pub rust_compiler: AndroidRustCompiler,
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
