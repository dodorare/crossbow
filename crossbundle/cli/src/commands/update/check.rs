//! # version
//!
//! Checks if the currently running version is the most up to date version and
//! if not, it will print a notification message.
//!

use crossbundle_tools::error::CommandExt;
use crossbundle_tools::types::{Config, Version};

// use crate::cache;
// use crate::command;
// use crate::types::{Cache, CliArgs, GlobalConfig};
// use lenient_semver;
// use semver::Version;
use crate::error::{Error, Result};
use std::process::{Command, ExitStatus};
use std::time::{SystemTime, UNIX_EPOCH};

static VERSION: &str = env!("CARGO_PKG_VERSION");

// pub fn get_exit_code(exit_status: Result<ExitStatus>, force: bool) -> i32 {
//     match exit_status {
//         Ok(code) => {
//             if !code.success() {
//                 match code.code() {
//                     Some(value) => value,
//                     None => -1,
//                 }
//             } else {
//                 0
//             }
//         }
//         Err(error) => {
//             if !force {
//                 error!("Error while executing command, error: {:#?}", error);
//                 // return Error::CantFindTargetToRun;
//             }

//             -1
//         }
//     }
// }

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

fn get_latest_version(config: &Config) -> Option<String> {
    let result = Command::new("cargo")
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
            println!("{}", line);
            // debug!("Checking: {}", &line);
            // config.status_message("Checking: {}", &line).unwrap();

            if line.starts_with("crossbundle = ") {
                output = get_version_from_output(line);

                break;
            }
        }
        output
    } else {
        None
    }

    // match result {
    //     Ok(output) => {
    //         let exit_code = get_exit_code(Ok(output.status), false);
    //         if exit_code == 0 {
    //             let stdout = String::from_utf8_lossy(&output.stdout);
    //             let lines: Vec<&str> = stdout.split('\n').collect();

    //             let mut output = None;
    //             for mut line in lines {
    //                 line = line.trim();

    //                 debug!("Checking: {}", &line);

    //                 if line.starts_with("crossbundle = ") {
    //                     output = get_version_from_output(line);

    //                     break;
    //                 }
    //             }

    //             output
    //         } else {
    //             None
    //         }
    //     }
    //     _ => None,
    // }
}

fn parse(version_string: &str) -> Result<Version> {
    let version = match Version::from_semver(version_string) {
        Ok(version) => Ok(version),
        Err(_) => {
            return Err(Error::InvalidSemver);
            // if allow_partial_version_string {
            //     match lenient_semver::parse(version_string) {
            //         Ok(version) => Ok(version),
            //         Err(_) => Err(()),
            //     }
            // } else {
            //     Err(())
            // }
        }
    };
    println!("version {:?}", version);
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

pub fn is_newer(
    old_string: &str,
    new_string: &str,
    // allow_partial_version_string: bool,
    default_result: bool,
) -> bool {
    // let old_version = parse(old_string, allow_partial_version_string);
    let old_version = parse(old_string);
    match old_version {
        Ok(old_values) => {
            // let new_version = parse(new_string, allow_partial_version_string);
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
    debug!("Checking Version: {}", &version_string);

    // is_newer(&VERSION, &version_string, false, false)
    is_newer(&VERSION, &version_string, false)
}

pub fn is_same_found(version_string: &str, config: &Config) -> Result<bool> {
    config.status_message(
        "You are using latest version of crossbundle project",
        &version_string,
    )?;

    // is_newer(&VERSION, &version_string, false, false)
    let is_same = is_newer(&VERSION, &version_string, false);
    Ok(is_same)
}

fn print_notification(latest_string: &str, new_version: bool) {
    warn!("#####################################################################");
    warn!("#                                                                   #");
    warn!("#                                                                   #");
    warn!("#                  NEW CROSSBUNDLE VERSION FOUND!!!                  #");
    warn!(
        "#{:^67}#",
        format!("Current: {}, Latest: {}", VERSION, latest_string)
    );
    warn!("#    Run 'cargo install --force cargo-make' to get latest version   #");
    warn!("#                                                                   #");
    warn!("#                                                                   #");
    warn!("#####################################################################");
}

pub fn check(config: &Config) -> Result<()> {
    let latest = get_latest_version(&config);

    match latest {
        Some(value) => {
            if is_newer_found(&value) {
                print_notification(&value, true);
            } else if is_same_found(&value, config)? {
                print_notification(&value, false)
            }
        }
        None => (),
    }
    Ok(())
}
