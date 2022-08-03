pub use android_manifest;
pub use apple_bundle;

use android_manifest::*;
use apple_bundle::prelude::*;

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
        manifest.version_name = Some("0.1.0".to_owned());
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
        if activity.intent_filter.is_empty() {
            activity.intent_filter = vec![IntentFilter {
                action: vec![Action {
                    name: Some("android.intent.action.MAIN".to_string()),
                }],
                category: vec![Category {
                    name: Some("android.intent.category.LAUNCHER".to_string()),
                }],
                ..Default::default()
            }];
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

/// Updates [`InfoPlist`](InfoPlist) with default values.
pub fn update_info_plist_with_default(
    props: &mut InfoPlist,
    package_name: &str,
    app_name: Option<String>,
) {
    if props.launch.bundle_executable.is_none() {
        props.launch.bundle_executable = Some(package_name.to_owned());
    }
    if props.localization.bundle_development_region.is_none() {
        props.localization.bundle_development_region = Some("en".to_owned());
    }
    if props.identification.bundle_identifier.is_empty() {
        props.identification.bundle_identifier =
            format!("com.crossbow.{}", package_name.replace('-', "_"));
    }
    if props.bundle_version.bundle_version.is_none() {
        props.bundle_version.bundle_version = Some("0.1.0".to_owned());
    }
    if props
        .bundle_version
        .bundle_info_dictionary_version
        .is_none()
    {
        props.bundle_version.bundle_info_dictionary_version = Some("0.1.0".to_owned());
    }
    if props.bundle_version.bundle_short_version_string.is_none() {
        props.bundle_version.bundle_short_version_string = Some("0.1.0".to_owned());
    }
    if props.naming.bundle_name.is_none() {
        props.naming.bundle_name = Some(app_name.unwrap_or_else(|| package_name.to_owned()));
    }
    if props.categorization.bundle_package_type.is_none() {
        props.categorization.bundle_package_type = Some("APPL".to_owned());
    }
    if props.launch_interface.launch_storyboard_name.is_none() {
        props.launch_interface.launch_storyboard_name = Some("LaunchScreen".to_owned());
    }
}
