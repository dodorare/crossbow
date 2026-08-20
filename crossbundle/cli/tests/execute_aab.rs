#![cfg(feature = "android")]

use crossbundle_lib::commands::build::{BuildContext, android::AndroidBuildCommand};
use crossbundle_tools::{
    commands::gen_minimal_project,
    types::{AndroidNdk, AndroidSdk, AndroidStrategy, AndroidTarget, CliContext, Shell},
};

#[test]
/// Use bevy minimal project in a temporary directory to test AAB generation.
/// It is working like the command below.
/// ```sh
/// crossbundle build android -s=native-aab
/// ```
fn test_execute_aab() {
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();
    let macroquad_project = false;
    gen_minimal_project(project_path, macroquad_project).unwrap();

    let target_dir = std::path::PathBuf::from(project_path).join("target");
    std::fs::create_dir_all(&target_dir).unwrap();

    let shell = Shell::new();
    let config = CliContext::new(shell, target_dir.clone());
    let context = BuildContext::new(&config, &Default::default()).unwrap();

    let android_build_command = AndroidBuildCommand {
        target: vec![AndroidTarget::Aarch64],
        strategy: AndroidStrategy::NativeAab,
        ..Default::default()
    };

    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(sdk.sdk_path()).unwrap();
    let java = std::path::PathBuf::from("java");
    let jarsigner = std::path::PathBuf::from("jarsigner");
    let bundletool = std::path::PathBuf::from(std::env::var_os("BUNDLETOOL_PATH").unwrap());
    let (_, _, generated_aab_path, _, _) = AndroidBuildCommand::execute_aab(
        &android_build_command,
        &config,
        &context,
        &sdk,
        &ndk,
        &java,
        &jarsigner,
        &bundletool,
    )
    .unwrap();
    let expected_path = target_dir
        .join("android")
        .join("example")
        .join("outputs")
        .join("example_signed.aab");
    assert_eq!(generated_aab_path, expected_path);
    assert!(
        generated_aab_path.exists(),
        "Final generated .aab file should exist"
    );
}
