use crate::error::*;
use crossbundle_tools::types::{Config, Version};

static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check current the version of crossbundle package and warn the user if a new version is
/// available
pub fn check_new_version(config: &Config) -> Result<bool> {
    let latest = get_latest_version();

    if let Some(value) = latest {
        if is_newer_found(&value) {
            print_new_version_available(&value, config)?;
            return Ok(true);
        } else if is_same_found(&value) {
            print_latest_version_using(&value, config)?;
            return Ok(false);
        }
    }
    Ok(true)
}

/// Helper function. Check crossbundle project and return true if a new version found
pub fn is_newer_found(version_string: &str) -> bool {
    is_newer(VERSION, version_string, false)
}

/// Helper function. Check crossbundle project and return true if latest version found
pub fn is_same_found(version_string: &str) -> bool {
    is_same(VERSION, version_string, false)
}

/// Print message if crossbundle project can update
pub fn print_new_version_available(latest_version: &str, config: &Config) -> Result<()> {
    config.shell().warn("NEW CROSSBUNDLE VERSION FOUND!")?;
    config.shell().warn(format!(
        "Current version: {}, Latest: {}",
        VERSION, latest_version
    ))?;
    Ok(())
}

/// Print message if user uses latest version of crossbundle project
pub fn print_latest_version_using(version_string: &str, config: &Config) -> Result<()> {
    config.status_message(
        "You are using the latest version of crossbundle project",
        version_string,
    )?;
    Ok(())
}

/// Parse the crossbundle project version used by the user and compare it with the latest
/// available version. Return true if the user has the latest version
fn is_same(version1: &str, version2: &str, default_result: bool) -> bool {
    let version1 = Version::from_semver(version1);
    match version1 {
        Ok(values1) => {
            let version2 = Version::from_semver(version2);

            match version2 {
                Ok(values2) => {
                    values1.major == values2.major
                        && values1.minor == values2.minor
                        && values1.patch == values2.patch
                }
                _ => default_result,
            }
        }
        _ => default_result,
    }
}

/// Parse the crossbundle project version used by the user and compare it with the latest
/// available version
fn is_newer(old_string: &str, new_string: &str, default_result: bool) -> bool {
    let old_version = Version::from_semver(old_string);
    if let Ok(old_values) = old_version {
        let new_version = Version::from_semver(new_string);
        if let Ok(new_values) = new_version {
            if new_values.major == old_values.major {
                if new_values.minor > old_values.minor {
                    true
                } else {
                    new_values.minor == old_values.minor && new_values.patch > old_values.patch
                }
            } else {
                version_comparison(new_values, old_values)
            }
        } else {
            default_result
        }
    } else {
        default_result
    }
}

/// Comparison to check whether the version is newer or not
fn version_comparison(new_values: Version, old_values: Version) -> bool {
    // match new_values.major > old_values.major {
    //     true => match new_values.major == old_values.major {
    //         true => new_values.minor > old_values.minor,
    //         _ => new_values.minor == old_values.minor && new_values.patch > old_values.patch,
    //     },
    //     false => new_values.major < old_values.major,
    // }
    if new_values.major > old_values.major {
        true
    // } else if new_values.major == old_values.major {
    //     if new_values.minor > old_values.minor {
    //         true
    //     } else {
    //         new_values.minor == old_values.minor && new_values.patch > old_values.patch
    //     }
    } else {
        false
    }
}

/// Initialize `cargo search` tool and search crossbundle project in crates.io
fn get_latest_version() -> Option<String> {
    let result = std::process::Command::new("cargo")
        .arg("search")
        .arg("crossbundle")
        .arg("--limit=1")
        .output();

    if let Ok(output) = result {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.split('\n').collect();

        let mut output = None;

        for mut line in lines {
            line = line.trim();

            if line.starts_with("crossbundle = ") {
                output = get_version_from_output(line);

                break;
            }
        }
        output
    } else {
        None
    }
}

/// Parse `cargo search` output and return version from it
fn get_version_from_output(line: &str) -> Option<String> {
    let parts = line.split(' ').collect::<Vec<&str>>();

    if parts.len() >= 3 {
        let version_part = parts[2];
        let version = str::replace(version_part, "\"", "");

        Some(version)
    } else {
        None
    }
}
