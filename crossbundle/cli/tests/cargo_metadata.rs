use crossbundle_lib::build::{android::AndroidBuildCommand, BuildContext};
use crossbundle_tools::{
    commands::gen_minimal_project,
    types::{android_manifest::from_str, AndroidStrategy, AndroidTarget},
    utils::{Config, Shell},
};

#[test]
/// Create macroquad minimal project with full cargo toml metadata in a
/// temporary directory to test manifest generating.
fn test_cargo_metadata() {
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();
    let macroquad_project = true;
    let minimal_cargo_toml = false;
    gen_minimal_project(project_path, macroquad_project, minimal_cargo_toml).unwrap();

    let target_dir = std::path::PathBuf::from(project_path).join("target");
    std::fs::create_dir_all(&target_dir).unwrap();

    let shell = Shell::new();
    let config = Config::new(shell, target_dir.clone());
    let context = BuildContext::new(&config, Some(target_dir)).unwrap();

    let android_build_command = AndroidBuildCommand {
        target: vec![AndroidTarget::Aarch64],
        strategy: AndroidStrategy::NativeApk,
        ..Default::default()
    };

    let example = android_build_command.shared.example.as_ref();
    let (_, _, package_name) = AndroidBuildCommand::needed_project_dirs(example, &context).unwrap();
    config
        .status_message("Starting apk build process", &package_name)
        .unwrap();

    let android_manifest = AndroidBuildCommand::get_android_manifest(&context, false).unwrap();

    let expected_manifest = r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="com.crossbow.example" android:versionCode="1" android:versionName="0.1.0">
  <application android:hasCode="false" android:label="Crossbow" android:theme="@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen">
    <activity android:name="android.app.NativeActivity" android:resizeableActivity="true">
      <intent-filter>
        <action android:name="android.intent.action.MAIN" />
        <category android:name="android.intent.category.LAUNCHER" />
      </intent-filter>
      <meta-data android:name="android.app.lib_name" android:value="example" />
    </activity>
  </application>
  <uses-sdk android:minSdkVersion="19" android:targetSdkVersion="30" />
</manifest>
"#;
    let expected_manifest = from_str(expected_manifest).unwrap();
    assert_eq!(expected_manifest, android_manifest);
}
