use crossbundle_lib::build::{android::AndroidBuildCommand, BuildContext};
use crossbundle_tools::{
    commands::gen_minimal_project,
    types::android_manifest::from_str,
    utils::{Config, Shell},
};

#[test]
/// Create macroquad minimal project with full cargo toml metadata in a temporary directory to test manifest generating.
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

    let android_build_command = AndroidBuildCommand::default();

    let example = android_build_command.shared.example.as_ref();
    let (_project_path, target_dir, package_name) =
        AndroidBuildCommand::needed_project_dirs(example, &context).unwrap();
    config
        .status_message("Starting apk build process", &package_name)
        .unwrap();
    let (_sdk, _ndk) = AndroidBuildCommand::android_toolchain().unwrap();

    let android_build_dir = target_dir.join("android").join(&package_name);
    let native_build_dir = android_build_dir.join("native");

    // Get AndroidManifest.xml from file or generate from Cargo.toml
    let (android_manifest, manifest_path) = AndroidBuildCommand::android_manifest(
        &config,
        &context,
        &package_name,
        &native_build_dir,
        false,
    )
    .unwrap();

    let expected_manifest = r#"
    <?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="com.crossbow.example" android:versionCode="1" android:versionName="1">
  <application android:hasCode="false" android:label="Crossbow" android:theme="@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen">
    <activity android:name="android.app.NativeActivity" android:resizeableActivity="true">
      <meta-data android:name="android.app.lib_name" android:value="example" />
    </activity>
  </application>
  <uses-sdk android:minSdkVersion="19" android:targetSdkVersion="30" />
</manifest>
"#;

    let expected_manifest = from_str(expected_manifest).unwrap();
    assert_eq!(expected_manifest, android_manifest);
    assert!(manifest_path.exists())
}
