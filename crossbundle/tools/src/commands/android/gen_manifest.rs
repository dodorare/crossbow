use android_manifest::*;

/// Generates minimal [`AndroidManifest`](android_manifest::AndroidManifest) with given
/// changes
pub fn gen_minimal_android_manifest(
    app_id: Option<String>,
    package_name: &str,
    app_name: Option<String>,
    version_name: String,
    version_code: Option<u32>,
    min_sdk_version: Option<u32>,
    target_sdk_version: u32,
    max_sdk_version: Option<u32>,
    icon: Option<String>,
    debuggable: bool,
    permissions_sdk_23: Option<Vec<UsesPermissionSdk23>>,
    permissions: Option<Vec<UsesPermission>>,
    features: Option<Vec<UsesFeature>>,
    service: Option<Vec<Service>>,
) -> AndroidManifest {
    AndroidManifest {
        package: app_id.unwrap_or(format!("com.rust.{}", package_name.replace('-', "_"))),
        version_name: Some(version_name),
        version_code,
        uses_sdk: Some(UsesSdk {
            min_sdk_version: Some(min_sdk_version.unwrap_or(9)),
            target_sdk_version: Some(target_sdk_version),
            max_sdk_version,
        }),
        uses_permission_sdk_23: permissions_sdk_23.unwrap_or_default(),
        uses_permission: permissions.unwrap_or_default(),
        uses_feature: features.unwrap_or_default(),
        application: Application {
            has_code: Some(false),
            label: Some(StringResourceOrString::string(
                app_name.as_ref().unwrap_or(&package_name.to_owned()),
            )),
            debuggable: Some(debuggable),
            icon: icon.map(|i| MipmapOrDrawableResource::mipmap(&i, None)),
            theme: Some(Resource::new_with_package(
                "Theme.DeviceDefault.NoActionBar.Fullscreen",
                Some("android".to_string()),
            )),
            activity: vec![Activity {
                name: "android.app.NativeActivity".to_string(),
                resizeable_activity: Some(true),
                label: Some(StringResourceOrString::string(
                    app_name.as_ref().unwrap_or(&package_name.to_owned()),
                )),
                config_changes: vec![
                    ConfigChanges::Orientation,
                    ConfigChanges::KeyboardHidden,
                    ConfigChanges::ScreenSize,
                ]
                .into(),
                meta_data: vec![MetaData {
                    name: Some("android.app.lib_name".to_string()),
                    value: Some(package_name.replace('-', "_")),
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
            service: service.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    }
}
