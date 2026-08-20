pub use android_manifest;

use super::AndroidStrategy;
use android_manifest::*;

pub const DEFAULT_ANDROID_MIN_SDK: u32 = 23;
pub const DEFAULT_ANDROID_TARGET_SDK: u32 = 36;

/// Adapts `android-manifest`'s JSON representation for its own deserializer. Its `VarOrBool`
/// serializer emits JSON booleans while its deserializer currently requests strings; the one
/// native Boolean field must remain a Boolean.
pub fn normalize_android_manifest_json(value: &mut serde_json::Value) {
    fn normalize(value: &mut serde_json::Value, field: Option<&str>) {
        match value {
            serde_json::Value::Bool(boolean) if field != Some("auto_verify") => {
                *value = serde_json::Value::String(boolean.to_string());
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    normalize(value, None);
                }
            }
            serde_json::Value::Object(values) => {
                for (field, value) in values {
                    normalize(value, Some(field));
                }
            }
            _ => {}
        }
    }
    normalize(value, None);
}

/// Returns the Activity that handles the manifest's launcher intent.
pub fn launcher_activity(manifest: &AndroidManifest) -> Option<&str> {
    manifest
        .application
        .activity
        .iter()
        .find(|activity| !activity.name.is_empty() && handles_launcher(&activity.intent_filter))
        .map(|activity| activity.name.as_str())
        .or_else(|| {
            manifest
                .application
                .activity_alias
                .iter()
                .find(|alias| handles_launcher(&alias.intent_filter))
                .and_then(|alias| alias.name.as_deref().filter(|name| !name.is_empty()))
        })
}

fn handles_launcher(filters: &[IntentFilter]) -> bool {
    filters.iter().any(|filter| {
        filter
            .action
            .iter()
            .any(|action| action.name.as_deref() == Some("android.intent.action.MAIN"))
            && filter.category.iter().any(|category| {
                category.name.as_deref() == Some("android.intent.category.LAUNCHER")
            })
    })
}

/// Updates [`AndroidManifest`](android_manifest::AndroidManifest) with default values.
pub fn update_android_manifest_with_default(
    manifest: &mut AndroidManifest,
    app_name: Option<String>,
    library_name: &str,
    strategy: super::AndroidStrategy,
    runtime: super::AndroidRuntime,
    crossbow_bridge: bool,
) {
    if manifest.package.as_ref().is_none_or(String::is_empty) {
        manifest.package = Some(format!("com.crossbow.{}", library_name.replace('-', "_")));
    }
    if manifest.version_name.is_none() {
        manifest.version_name = Some("0.1.0".to_owned());
    }
    if manifest.version_code.is_none() {
        manifest.version_code = Some(1_u32);
    }
    if manifest.uses_sdk.is_none() {
        manifest.uses_sdk = Some(UsesSdk {
            min_sdk_version: Some(DEFAULT_ANDROID_MIN_SDK),
            target_sdk_version: Some(DEFAULT_ANDROID_TARGET_SDK),
            max_sdk_version: None,
        });
    }
    if manifest.application.has_code.is_none() {
        manifest.application.has_code =
            VarOrBool::Bool(strategy == AndroidStrategy::GradleApk).into();
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
        let activity = manifest.application.activity.get_mut(0).unwrap();
        if activity.name.is_empty() {
            activity.name = match (strategy, runtime) {
                (AndroidStrategy::GradleApk, super::AndroidRuntime::Miniquad)
                    if crossbow_bridge =>
                {
                    format!("{}.CrossbowApp", manifest.package.as_deref().unwrap())
                }
                (AndroidStrategy::GradleApk, super::AndroidRuntime::Miniquad) => {
                    format!("{}.MainActivity", manifest.package.as_deref().unwrap())
                }
                (AndroidStrategy::GradleApk, super::AndroidRuntime::NativeActivity)
                    if crossbow_bridge =>
                {
                    "com.crossbow.game.CrossbowApp".to_string()
                }
                _ => "android.app.NativeActivity".to_string(),
            };
        }
        if activity.resizeable_activity.is_none() {
            activity.resizeable_activity = VarOrBool::Bool(true).into();
        }
        if activity.exported.is_none() {
            activity.exported = VarOrBool::Bool(true).into();
        }
        if runtime == super::AndroidRuntime::Miniquad && activity.config_changes.is_empty() {
            activity.config_changes = vec![
                ConfigChanges::Orientation,
                ConfigChanges::KeyboardHidden,
                ConfigChanges::ScreenSize,
            ]
            .into();
        }
        if runtime == super::AndroidRuntime::NativeActivity
            && !activity
                .meta_data
                .iter()
                .any(|metadata| metadata.name.as_deref() == Some("android.app.lib_name"))
        {
            activity.meta_data.push(MetaData {
                name: Some("android.app.lib_name".to_string()),
                value: Some(library_name.replace('-', "_")),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn miniquad_defaults_to_its_activity_and_handles_rotation() {
        let mut manifest = AndroidManifest::default();
        update_android_manifest_with_default(
            &mut manifest,
            None,
            "my_game",
            AndroidStrategy::GradleApk,
            super::super::AndroidRuntime::Miniquad,
            false,
        );

        assert_eq!(
            launcher_activity(&manifest),
            Some("com.crossbow.my_game.MainActivity")
        );
        assert_eq!(
            manifest.application.activity[0].config_changes.vec().len(),
            3
        );
        assert!(manifest.application.activity[0].meta_data.is_empty());
    }

    #[test]
    fn finds_launcher_among_multiple_activities() {
        let mut manifest = AndroidManifest::default();
        manifest.application.activity = vec![Activity::default(), Activity::default()];
        manifest.application.activity[0].name = ".SettingsActivity".into();
        manifest.application.activity[1].name = ".GameActivity".into();
        manifest.application.activity[1].intent_filter = vec![IntentFilter {
            action: vec![Action {
                name: Some("android.intent.action.MAIN".into()),
            }],
            category: vec![Category {
                name: Some("android.intent.category.LAUNCHER".into()),
            }],
            ..Default::default()
        }];

        assert_eq!(launcher_activity(&manifest), Some(".GameActivity"));
    }

    #[test]
    fn native_activity_does_not_require_the_java_bridge() {
        let mut manifest = AndroidManifest::default();
        update_android_manifest_with_default(
            &mut manifest,
            None,
            "my_game",
            AndroidStrategy::GradleApk,
            super::super::AndroidRuntime::NativeActivity,
            false,
        );

        assert_eq!(
            launcher_activity(&manifest),
            Some("android.app.NativeActivity")
        );
        assert_eq!(manifest.application.activity[0].meta_data.len(), 1);
    }

    #[test]
    fn json_round_trip_handles_manifest_boolean_wrappers() {
        let manifest = android_manifest::from_str(
            r#"<manifest xmlns:android="http://schemas.android.com/apk/res/android"
                package="dev.crossbow.example">
                <application android:hasCode="true">
                    <activity android:name=".MainActivity" android:exported="true">
                        <intent-filter android:autoVerify="true" />
                    </activity>
                </application>
            </manifest>"#,
        )
        .unwrap();
        let mut value = serde_json::to_value(&manifest).unwrap();
        normalize_android_manifest_json(&mut value);
        let round_trip: AndroidManifest = serde_json::from_value(value).unwrap();
        assert_eq!(round_trip, manifest);
    }
}
