#![cfg(target_os = "macos")]

use creator_build::*;
use fs_extra::dir::{ls, DirEntryAttr, DirEntryValue};
use std::collections::HashSet;

#[test]
fn test_gen_apple_app() {
    let dir = tempfile::tempdir().unwrap();
    let generate_minimal_project = GenMinimalProject::new(dir.path().to_owned());
    let name = generate_minimal_project.run().unwrap();
    // Creates target dir
    let target_dir = dir.path().join("target");
    std::fs::create_dir(&target_dir).unwrap();
    // Generate app folder
    let gen_apple_app = GenAppleApp::new(
        target_dir,
        name,
        dir.path().join("src"),
        dir.path().join("src"),
    );
    let app_dir = gen_apple_app.run().unwrap();
    // Check app dir
    let mut config = HashSet::new();
    config.insert(DirEntryAttr::FullName);
    let res = ls(&app_dir, &config).unwrap();
    res.items.iter().for_each(|vec| {
        vec.iter().for_each(|(_, value)| {
            if let DirEntryValue::String(val) = value {
                println!("value: {:?}", val)
            }
        })
    });
    assert_eq!(2, res.items.len());
}
