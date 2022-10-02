//! # version
//!
//! Checks if the currently running version is the most up to date version and
//! if not, it will print a notification message.
//!

use crate::error::*;
use crossbundle_tools::types::{Config, Version};

static VERSION: &str = env!("CARGO_PKG_VERSION");

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

fn parse(version_string: &str) -> Result<Version> {
    let version = match Version::from_semver(version_string) {
        Ok(version) => Ok(version),
        Err(_) => {
            return Err(Error::InvalidSemver);
        }
    };
    version
}

pub fn is_same(version1: &str, version2: &str, default_result: bool) -> bool {
    let version1 = parse(version1);
    match version1 {
        Ok(values1) => {
            let version2 = parse(version2);

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

pub fn is_newer(old_string: &str, new_string: &str, default_result: bool) -> bool {
    let old_version = parse(old_string);

    match old_version {
        Ok(old_values) => {
            let new_version = parse(new_string);

            match new_version {
                Ok(new_values) => {
                    if new_values.major > old_values.major {
                        true
                    } else if new_values.major == old_values.major {
                        if new_values.minor > old_values.minor {
                            true
                        } else {
                            new_values.minor == old_values.minor
                                && new_values.patch > old_values.patch
                        }
                    } else {
                        false
                    }
                }
                _ => default_result,
            }
        }
        _ => default_result,
    }
}

pub fn is_newer_found(version_string: &str) -> bool {
    is_newer(&VERSION, &version_string, false)
}

pub fn is_same_found(version_string: &str, config: &Config) -> Result<bool> {
    config.status_message(
        "You are using latest version of crossbundle project",
        &version_string,
    )?;
    let is_same = is_newer(&VERSION, &version_string, false);
    Ok(is_same)
}

fn print_notification(latest_version: &str, config: &Config) -> Result<()> {
    config.status("NEW CROSSBUNDLE VERSION FOUND!!!")?;
    config.status_message("Current version", VERSION)?;
    config.status_message("latest", latest_version)?;
    Ok(())
}

pub fn check(config: &Config) -> Result<()> {
    let latest = get_latest_version();

    match latest {
        Some(value) => {
            if is_newer_found(&value) {
                print_notification(&value, config)?;
            } else if is_same_found(&value, config)? {
                print_notification(&value, config)?;
            }
        }
        None => (),
    }
    Ok(())
}
