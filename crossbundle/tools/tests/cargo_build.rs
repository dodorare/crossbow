use crossbundle_tools::{
    commands::{CargoBuild, CargoProject},
    types::Profile,
};

#[test]
fn cargo_reports_a_renamed_executable_in_a_custom_target_directory() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("src")).unwrap();
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"fixture-package\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\
         [[bin]]\nname = \"mobile-app\"\npath = \"src/main.rs\"\n",
    )
    .unwrap();
    std::fs::write(root.path().join("src/main.rs"), "fn main() {}\n").unwrap();

    let project = CargoProject::load(&root.path().join("Cargo.toml")).unwrap();
    let target = project.executable_target(None, None).unwrap();
    let target_dir = root.path().join("custom-target");
    let version = std::process::Command::new("rustc")
        .arg("-vV")
        .output()
        .unwrap();
    let version = String::from_utf8(version.stdout).unwrap();
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .unwrap();

    let artifact = CargoBuild {
        package: &project.package,
        target: &target,
        target_triple: host,
        target_dir: &target_dir,
        profile: Profile::Debug,
        features: &[],
        all_features: false,
        no_default_features: false,
    }
    .run(|_| {})
    .unwrap();

    let executable = artifact.executable.unwrap();
    assert!(executable.starts_with(target_dir));
    assert_eq!(executable.file_stem().unwrap(), "mobile-app");
}
