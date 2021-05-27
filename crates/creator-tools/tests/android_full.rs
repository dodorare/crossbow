use android_manifest::*;
use std::{clone::Clone, path::Path};
use creator_tools::{
    commands::{android::*, gen_minimal_project},
    deps::{AndroidNdk, AndroidSdk},
    types::*,
};

pub fn get_android_manifest(
    project_name: &str,
    target_sdk_version: u32,
) -> AndroidManifest {
    AndroidManifest {
        package: "com.example.toggletest".to_string(),
        shared_user_id: Some("com.example".to_string()),
        target_sandbox_version: Some("2".to_string()),
        version_code: Some("1".to_string()),
        version_name: Some("1.0".to_string()),
        install_location: Some(InstallLocation::Auto),
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
        shared_user_label: None,
        uses_sdk: Some(UsesSdk {
            min_sdk_version: Some(29),
            target_sdk_version: Some(29),
            ..Default::default()
        }),
        compatible_screens: Some(CompatibleScreens {
            screen:  vec![Screen {
                screen_size: ScreenSize::Normal,
                screen_density: "mdpi".to_string(),
            }], 
        }),
        instrumentation: vec![Instrumentation {
            functional_test: Some(true),
            handle_profiling: Some(true),
            icon: None,
            label: Some(StringResourceOrString::string("app_name3")),
            name: "com.example.toggletest.MainActivity".to_string(),
            target_package: Some("com.example.toggletest.MainActivity".to_string()),
            target_processes: Some("com.example.toggletest.MainActivity".to_string()),
        }],
        permission: vec![Permission {
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

#[test]
fn test_android_full() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(&dir).unwrap();

    // Create dependencies
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

    // Compile rust lib for android
    let target_sdk_version = 29;
    let profile = Profile::Release;
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    compile_rust_for_android(
        &ndk,
        Target::Lib,
        build_target,
        &dir,
        profile,
        vec![],
        false,
        false,
        target_sdk_version,
    )
    .unwrap();
    let out_dir = dir
        .join("target")
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let compiled_lib = out_dir.join(format!("lib{}.so", name));
    assert!(compiled_lib.exists());

    // Gen android manifest
    let target_dir = dir.join("target");
    let mut manifest = get_android_manifest(&name, target_sdk_version);
    let apk_build_dir = target_dir.join(&profile).join("apk");
    let manifest_path = create_android_manifest(&apk_build_dir, manifest.clone()).unwrap();
    assert!(manifest_path.exists());

    // Gen unaligned apk
    let unaligned_apk_path = gen_unaligned_apk(&sdk, &apk_build_dir, &manifest_path, None, None, manifest.clone()).unwrap();
    assert!(unaligned_apk_path.exists());

    // Add all needed libs into apk
    add_libs_into_apk(
        &sdk,
        &ndk,
        &unaligned_apk_path,
        &compiled_lib,
        build_target,
        profile,
        29,
        &apk_build_dir,
        &target_dir,
    )
    .unwrap();

    // Align apk
    let aligned_apk_path = align_apk(
        &sdk,
        &unaligned_apk_path,
        &manifest.package,
        &apk_build_dir,
    )
    .unwrap();
    assert!(aligned_apk_path.exists());

    // Gen debug key for signing apk
    let key = gen_debug_key().unwrap();
    println!("{:?}", key);

    // Sign apk
    sign_apk(&sdk, &aligned_apk_path, key).unwrap();
}
