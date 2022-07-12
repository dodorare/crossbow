use crossbundle_lib::build::{android::AndroidBuildCommand, BuildContext, SharedBuildCommand};
use crossbundle_tools::{
    commands::gen_minimal_project,
    types::{android_manifest::from_str, AndroidTarget},
    utils::{Config, Shell},
};

#[test]
/// Create macroquad minimal project with full cargo toml metadata in a temporary directory to test manifest generating.
fn test_cargo_metadata() {
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();
    let macroquad_project = true;
    let minimal_cargo_toml = false;
    gen_minimal_project(&project_path, macroquad_project, minimal_cargo_toml).unwrap();

    let target_dir = std::path::PathBuf::from(project_path).join("target");
    std::fs::create_dir_all(&target_dir).unwrap();

    let shell = Shell::new();
    let config = Config::new(shell, target_dir.clone());
    let context = BuildContext::new(&config, Some(target_dir.clone())).unwrap();

    let shared_build_command = SharedBuildCommand {
        example: None,
        features: vec![],
        all_features: false,
        no_default_features: false,
        release: false,
        target_dir: None,
        quad: false,
    };

    let android_build_command = AndroidBuildCommand {
        shared: shared_build_command,
        target: vec![AndroidTarget::Aarch64LinuxAndroid],
        aab: false,
        lib: None,
        gradle: None,
        sign_key_path: None,
        sign_key_pass: None,
        sign_key_alias: None,
    };

    let profile = android_build_command.shared.profile();
    let example = android_build_command.shared.example.as_ref();
    let (_project_path, target_dir, package_name) =
        AndroidBuildCommand::needed_project_dirs(example, &context).unwrap();
    config
        .status_message("Starting apk build process", &package_name)
        .unwrap();
    let (sdk, _ndk, _target_sdk_version) =
        AndroidBuildCommand::android_toolchain(&context).unwrap();

    let android_build_dir = target_dir.join("android").join(&package_name);
    let native_build_dir = android_build_dir.join("native");

    // Get AndroidManifest.xml from file or generate from Cargo.toml
    let (android_manifest, manifest_path) = AndroidBuildCommand::android_manifest(
        &config,
        &context,
        &sdk,
        &package_name,
        profile,
        &native_build_dir,
        false,
    )
    .unwrap();

    let expected_manifest = r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="com.rust.example" android:versionCode="1" android:versionName="0.1.0">
  <application android:debuggable="true" android:hasCode="false" android:label="example" android:theme="@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen">
    <activity android:configChanges="orientation|keyboardHidden|screenSize" android:name="android.app.NativeActivity" android:resizeableActivity="true">
      <intent-filter>
        <action android:name="android.intent.action.MAIN" />
        <category android:name="android.intent.category.LAUNCHER" />
      </intent-filter>
      <meta-data android:name="android.app.lib_name" android:value="example" />
    </activity>
    <service android:name="UpdateService" />
    <meta-data android:name="com.oculus.vr.focusaware" android:value="true" />
  </application>
  <uses-sdk android:minSdkVersion="21" android:targetSdkVersion="30" />
  <uses-feature android:name="android.hardware.vulkan.level" android:required="true" />
  <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" android:maxSdkVersion="30" />
  <uses-permission-sdk-23 android:name="android.permission.INTERNET" android:maxSdkVersion="30" />
  <queries>
    <provider android:authorities="org.khronos.openxr.runtime_broker;org.khronos.openxr.system_runtime_broker" android:name="org.khronos.openxr" />
  </queries>
</manifest>
"#;
    let expected_manifest = from_str(expected_manifest).unwrap();
    assert_eq!(expected_manifest, android_manifest);
    assert!(manifest_path.exists())
}
