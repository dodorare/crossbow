use android_manifest::*;
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
    pub manifest: AndroidManifest,
}

    pub fn into_android_manifest(
        package_name: String,
        target_sdk_version: i32,
    ) -> AndroidManifest {
        AndroidManifest{
            package: "com.example.toggletest".to_string(),
            shared_user_id: Some("com.example".to_string()),
            target_sandbox_version: None,
            shared_user_label: None,
            version_code: None,
            version_name: None,
            install_location:  Some(InstallLocation::Auto),
            application: Application {
                allow_backup: Some(true),
                label: Some(StringResourceOrString::string("app_name1")),
                activity: vec![Activity {
                    name: "com.example.toggletest.MainActivity".to_string(),
                    label: Some(StringResourceOrString::string("app_name2")),
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
            uses_sdk: Some(UsesSdk {
                min_sdk_version: Some(29),
                target_sdk_version: Some(29),
                ..Default::default()
            }),
            compatible_screens: None,
            instrumentation: vec![Instrumentation {
                functional_test: Some(true),
                handle_profiling: Some(true),
                icon: None,
                label: Some(StringResourceOrString::string("app_name3")),
                name: "com.example.toggletest.MainActivity".to_string(),
                target_package: Some("com.example.toggletest.MainActivity".to_string()),
                target_processes: Some("com.example.toggletest.MainActivity".to_string()),
            }],
            permission:  vec![Permission {
                name: Some("org.domokit.gcm.permission.C2D_MESSAGE".to_string()),
                protection_level: Some(ProtectionLevel::Signature),
                ..Default::default()
            }],
            permission_group: vec![PermissionGroup {
                description: None,
                icon:  None,
                label: Some(StringResourceOrString::string("app_name4")),
                name:  Some("com.example.toggletest.MainActivity".to_string()),
            }],
            permission_tree: vec![PermissionTree {
                icon: None,
                label: Some(StringResourceOrString::string("app_name5")),
                name:  Some("com.example.toggletest.MainActivity".to_string()),
            }],
            supports_gl_texture: vec![SupportsGlTexture {
                name: Some(SupportsGlTextureName::GL_AMD_compressed_3DC_texture),
            }],
            supports_screens: vec![SupportsScreens{
                resizeable: Some(true),
                small_screens: Some(false),
                normal_screens: Some(false),
                large_screens: Some(true),
                xlarge_screens: Some(false),
                any_density: Some(true),
                requires_smallest_width_dp: Some("600".to_string()),
                compatible_width_limit_dp: Some("600".to_string()),
                largest_width_limit_dp: Some("600".to_string()),
            }],
            uses_configuration: vec![UsesConfiguration {
                req_five_way_nav: Some(true),
                req_hard_keyboard: Some(false),
                req_keyboard_type: Some(ReqKeyboardType::Undefined),
                req_navigation: Some(ReqNavigation::Dpad),
                req_touch_screen: Some(ReqTouchScreen::Stylus),
            }],
            uses_feature: vec![UsesFeature {
                name: Some("com.example.toggletest.MainActiy".to_string()),
                required: Some(false),
                gl_es_version: None,
            }],
            uses_permission: vec![
            UsesPermission {
                name: Some("android.permission.INTERNET".to_string()),
                ..Default::default()
            },
            UsesPermission {
                name: Some("android.permission.WAKE_LOCK".to_string()),
                ..Default::default()
            },
            UsesPermission {
                name: Some("com.google.android.c2dm.permission.RECEIVE".to_string()),
                ..Default::default()
            },
            UsesPermission {
                name: Some("org.domokit.gcm.permission.C2D_MESSAGE".to_string()),
                ..Default::default()
            },
        ],
        uses_permission_sdk_23: vec![UsesPermissionSdk23 {
            name: Some("com.google.android.c2dm.permission.RECEIVE".to_string()),
            max_sdk_version: Some(32),
        }],
            queries: None ,
        }
    }
