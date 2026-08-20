use crossbow::Permission;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};

use super::{
    BuildVariableDefinitions, BuildVariables, interpolate_metadata, resolve_definitions,
    resolve_process_environment, take_definitions,
};

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
#[derive(Clone, Deserialize, Serialize, Default)]
pub struct ProjectConfig {
    /// Resolved allow-listed values used by platform configuration templates.
    #[serde(skip)]
    build_variables: BuildVariables,
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

impl fmt::Debug for ProjectConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut metadata = serde_json::to_value(self).map_err(|_| fmt::Error)?;
        if !self.build_variables.is_empty() {
            for pointer in ["/android/manifest", "/apple/info_plist"] {
                if let Some(value) = metadata.pointer_mut(pointer) {
                    *value = serde_json::Value::String("<redacted>".into());
                }
            }
        }
        metadata.fmt(formatter)
    }
}

impl ProjectConfig {
    /// Returns resolved values for platform-document interpolation.
    pub fn build_variables(&self) -> &BuildVariables {
        &self.build_variables
    }

    #[cfg(feature = "android")]
    pub fn android_assets(&self) -> &[PathBuf] {
        if self.android.assets.is_empty() {
            &self.assets
        } else {
            &self.android.assets
        }
    }

    #[cfg(feature = "android")]
    pub fn android_resources(&self) -> &[PathBuf] {
        &self.android.resources
    }

    #[cfg(feature = "android")]
    pub fn android_uses_crossbow_bridge(&self) -> bool {
        !self.permissions.is_empty() || !self.android.plugins.is_empty()
    }

    #[cfg(feature = "apple")]
    pub fn apple_assets(&self) -> &[PathBuf] {
        if self.apple.assets.is_empty() {
            &self.assets
        } else {
            &self.apple.assets
        }
    }

    #[cfg(feature = "apple")]
    pub fn apple_resources(&self) -> &[PathBuf] {
        &self.apple.resources
    }

    /// Resolves every configured filesystem path relative to the selected Cargo package.
    pub fn resolve_paths(&mut self, root: &Path) {
        resolve_paths(&mut self.assets, root);
        resolve_path(&mut self.icon, root);
        #[cfg(feature = "android")]
        {
            resolve_path(&mut self.android.manifest_path, root);
            resolve_paths(&mut self.android.resources, root);
            resolve_paths(&mut self.android.assets, root);
            resolve_paths(&mut self.android.plugins.local, root);
            for project in &mut self.android.plugins.local_projects {
                resolve_path(&mut project.project_dir, root);
            }
        }
        #[cfg(feature = "apple")]
        {
            resolve_path(&mut self.apple.info_plist_path, root);
            resolve_paths(&mut self.apple.resources, root);
            resolve_paths(&mut self.apple.assets, root);
        }
    }
}

fn resolve_path(path: &mut Option<PathBuf>, root: &Path) {
    if let Some(path) = path
        && path.is_relative()
    {
        *path = root.join(&*path);
    }
}

