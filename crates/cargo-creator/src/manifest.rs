use creator_tools::types::*;
use serde::{Deserialize, Serialize};

pub type Manifest = cargo_toml::Manifest<Metadata>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub android: AndroidMetadata,
    pub apple: AppleMetadata,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AndroidMetadata {
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
    pub build_targets: Option<Vec<String>>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppleMetadata {
    /// Build targets
    pub build_targets: Option<Vec<AppleTarget>>,
    /// Resources directory path relatively to project path
    #[serde(rename = "res")]
    pub resources: Option<String>,
    /// Assets directory path relatively to project path
    pub assets: Option<String>,

    /// Info.plist specification
    #[serde(rename = "info-plist")]
    pub info_plist: InfoPlist,
}
