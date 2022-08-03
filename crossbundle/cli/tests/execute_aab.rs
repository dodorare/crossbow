use crossbundle_lib::build::{android::AndroidBuildCommand, BuildContext};
use crossbundle_tools::{
    commands::gen_minimal_project,
    utils::{Config, Shell},
};

#[test]
/// Use bevy minimal project in a temporary directory to test AAB generation.
/// It is working likewise the command below.
/// ```sh
/// crossbundle build android --aab
/// ```
fn test_execute_aab() {
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();
    let macroquad_project = false;
    gen_minimal_project(project_path, macroquad_project, true).unwrap();

    let target_dir = std::path::PathBuf::from(project_path).join("target");
    std::fs::create_dir_all(&target_dir).unwrap();

    let shell = Shell::new();
    let config = Config::new(shell, target_dir.clone());
    let context = BuildContext::new(&config, Some(target_dir.clone())).unwrap();

    let android_build_command = AndroidBuildCommand::default();

    let (_, _, generated_aab_path, _, _) =
        AndroidBuildCommand::execute_aab(&android_build_command, &config, &context).unwrap();
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
