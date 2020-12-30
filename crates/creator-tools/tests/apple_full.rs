#![cfg(target_os = "macos")]

use creator_tools::types::*;
use creator_tools::*;
use fs_extra::dir::{ls, DirEntryAttr, DirEntryValue};
use std::collections::HashSet;

fn get_minimal_info_plist(name: &str) -> InfoPlist {
    InfoPlist {
        localization: Localization {
            bundle_development_region: Some("en".to_owned()),
            ..Default::default()
        },
        launch: Launch {
            bundle_executable: Some(name.to_owned()),
            ..Default::default()
        },
        identification: Identification {
            bundle_identifier: "com.test.test-id".to_owned(),
            ..Default::default()
        },
        bundle_version: BundleVersion {
            bundle_version: Some("1".to_owned()),
            bundle_info_dictionary_version: Some("1.0".to_owned()),
            bundle_short_version_string: Some("1.0".to_owned()),
            ..Default::default()
        },
        naming: Naming {
            bundle_name: Some(name.to_owned()),
            ..Default::default()
        },
        categorization: Categorization {
            bundle_package_type: Some("APPL".to_owned()),
            ..Default::default()
        },
        launch_interface: LaunchInterface {
            launch_storyboard_name: Some("LaunchScreen".to_owned()),
            ..Default::default()
        },
        styling: Styling {
            requires_full_screen: Some(false),
            ..Default::default()
        },
        orientation: Orientation {
            supported_interface_orientations: Some(vec![
                InterfaceOrientation::Portrait,
                InterfaceOrientation::PortraitUpsideDown,
                InterfaceOrientation::LandscapeLeft,
                InterfaceOrientation::LandscapeRight,
            ]),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn test_apple_full() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir).unwrap();

    // Create target dir
    let target_dir = dir.join("target");
    std::fs::create_dir(&target_dir).unwrap();

    // Generate app folder
    let app_dir = gen_apple_app(
        &target_dir,
        &name,
        Some(dir.join("src")),
        Some(dir.join("src")),
    )
    .unwrap();
    assert!(app_dir.exists());

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

    // Compile app
    let out_dir = apple_rust_compile(
        &name,
        AppleTarget::X86_64AppleIos,
        dir,
        Profile::Release,
        vec![],
    )
    .unwrap();

    // Copy binary to app folder
    let bin_path = out_dir.join(&name);
    std::fs::copy(&bin_path, &app_dir.join(&name)).unwrap();

    // Generate Info.plist
    let properties = get_minimal_info_plist(&name);
    gen_apple_plist(&app_dir, &properties, false).unwrap();

    // Install and launch on simulator
    launch_apple_app(&app_dir, "iPhone 8", "com.test.test-id", false).unwrap();
}