fn resolve_paths(paths: &mut [PathBuf], root: &Path) {
    for path in paths {
        if path.is_relative() {
            *path = root.join(&*path);
        }
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

/// Cargo metadata parsed independently of the process environment.
pub struct ParsedProjectConfig {
    metadata: serde_json::Value,
    build_variables: BuildVariableDefinitions,
}

impl ParsedProjectConfig {
    /// Resolves declared build variables and deserializes the typed project configuration.
    pub fn resolve(mut self) -> anyhow::Result<ProjectConfig> {
        let build_variables = resolve_process_environment(&self.build_variables)?;
        self.finish(build_variables)
    }

    /// Resolves with an injected environment source.
    pub fn resolve_with(
        mut self,
        environment: impl FnMut(&str) -> anyhow::Result<Option<String>>,
    ) -> anyhow::Result<ProjectConfig> {
        let build_variables = resolve_definitions(&self.build_variables, environment)?;
        self.finish(build_variables)
    }

    fn finish(&mut self, build_variables: BuildVariables) -> anyhow::Result<ProjectConfig> {
        interpolate_metadata(&mut self.metadata, &build_variables)?;
        let mut config: ProjectConfig = serde_json::from_value(self.metadata.take())?;
        config.build_variables = build_variables;
        Ok(config)
    }
}

/// Parses project metadata without consulting the process environment.
pub fn parse_project_config(
    mut metadata: serde_json::Value,
) -> anyhow::Result<ParsedProjectConfig> {
    if metadata.is_null() {
        metadata = serde_json::Value::Object(Default::default());
    }
    let build_variables = take_definitions(&mut metadata)?;
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
    Ok(ParsedProjectConfig {
        metadata,
        build_variables,
    })
}

#[cfg(test)]
mod project_config_tests {
    use super::*;

    #[test]
    fn parsing_does_not_read_the_environment() {
        let parsed = parse_project_config(serde_json::json!({
            "build_variables": { "VALUE": { "env": "CROSSBOW_REQUIRED_VALUE" } }
        }))
        .unwrap();

        assert!(
            parsed
                .resolve_with(|_| Ok(None))
                .unwrap_err()
                .to_string()
                .contains("requires environment variable")
        );
    }

    #[test]
    fn resolves_project_paths_once() {
        let mut config = parse_project_config(serde_json::json!({
            "assets": ["assets"],
            "icon": "icon.png",
            "android": { "manifest_path": "AndroidManifest.xml" },
            "apple": { "info_plist_path": "Info.plist" }
        }))
        .unwrap()
        .resolve_with(|_| Ok(None))
        .unwrap();
        let root = Path::new("/project");
        config.resolve_paths(root);
        config.resolve_paths(root);

        assert_eq!(config.assets, [root.join("assets")]);
        assert_eq!(
            config.icon.as_deref(),
            Some(root.join("icon.png").as_path())
        );
        #[cfg(feature = "android")]
        assert_eq!(
            config.android.manifest_path.as_deref(),
            Some(root.join("AndroidManifest.xml").as_path())
        );
        #[cfg(feature = "apple")]
        assert_eq!(
            config.apple.info_plist_path.as_deref(),
            Some(root.join("Info.plist").as_path())
        );
    }
}

#[cfg(all(test, feature = "android"))]
mod android_config_tests {
    use super::*;

    #[test]
    fn rejects_removed_compiler_configuration_with_migration_help() {
        let error = parse_project_config(serde_json::json!({
            "android": { "rust_compiler": "quad" }
        }))
        .err()
        .expect("removed configuration must be rejected")
        .to_string();
        assert!(error.contains("runtime = \"miniquad\""));

        let error = parse_project_config(serde_json::json!({
            "android": { "app_wrapper": "ndk-glue" }
        }))
        .err()
        .expect("removed configuration must be rejected")
        .to_string();
        assert!(error.contains("app_wrapper` was removed"));
    }

    #[test]
    fn accepts_cargo_metadata_null_for_unconfigured_packages() {
        let metadata = parse_project_config(serde_json::Value::Null)
            .unwrap()
            .resolve()
            .unwrap();
        assert!(metadata.app_name.is_none());
        assert_eq!(metadata.android.runtime, AndroidRuntime::NativeActivity);
    }

    #[test]
    fn resolves_inline_android_metadata_before_typed_deserialization() {
        let metadata = parse_project_config(serde_json::json!({
            "build_variables": {
                "CODE": {
                    "env": "CROSSBOW_TEST_UNSET_INLINE_ANDROID_CODE",
                    "type": "integer",
                    "default": 73
                },
                "LABEL": {
                    "env": "CROSSBOW_TEST_UNSET_INLINE_ANDROID_LABEL",
                    "default": "Preview"
                },
                "ENABLED": {
                    "env": "CROSSBOW_TEST_UNSET_INLINE_ANDROID_ENABLED",
                    "type": "boolean",
                    "default": true
                }
            },
            "android": {
                "manifest": {
                    "version_code": "{{crossbow.CODE}}",
                    "application": {
                        "label": "{{crossbow.LABEL}}",
                        "has_code": "{{crossbow.ENABLED}}",
                        "activity": [{
                            "name": ".MainActivity",
                            "intent_filter": [{ "auto_verify": "{{crossbow.ENABLED}}" }]
                        }]
                    }
                }
            }
        }))
        .unwrap()
        .resolve_with(|_| Ok(None))
        .unwrap();
        assert!(!format!("{metadata:?}").contains("Preview"));
        let manifest = metadata.android.manifest.unwrap();
        assert_eq!(manifest.version_code, Some(73));
        assert_eq!(manifest.application.label.unwrap().to_string(), "Preview");
        assert_eq!(
            manifest.application.has_code,
            Some(android_manifest::VarOrBool::Bool(true))
        );
        assert_eq!(
            manifest.application.activity[0].intent_filter[0].auto_verify,
            Some(true)
        );
    }
}

#[cfg(all(test, feature = "apple"))]
mod apple_build_variable_tests {
    use super::*;

    #[test]
    fn resolves_inline_apple_metadata() {
        let metadata = parse_project_config(serde_json::json!({
            "build_variables": {
                "BUNDLE_ID": {
                    "env": "CROSSBOW_TEST_UNSET_INLINE_APPLE_BUNDLE_ID",
                    "default": "dev.crossbow.preview"
                }
            },
            "apple": {
                "info_plist": {
                    "CFBundleIdentifier": "{{crossbow.BUNDLE_ID}}"
                }
            }
        }))
        .unwrap()
        .resolve_with(|_| Ok(None))
        .unwrap();
        assert!(!format!("{metadata:?}").contains("dev.crossbow.preview"));
        assert_eq!(
            metadata
                .apple
                .info_plist
                .unwrap()
                .identification
                .bundle_identifier,
            "dev.crossbow.preview"
        );
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
