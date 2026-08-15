#![cfg(any(feature = "android", feature = "apple"))]

use std::process::Command;

fn crossbundle() -> Command {
    Command::new(env!("CARGO_BIN_EXE_crossbundle"))
}

fn enabled_platform() -> &'static str {
    for platform in ["android", "apple"] {
        let output = crossbundle()
            .env("PATH", "")
            .args(["doctor", "--platform", platform, "--json"])
            .output()
            .unwrap();
        if output.status.code() != Some(2) {
            return platform;
        }
    }
    panic!("test binary has no enabled doctor platform")
}

#[test]
fn invalid_platform_exits_two_without_stdout() {
    let output = crossbundle()
        .args(["doctor", "--platform", "commodore", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}

#[cfg(all(feature = "android", not(feature = "apple")))]
#[test]
fn explicitly_disabled_apple_platform_exits_two() {
    let output = crossbundle()
        .args(["doctor", "--platform", "apple", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not compiled"));
}

#[cfg(all(feature = "apple", not(feature = "android")))]
#[test]
fn explicitly_disabled_android_platform_exits_two() {
    let output = crossbundle()
        .args(["doctor", "--platform", "android", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not compiled"));
}

#[test]
fn json_stdout_is_a_pure_json_document() {
    let platform = enabled_platform();
    let output = crossbundle()
        .args(["doctor", "--platform", platform, "--json"])
        .output()
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["platforms"], serde_json::json!([platform]));
}

#[test]
fn failed_checks_exit_one_and_keep_json_parseable() {
    let output = crossbundle()
        .env("PATH", "")
        .args(["doctor", "--platform", enabled_platform(), "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["status"], "fail");
}

#[test]
fn human_report_uses_stderr_only() {
    let output = crossbundle()
        .env("PATH", "")
        .args(["doctor", "--platform", enabled_platform()])
        .output()
        .unwrap();
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Crossbundle doctor"));
}
