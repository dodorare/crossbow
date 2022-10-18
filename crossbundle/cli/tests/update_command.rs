use std::env::current_dir;

use crossbundle_lib::commands::update::check::{
    is_newer_found, is_same_found, print_latest_version_using, print_new_version_available,
};
use crossbundle_tools::types::{Config, Shell, Version};

#[test]
/// Simulate a situation where a new version of crossbundle was released and note the user
/// about that
fn test_new_version_released() {
    let target_dir = current_dir().unwrap();
    let shell = Shell::new();
    let config = Config::new(shell, target_dir);

    let current = env!("CARGO_PKG_VERSION");
    let version = Version::from_semver(current).unwrap();
    let version_string = version.major.to_string()
        + "."
        + &(version.minor as i64).to_string()
        + "."
        + &(version.patch + 1).to_string();
    let same_version = is_same_found(&version_string);
    assert!(!same_version);

    let latest = Some(version_string);
    if let Some(value) = latest {
        if is_newer_found(&value) {
            print_new_version_available(&value, &config).unwrap();
        }
    }
}

#[test]
/// Simulate a situation where the user is using the latest available version of
/// crossbundle project and note the user about that
fn test_latest_version_is_using() {
    let target_dir = current_dir().unwrap();
    let shell = Shell::new();
    let config = Config::new(shell, target_dir);

    let current = env!("CARGO_PKG_VERSION");
    let version = Version::from_semver(current).unwrap();
    let version_string = version.major.to_string()
        + "."
        + &(version.minor as i64).to_string()
        + "."
        + &(version.patch).to_string();
    let newer_version = is_newer_found(&version_string);
    assert!(!newer_version);

    let latest = Some(version_string);
    if let Some(value) = latest {
        if is_same_found(&value) {
            print_latest_version_using(&value, &config).unwrap();
        }
    }
}
