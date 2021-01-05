use creator_tools::types::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AndroidMetadata {
    /// Resources directory path relatively to project path.
    #[serde(rename = "res")]
    pub resources: Option<PathBuf>,
    /// Assets directory path relatively to project path.
    pub assets: Option<PathBuf>,
    /// Build targets.
    pub build_targets: Option<Vec<AndroidTarget>>,
    /// Android manifest.
    pub manifest: AndroidManifestConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AndroidManifestConfig {
    pub apk_label: Option<String>,
    pub target_sdk_version: Option<u32>,
    pub min_sdk_version: Option<u32>,
    pub icon: Option<String>,
    pub fullscreen: Option<bool>,
    pub orientation: Option<String>,
    pub opengles_version: Option<(u8, u8)>,
    pub feature: Option<Vec<FeatureConfig>>,
    pub permission: Option<Vec<PermissionConfig>>,
    pub intent_filter: Option<Vec<IntentFilterConfig>>,
    pub application_metadatas: Option<Vec<ApplicationMetadataConfig>>,
    pub activity_metadatas: Option<Vec<ActivityMetadataConfig>>,
}

impl AndroidManifestConfig {
    pub fn into_android_manifest(
        self,
        target: &Target,
        profile: Profile,
        package_name: String,
        package_version: String,
        target_sdk_version: u32,
    ) -> AndroidManifest {
        let pkg_name = match target {
            Target::Lib => format!("rust.{}", package_name.replace("-", "_")),
            Target::Example(_) => format!("rust.example.{}", package_name.replace("-", "_")),
            _ => panic!(),
        };
        let package_label = self
            .apk_label
            .as_deref()
            .unwrap_or(&package_name)
            .to_string();
        let version_code = VersionCode::from_semver(&package_version)
            .unwrap()
            .to_code(1);
        let version_name = package_version;
        let min_sdk_version = self.min_sdk_version.unwrap_or(23);
        let opengles_version = self.opengles_version.unwrap_or((3, 1));
        let features = self
            .feature
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let permissions = self
            .permission
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let intent_filters = self
            .intent_filter
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let application_metadatas = self
            .application_metadatas
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let activity_metadatas = self
            .activity_metadatas
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        AndroidManifest {
            package_name: pkg_name,
            package_label,
            version_name,
            version_code,
            split: None,
            target_name: package_name.replace("-", "_"),
            debuggable: profile == Profile::Debug,
            target_sdk_version,
            min_sdk_version,
            opengles_version,
            features,
            permissions,
            intent_filters,
            icon: self.icon.clone(),
            fullscreen: self.fullscreen.unwrap_or(false),
            orientation: self.orientation,
            application_metadatas,
            activity_metadatas,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeatureConfig {
    name: String,
    required: Option<bool>,
}

impl From<FeatureConfig> for Feature {
    fn from(config: FeatureConfig) -> Self {
        Self {
            name: config.name,
            required: config.required.unwrap_or(true),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionConfig {
    name: String,
    max_sdk_version: Option<u32>,
}

impl From<PermissionConfig> for Permission {
    fn from(config: PermissionConfig) -> Self {
        Self {
            name: config.name,
            max_sdk_version: config.max_sdk_version,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntentFilterConfigData {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub prefix: Option<String>,
}

impl From<IntentFilterConfigData> for IntentFilterData {
    fn from(config: IntentFilterConfigData) -> Self {
        Self {
            scheme: config.scheme,
            host: config.host,
            prefix: config.prefix,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntentFilterConfig {
    name: String,
    data: Vec<IntentFilterConfigData>,
    categories: Vec<String>,
}

impl From<IntentFilterConfig> for IntentFilter {
    fn from(config: IntentFilterConfig) -> Self {
        Self {
            name: config.name,
            data: config
                .data
                .into_iter()
                .map(IntentFilterData::from)
                .rev()
                .collect(),
            categories: config.categories,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationMetadataConfig {
    name: String,
    value: String,
}

impl From<ApplicationMetadataConfig> for ApplicationMetadata {
    fn from(config: ApplicationMetadataConfig) -> Self {
        Self {
            name: config.name,
            value: config.value,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityMetadataConfig {
    name: String,
    value: String,
}

impl From<ActivityMetadataConfig> for ActivityMetadata {
    fn from(config: ActivityMetadataConfig) -> Self {
        Self {
            name: config.name,
            value: config.value,
        }
    }
}
