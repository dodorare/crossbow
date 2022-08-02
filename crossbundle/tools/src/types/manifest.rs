pub use android_manifest;
pub use apple_bundle;

use android_manifest::*;

/// Updates [`AndroidManifest`](android_manifest::AndroidManifest) with default values.
pub fn update_android_manifest_with_default(
    manifest: &mut AndroidManifest,
    app_name: Option<String>,
    package_name: &str,
    gradle: bool,
) {
    if manifest.package.is_empty() {
        manifest.package = format!("com.crossbow.{}", package_name.replace('-', "_"));
    }
    if manifest.version_name.is_none() {
        manifest.version_name = Some("1".to_owned());
    }
    if manifest.version_code.is_none() {
        manifest.version_code = Some(1_u32);
    }
    if manifest.uses_sdk.is_none() {
        manifest.uses_sdk = Some(UsesSdk {
            min_sdk_version: Some(19),
            target_sdk_version: Some(30),
            max_sdk_version: None,
        });
    }
    if manifest.application.has_code.is_none() {
        manifest.application.has_code = Some(gradle);
    }
    if manifest.application.label.is_none() {
        manifest.application.label = Some(StringResourceOrString::string(
            &app_name.unwrap_or_else(|| "Crossbow".to_owned()),
        ));
    }
    if manifest.application.theme.is_none() {
        manifest.application.theme = Some(Resource::new_with_package(
            "Theme.DeviceDefault.NoActionBar.Fullscreen",
            Some("android".to_string()),
        ));
    }
    if manifest.application.activity.is_empty() {
        manifest.application.activity = vec![Activity::default()];
    }
    if manifest.application.activity.len() == 1 {
        let mut activity = manifest.application.activity.get_mut(0).unwrap();
        if activity.name.is_empty() {
            activity.name = match gradle {
                true => "com.crossbow.game.CrossbowApp".to_string(),
                false => "android.app.NativeActivity".to_string(),
            };
        }
        if activity.resizeable_activity.is_none() {
            activity.resizeable_activity = Some(true);
        }
        if !activity
            .meta_data
            .iter()
            .any(|m| m.name == Some("android.app.lib_name".to_string()))
        {
            activity.meta_data.push(MetaData {
                name: Some("android.app.lib_name".to_string()),
                value: Some(match gradle {
                    true => "crossbow_android".to_string(),
                    false => package_name.replace('-', "_"),
                }),
                ..Default::default()
            });
        }
    }
}

/// Generate android manifest with minimal required tags.
pub fn get_default_android_manifest() -> AndroidManifest {
    let mut manifest = AndroidManifest {
        package: "com.crossbow.minimal".to_owned(),
        ..Default::default()
    };
    update_android_manifest_with_default(
        &mut manifest,
        Some("Minimal".to_owned()),
        "minimal",
        false,
    );
    manifest
}
