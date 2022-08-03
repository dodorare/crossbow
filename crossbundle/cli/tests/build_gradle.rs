use crossbundle_lib::build::{android::AndroidBuildCommand, BuildContext};
use crossbundle_tools::{
    commands::gen_minimal_project,
    utils::{Config, Shell},
};

#[test]
/// Use macroquad minimal project in a temporary directory to test gradle project generation.
/// It is working likewise the command below.
/// ```sh
/// crossbundle build android
/// ```
fn test_build_gradle() {
    let tempdir = tempfile::tempdir().unwrap();
    let project_path = tempdir.path();
    let macroquad_project = true;
    gen_minimal_project(project_path, macroquad_project, true).unwrap();

    let target_dir = std::path::PathBuf::from(project_path).join("target");
    std::fs::create_dir_all(&target_dir).unwrap();

    let shell = Shell::new();
    let config = Config::new(shell, target_dir.clone());
    let context = BuildContext::new(&config, Some(target_dir)).unwrap();

    let android_build_command = AndroidBuildCommand::default();

    let (_, _, gradle_project_path) = AndroidBuildCommand::build_gradle(
        &android_build_command,
        &config,
        &context,
        &Some(project_path.to_owned()),
    )
    .unwrap();
    assert!(
        gradle_project_path.join("build.gradle").exists(),
        "Gradle Project's build.gradle file should exist"
    );
}
