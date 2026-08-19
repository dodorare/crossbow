#![cfg(all(target_os = "macos", feature = "apple"))]

use crossbundle_tools::{
    commands::{CargoBuild, CargoProject, apple::*, gen_minimal_project},
    types::{IntoRustTriple, IosTarget, Profile},
};

#[test]
fn test_compile_apple() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir, false).unwrap();

    let project = CargoProject::load(&dir.join("Cargo.toml")).unwrap();
    let target = project.executable_target(None, None).unwrap();
    assert_eq!(target.name(), name);
    let target_dir = dir.join("custom-target");
    let executable = compile_ios_executable(
        CargoBuild {
            package: &project.package,
            target: &target,
            target_triple: IosTarget::Aarch64Device.rust_triple(),
            target_dir: &target_dir,
            profile: Profile::Release,
            features: &[],
            all_features: false,
            no_default_features: false,
        },
        None,
    )
    .unwrap();
    assert!(executable.starts_with(target_dir));
}
