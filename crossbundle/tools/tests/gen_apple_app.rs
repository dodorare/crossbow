#![cfg(all(target_os = "macos", feature = "apple"))]

use crossbundle_tools::commands::{apple::*, gen_minimal_project};

#[test]
fn test_gen_apple_app() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir, false, true).unwrap();

    // Creates target dir
    let target_dir = dir.join("target");
    std::fs::create_dir(&target_dir).unwrap();
    // Generate app folder
    let app_dir =
        gen_apple_app_folder(&target_dir, &name, Default::default(), Default::default()).unwrap();
    // Check app dir
    assert!(app_dir.exists());
}
