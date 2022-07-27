use android_manifest::*;

/// Generates [`AndroidManifest`](android_manifest::AndroidManifest) with
/// given changes
pub fn gen_android_manifest(
    app_id: Option<String>,
    package_name: String,
    app_name: Option<String>,
    version_name: String,
    version_code: u32,
    min_sdk_version: Option<u32>,
    target_sdk_version: u32,
    max_sdk_version: Option<u32>,
    icon: Option<String>,
    debuggable: bool,
    gradle: bool,
) -> AndroidManifest {
    AndroidManifest {
        package: app_id
            .unwrap_or(format!("com.rust.{}", package_name))
            .replace('-', "_"),
        version_name: Some(version_name),
        version_code: Some(version_code),
        uses_sdk: Some(UsesSdk {
            min_sdk_version: Some(min_sdk_version.unwrap_or(9)),
            target_sdk_version: Some(target_sdk_version),
            max_sdk_version,
        }),
        application: Application {
            has_code: Some(gradle),
            label: Some(StringResourceOrString::string(
                app_name.as_ref().unwrap_or(&package_name),
            )),
            debuggable: Some(debuggable),
            icon: icon.map(|i| MipmapOrDrawableResource::mipmap(&i, None)),
            theme: Some(Resource::new_with_package(
                "Theme.DeviceDefault.NoActionBar.Fullscreen",
                Some("android".to_string()),
            )),
            activity: vec![Activity {
                name: match gradle {
                    true => "com.crossbow.game.CrossbowApp".to_string(),
                    false => "android.app.NativeActivity".to_string(),
                },
                resizeable_activity: Some(true),
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
                        false => package_name.replace('-', "_"),
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
pub fn gen_min_android_manifest(
    version_name: &str,
    version_code: u32,
    package_name: &str,
) -> AndroidManifest {
    AndroidManifest {
        package: (format!("com.rust.{}", package_name)).replace('-', "_"),
        version_name: Some(version_name.to_string()),
        version_code: Some(version_code),
        application: Application {
            has_code: Some(false),
            label: Some(StringResourceOrString::string(package_name)),
            activity: vec![Activity {
                name: "android.app.NativeActivity".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    }
}
