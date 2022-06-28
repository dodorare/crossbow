use android_manifest::*;

#[derive(Default)]
pub struct GenAndroidManifest {
    pub app_id: Option<String>,
    pub package_name: String,
    pub app_name: Option<String>,
    pub version_name: String,
    pub version_code: u32,
    pub min_sdk_version: Option<u32>,
    pub target_sdk_version: u32,
    pub max_sdk_version: Option<u32>,
    pub icon: Option<String>,
    pub debuggable: bool,
    pub permissions_sdk_23: Option<Vec<UsesPermissionSdk23>>,
    pub permissions: Option<Vec<UsesPermission>>,
    pub features: Option<Vec<UsesFeature>>,
    pub service: Option<Vec<Service>>,
}

impl GenAndroidManifest {
    /// Generates [`AndroidManifest`](android_manifest::AndroidManifest) with
    /// given changes
    pub fn gen_android_manifest(&self, gradle: bool) -> AndroidManifest {
        AndroidManifest {
            package: self
                .app_id
                .clone()
                .unwrap_or(format!("com.rust.{}", self.package_name))
                .replace('-', "_"),
            version_name: Some(self.version_name.clone()),
            version_code: Some(self.version_code),
            uses_sdk: Some(UsesSdk {
                min_sdk_version: Some(self.min_sdk_version.unwrap_or(9)),
                target_sdk_version: Some(self.target_sdk_version),
                max_sdk_version: self.max_sdk_version,
            }),
            uses_permission_sdk_23: self.permissions_sdk_23.clone().unwrap_or_default(),
            uses_permission: self.permissions.clone().unwrap_or_default(),
            uses_feature: self.features.clone().unwrap_or_default(),
            application: Application {
                has_code: Some(gradle),
                label: Some(StringResourceOrString::string(
                    self.app_name
                        .as_ref()
                        .unwrap_or(&self.package_name.to_owned()),
                )),
                debuggable: Some(self.debuggable),
                icon: self
                    .icon
                    .clone()
                    .map(|i| MipmapOrDrawableResource::mipmap(&i, None)),
                theme: Some(Resource::new_with_package(
                    "Theme.DeviceDefault.NoActionBar.Fullscreen",
                    Some("android".to_string()),
                )),
                service: self.service.clone().unwrap_or_default(),
                activity: vec![Activity {
                    name: match gradle {
                        true => "android.app.NativeActivity".to_string(),
                        false => ".CrossbowApp".to_string(),
                    },
                    resizeable_activity: Some(true),
                    label: Some(StringResourceOrString::string(
                        self.app_name
                            .as_ref()
                            .unwrap_or(&self.package_name.to_owned()),
                    )),
                    config_changes: vec![
                        ConfigChanges::Orientation,
                        ConfigChanges::KeyboardHidden,
                        ConfigChanges::ScreenSize,
                    ]
                    .into(),
                    meta_data: vec![MetaData {
                        name: Some("android.app.lib_name".to_string()),
                        value: Some(match gradle {
                            true => "crossbow_android".to_string(),
                            false => self.package_name.replace('-', "_"),
                        }),
                        ..Default::default()
                    }],
                    intent_filter: vec![IntentFilter {
                        action: vec![Action {
                            name: Some("android.intent.action.MAIN".to_string()),
                        }],
                        category: vec![Category {
                            name: Some("android.intent.category.LAUNCHER".to_string()),
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Generate android manifest with minimal required tags.
    pub fn gen_min_android_manifest(&self) -> AndroidManifest {
        AndroidManifest {
            package: self
                .app_id
                .clone()
                .unwrap_or(format!("com.rust.{}", self.package_name))
                .replace('-', "_"),
            version_name: Some(self.version_name.clone()),
            version_code: Some(self.version_code),
            application: Application {
                has_code: Some(false),
                label: Some(StringResourceOrString::string(
                    self.app_name
                        .as_ref()
                        .unwrap_or(&self.package_name.to_owned()),
                )),
                activity: vec![Activity {
                    name: "android.app.NativeActivity".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
