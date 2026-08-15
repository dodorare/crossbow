use super::*;
use crate::types::IntoRustTriple;
use std::process::Command;

const READ_ONLY_COMMANDS: &[(&str, &str, &[&str])] = &[
    ("apple.developer_dir", "xcode-select", &["--print-path"]),
    ("apple.xcode.version", "xcodebuild", &["-version"]),
    ("apple.command_line_tools", "xcrun", &["--find", "clang"]),
    ("apple.simctl", "xcrun", &["--find", "simctl"]),
    (
        "apple.sdk.iphoneos",
        "xcrun",
        &["--sdk", "iphoneos", "--show-sdk-path"],
    ),
    (
        "apple.sdk.iphonesimulator",
        "xcrun",
        &["--sdk", "iphonesimulator", "--show-sdk-path"],
    ),
];
const SIGNING_COMMAND: (&str, &str, &[&str]) = (
    "apple.signing.identities",
    "security",
    &["find-identity", "-v", "-p", "codesigning"],
);

fn read_only_commands(
    signing_relevant: bool,
) -> impl Iterator<Item = (&'static str, &'static str, &'static [&'static str])> {
    READ_ONLY_COMMANDS
        .iter()
        .copied()
        .chain(signing_relevant.then_some(SIGNING_COMMAND))
}

pub(super) fn discover_read_only_commands(
    signing_relevant: bool,
) -> BTreeMap<String, CommandObservation> {
    if !cfg!(target_os = "macos") {
        return BTreeMap::new();
    }
    read_only_commands(signing_relevant)
        .map(|(id, program, arguments)| {
            let observation = Command::new(program)
                .args(arguments)
                .output()
                .map(|output| CommandObservation {
                    success: output.status.success(),
                    stdout: String::from_utf8_lossy(&output.stdout).trim().to_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
                })
                .unwrap_or_default();
            (id.to_owned(), observation)
        })
        .collect()
}

pub(super) fn checks(
    request: &DoctorRequest,
    environment: &Environment,
    policy: &CompatibilityPolicy,
    project: Option<&project::ProjectContext>,
) -> Vec<DoctorCheck> {
    if environment.host_os != "macos" {
        let mut checks = unsupported_host_checks(request, environment, project);
        if let Some(project) = project {
            checks.extend(project_checks(project));
        }
        return checks;
    }

    let developer_dir = developer_dir(environment);
    let mut checks = vec![
        check(
            "apple.host.os",
            CheckStatus::Pass,
            "Apple",
            "Apple tooling is supported on macOS".into(),
            true,
            None,
            None,
            None,
        ),
        developer_dir_check(developer_dir.as_deref()),
        xcode_installation_check(developer_dir.as_deref()),
        xcode_version_check(environment, policy, request.strict),
        command_path_check(
            environment,
            "apple.xcode.command_line_tools",
            "apple.command_line_tools",
            "Command Line Tools",
        ),
        tool_check(
            environment,
            developer_dir.as_deref(),
            "apple.tool.xcodebuild",
            "xcodebuild",
        ),
        tool_check(
            environment,
            developer_dir.as_deref(),
            "apple.tool.xcrun",
            "xcrun",
        ),
        command_path_check(environment, "apple.tool.simctl", "apple.simctl", "simctl"),
        sdk_check(environment, "apple.sdk.iphoneos", "iPhoneOS"),
        sdk_check(environment, "apple.sdk.iphonesimulator", "iPhoneSimulator"),
    ];
    checks.extend(rust_target_checks(request, environment, false, project));
    checks.push(signing_identity_check(environment, request, project));
    if let Some(project) = project {
        checks.extend(project_checks(project));
    }
    checks
}

fn unsupported_host_checks(
    request: &DoctorRequest,
    environment: &Environment,
    project: Option<&project::ProjectContext>,
) -> Vec<DoctorCheck> {
    let reason = format!(
        "Apple tooling is only available on macOS; host is {}",
        environment.host_os
    );
    let mut checks = [
        "apple.host.os",
        "apple.xcode.installation",
        "apple.xcode.version",
        "apple.xcode.developer_dir",
        "apple.xcode.command_line_tools",
        "apple.tool.xcodebuild",
        "apple.tool.xcrun",
        "apple.tool.simctl",
        "apple.sdk.iphoneos",
        "apple.sdk.iphonesimulator",
        "apple.signing.identity",
    ]
    .into_iter()
    .map(|id| skipped(id, "Apple", id == "apple.host.os", &reason))
    .collect::<Vec<_>>();
    checks.extend(rust_target_checks(request, environment, true, project));
    checks
}

fn developer_dir(environment: &Environment) -> Option<PathBuf> {
    environment
        .variables
        .get("DEVELOPER_DIR")
        .map(PathBuf::from)
        .or_else(|| command_path(environment, "apple.developer_dir"))
}

fn developer_dir_check(path: Option<&Path>) -> DoctorCheck {
    match path {
        Some(path) if path.is_dir() => check(
            "apple.xcode.developer_dir",
            CheckStatus::Pass,
            "Apple",
            "Found the active developer directory".into(),
            true,
            Some(ObservedValue {
                version: None,
                path: Some(path.to_owned()),
            }),
            None,
            None,
        ),
        Some(path) => check(
            "apple.xcode.developer_dir",
            CheckStatus::Fail,
            "Apple",
            "The active developer directory does not exist".into(),
            true,
            Some(ObservedValue {
                version: None,
                path: Some(path.to_owned()),
            }),
            None,
            Some("Select Xcode with xcode-select or fix DEVELOPER_DIR".into()),
        ),
        None => check(
            "apple.xcode.developer_dir",
            CheckStatus::Fail,
            "Apple",
            "No active developer directory was found".into(),
            true,
            None,
            None,
            Some("Install Xcode and select its developer directory".into()),
        ),
    }
}

fn xcode_installation_check(developer_dir: Option<&Path>) -> DoctorCheck {
    let application = developer_dir.and_then(|path| {
        path.ancestors().find(|ancestor| {
            ancestor
                .extension()
                .is_some_and(|extension| extension == "app")
        })
    });
    match application.filter(|path| path.is_dir()) {
        Some(path) => check(
            "apple.xcode.installation",
            CheckStatus::Pass,
            "Apple",
            "Found an Xcode installation".into(),
            true,
            Some(ObservedValue {
                version: None,
                path: Some(path.to_owned()),
            }),
            None,
            None,
        ),
        None => check(
            "apple.xcode.installation",
            CheckStatus::Fail,
            "Apple",
            "A full Xcode installation was not found".into(),
            true,
            developer_dir.map(|path| ObservedValue {
                version: None,
                path: Some(path.to_owned()),
            }),
            None,
            Some("Install Xcode and select Xcode.app as the developer directory".into()),
        ),
    }
}

fn xcode_version_check(
    environment: &Environment,
    policy: &CompatibilityPolicy,
    strict: bool,
) -> DoctorCheck {
    let Some(observation) = environment
        .commands
        .get("apple.xcode.version")
        .filter(|observation| observation.success)
    else {
        return check(
            "apple.xcode.version",
            CheckStatus::Fail,
            "Apple",
            "xcodebuild -version did not succeed".into(),
            true,
            None,
            policy.apple_tool("xcode").map(expectation),
            Some("Install or repair Xcode so xcodebuild -version succeeds".into()),
        );
    };
    let version = version_in_text(&observation.stdout);
    let Some(version) = version else {
        return check(
            "apple.xcode.version",
            if strict {
                CheckStatus::Fail
            } else {
                CheckStatus::Warn
            },
            "Apple",
            "The Xcode version could not be determined".into(),
            true,
            None,
            policy.apple_tool("xcode").map(expectation),
            Some("Ensure xcodebuild -version succeeds".into()),
        );
    };
    let Some(version_policy) = policy.apple_tool("xcode") else {
        return check(
            "apple.xcode.version",
            CheckStatus::Pass,
            "Apple",
            "Discovered the Xcode version; Crossbundle imposes no version range".into(),
            true,
            Some(ObservedValue {
                version: Some(version),
                path: None,
            }),
            None,
            None,
        );
    };
    classified_observation(
        "apple.xcode.version",
        "Apple",
        PathBuf::from("xcodebuild"),
        Some(version),
        true,
        Some(version_policy),
        strict,
        "Install a supported Xcode version".into(),
    )
}

fn tool_check(
    environment: &Environment,
    developer_dir: Option<&Path>,
    id: &str,
    name: &str,
) -> DoctorCheck {
    let path = environment
        .executable_matching(name, executable_file)
        .or_else(|| {
            developer_dir
                .map(|directory| directory.join("usr/bin").join(name))
                .filter(|path| executable_file(path))
        });
    required_path_check(
        id,
        path,
        None,
        format!("Found {name}"),
        format!("{name} was not found"),
        format!("Install Xcode so {name} is available"),
    )
}

fn command_path_check(
    environment: &Environment,
    id: &str,
    command_id: &str,
    label: &str,
) -> DoctorCheck {
    required_path_check(
        id,
        command_path(environment, command_id).filter(|path| executable_file(path)),
        None,
        format!("Found {label}"),
        format!("{label} was not found"),
        format!("Install or repair Xcode so {label} is available"),
    )
}

fn executable_file(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

fn sdk_check(environment: &Environment, id: &str, label: &str) -> DoctorCheck {
    let path = command_path(environment, id).filter(|path| path.is_dir());
    let version = path
        .as_ref()
        .and_then(|path| version_in_text(path.to_string_lossy().as_ref()));
    required_path_check(
        id,
        path,
        version,
        format!("Found the installed {label} SDK"),
        format!("The {label} SDK was not found"),
        format!("Install {label} platform support in Xcode"),
    )
}

fn required_path_check(
    id: &str,
    path: Option<PathBuf>,
    version: Option<String>,
    found_summary: String,
    missing_summary: String,
    remediation: String,
) -> DoctorCheck {
    match path {
        Some(path) => check(
            id,
            CheckStatus::Pass,
            "Apple",
            found_summary,
            true,
            Some(ObservedValue {
                version,
                path: Some(path),
            }),
            None,
            None,
        ),
        None => check(
            id,
            CheckStatus::Fail,
            "Apple",
            missing_summary,
            true,
            None,
            None,
            Some(remediation),
        ),
    }
}

fn rust_target_checks(
    request: &DoctorRequest,
    environment: &Environment,
    unsupported_host: bool,
    project: Option<&project::ProjectContext>,
) -> Vec<DoctorCheck> {
    let mut targets: Vec<_> = request
        .targets
        .iter()
        .filter(|target| target.contains("-apple-ios"))
        .cloned()
        .collect();
    if targets.is_empty() {
        targets = project
            .and_then(project::ProjectContext::metadata)
            .map(|metadata| apple_targets(metadata).map(str::to_owned).collect())
            .unwrap_or_default();
        if targets.is_empty() {
            targets.push("aarch64-apple-ios-sim".into());
        }
    }
    targets.sort();
    targets.dedup();
    let project_dir = project
        .map(project::ProjectContext::base_dir)
        .or_else(|| {
            request.project.as_deref().map(|path| {
                if path.file_name().is_some_and(|name| name == "Cargo.toml") {
                    path.parent().unwrap_or_else(|| Path::new("."))
                } else {
                    path
                }
            })
        })
        .unwrap_or_else(|| Path::new("."));
    let sysroot = (!unsupported_host)
        .then(|| rust_sysroot(environment, project_dir))
        .flatten();
    targets
        .into_iter()
        .map(|target| {
            let id = format!("apple.rust.target.{target}");
            if unsupported_host {
                return skipped(
                    &id,
                    "Apple",
                    true,
                    "Apple Rust targets are irrelevant on a non-macOS host",
                );
            }
            let installed = sysroot
                .as_ref()
                .is_some_and(|root| root.join("lib/rustlib").join(&target).join("lib").is_dir());
            check(
                &id,
                if installed {
                    CheckStatus::Pass
                } else {
                    CheckStatus::Fail
                },
                "Apple",
                if installed {
                    format!("Rust target {target} is installed")
                } else {
                    format!("Rust target {target} is not installed")
                },
                true,
                sysroot.as_ref().map(|path| ObservedValue {
                    version: None,
                    path: Some(path.clone()),
                }),
                None,
                (!installed).then(|| format!("Run rustup target add {target}")),
            )
        })
        .collect()
}

fn successful_command<'a>(
    environment: &'a Environment,
    id: &str,
) -> Option<&'a CommandObservation> {
    environment.commands.get(id).filter(|output| output.success)
}

fn command_path(environment: &Environment, id: &str) -> Option<PathBuf> {
    successful_command(environment, id)
        .map(|output| output.stdout.trim())
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
}

pub(super) fn signing_relevant(
    request: &DoctorRequest,
    project: Option<&project::ProjectContext>,
) -> bool {
    request.targets.iter().any(|target| device_target(target))
        || project
            .and_then(project::ProjectContext::metadata)
            .is_some_and(|metadata| apple_targets(metadata).any(device_target))
}

fn device_target(target: &str) -> bool {
    matches!(
        target,
        "aarch64-apple-ios" | "armv7-apple-ios" | "armv7s-apple-ios"
    )
}

fn signing_identity_check(
    environment: &Environment,
    request: &DoctorRequest,
    project: Option<&project::ProjectContext>,
) -> DoctorCheck {
    if !signing_relevant(request, project) {
        return skipped(
            "apple.signing.identity",
            "Apple",
            false,
            "Signing identities are irrelevant unless a device target is configured",
        );
    }
    let identities = successful_command(environment, "apple.signing.identities")
        .map(|output| {
            output
                .stdout
                .lines()
                .filter(|line| line.trim_start().contains(") "))
                .count()
        })
        .unwrap_or_default();
    check(
        "apple.signing.identity",
        if identities > 0 {
            CheckStatus::Pass
        } else {
            CheckStatus::Warn
        },
        "Apple",
        if identities > 0 {
            format!("Found {identities} code-signing identity or identities")
        } else {
            "No code-signing identities were found; unsigned builds remain available".into()
        },
        false,
        None,
        None,
        (identities == 0)
            .then(|| "Install a signing identity before signing a device build".into()),
    )
}

fn project_checks(context: &project::ProjectContext) -> Vec<DoctorCheck> {
    let Some(project) = context.project() else {
        return Vec::new();
    };
    let Ok(metadata) = &project.metadata else {
        return vec![check(
            "project.apple.metadata",
            CheckStatus::Fail,
            "Project Apple",
            "Apple metadata could not be parsed with the build model".into(),
            true,
            None,
            None,
            Some("Fix typed package.metadata.apple fields".into()),
        )];
    };
    let base = context.base_dir();
    let plist = resolved_info_plist(metadata, &project.package_name, base);
    let (metadata_status, metadata_summary) = if plist.is_err() {
        (
            CheckStatus::Fail,
            "The configured Info.plist could not be read with the Apple build model",
        )
    } else if project.apple_metadata_present {
        (
            CheckStatus::Pass,
            "Apple metadata is valid and uses the Apple build model",
        )
    } else {
        (
            CheckStatus::Warn,
            "Apple metadata is absent; build defaults will be used",
        )
    };
    let mut checks = vec![
        check(
            "project.apple.metadata",
            metadata_status,
            "Project Apple",
            metadata_summary.into(),
            false,
            metadata
                .apple
                .info_plist_path
                .as_ref()
                .map(|path| ObservedValue {
                    version: None,
                    path: Some(base.join(path)),
                }),
            None,
            plist
                .is_err()
                .then(|| "Fix or remove info_plist_path".into()),
        ),
        bundle_identifier_check(plist.as_ref().ok()),
        deployment_target_check(plist.as_ref().ok()),
        project_paths_check(
            "project.apple.assets",
            "Project Apple",
            "Apple assets and resources",
            metadata
                .get_apple_assets()
                .iter()
                .chain(metadata.get_apple_resources())
                .map(|path| base.join(path))
                .collect(),
            true,
            PathRequirement::ReadableDirectory,
        ),
        project_path_check(
            "project.apple.icon",
            "Project Apple",
            "Apple icon",
            metadata.icon.as_ref().map(|path| base.join(path)),
            true,
            PathRequirement::ReadableFile,
        ),
        skipped(
            "project.apple.signing",
            "Project Apple",
            false,
            "The typed Apple project model has no signing fields; signing is requested per build invocation",
        ),
    ];
    let mut targets = apple_targets(metadata).collect::<Vec<_>>();
    if targets.is_empty() {
        targets.push("aarch64-apple-ios-sim");
    }
    targets.sort_unstable();
    targets.dedup();
    checks.extend(targets.into_iter().map(|target| {
        check(
            &format!("project.apple.target.{target}"),
            CheckStatus::Pass,
            "Project Apple",
            format!("Apple Rust target {target} is supported"),
            true,
            None,
            None,
            None,
        )
    }));
    let mut plugin_checks = project
        .android_plugins
        .iter()
        .map(|plugin| {
            skipped(
                &format!("project.apple.plugin.{}", normalize_id(plugin)),
                "Project Apple",
                false,
                &format!("Configured Android plugin {plugin} is not applied to Apple"),
            )
        })
        .collect::<Vec<_>>();
    plugin_checks.sort_by(|left, right| left.id.cmp(&right.id));
    plugin_checks.dedup_by(|left, right| left.id == right.id);
    checks.extend(plugin_checks);
    checks
}

fn apple_targets(
    metadata: &crate::types::CrossbowMetadata,
) -> impl Iterator<Item = &'static str> + '_ {
    metadata
        .apple
        .debug_build_targets
        .iter()
        .chain(&metadata.apple.release_build_targets)
        .map(IntoRustTriple::rust_triple)
}

fn resolved_info_plist(
    metadata: &crate::types::CrossbowMetadata,
    package_name: &str,
    base: &Path,
) -> Result<crate::types::apple_bundle::prelude::InfoPlist, ()> {
    if let Some(path) = &metadata.apple.info_plist_path {
        return crate::commands::apple::read_info_plist(&base.join(path)).map_err(|_| ());
    }
    let mut plist = metadata.apple.info_plist.clone().unwrap_or_default();
    crate::types::update_info_plist_with_default(
        &mut plist,
        package_name,
        metadata.app_name.clone(),
    );
    Ok(plist)
}

fn bundle_identifier_check(
    plist: Option<&crate::types::apple_bundle::prelude::InfoPlist>,
) -> DoctorCheck {
    let identifier = plist.map(|plist| plist.identification.bundle_identifier.as_str());
    let valid = identifier.is_some_and(valid_bundle_identifier);
    check(
        "project.apple.bundle_identifier",
        if valid {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        },
        "Project Apple",
        if valid {
            "Apple bundle identifier is valid"
        } else {
            "Apple bundle identifier is missing or invalid"
        }
        .into(),
        true,
        None,
        None,
        (!valid).then(|| "Use a reverse-DNS bundle identifier".into()),
    )
}

fn valid_bundle_identifier(identifier: &str) -> bool {
    let components = identifier.split('.').collect::<Vec<_>>();
    components.len() >= 2
        && components.iter().all(|component| {
            !component.is_empty()
                && component
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

fn deployment_target_check(
    plist: Option<&crate::types::apple_bundle::prelude::InfoPlist>,
) -> DoctorCheck {
    let deployment =
        plist.and_then(|plist| plist.operating_system_version.minimum_os_version.as_deref());
    match deployment {
        None => skipped(
            "project.apple.deployment_target",
            "Project Apple",
            false,
            "No Apple deployment target is configured",
        ),
        Some(version) if valid_dotted_version(version) => check(
            "project.apple.deployment_target",
            CheckStatus::Pass,
            "Project Apple",
            "Apple deployment target is valid".into(),
            true,
            Some(ObservedValue {
                version: Some(version.to_owned()),
                path: None,
            }),
            None,
            None,
        ),
        Some(version) => check(
            "project.apple.deployment_target",
            CheckStatus::Fail,
            "Project Apple",
            "Apple deployment target is invalid".into(),
            true,
            Some(ObservedValue {
                version: Some(version.to_owned()),
                path: None,
            }),
            None,
            Some("Use a numeric dotted iOS version".into()),
        ),
    }
}

fn valid_dotted_version(version: &str) -> bool {
    let mut components = version.split('.');
    let count = components.clone().count();
    (1..=3).contains(&count)
        && components.all(|component| {
            !component.is_empty() && component.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn normalize_id(name: &str) -> String {
    let mut normalized = String::new();
    let mut separator = false;
    for character in name.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            normalized.push(character);
            separator = false;
        } else if !normalized.is_empty() && !separator {
            normalized.push('-');
            separator = true;
        }
    }
    while normalized.ends_with('-') {
        normalized.pop();
    }
    if normalized.is_empty() {
        "unnamed".into()
    } else {
        normalized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_environment(root: &Path, xcode_version: &str) -> Environment {
        let developer = root.join("Applications/Xcode.app/Contents/Developer");
        let bin = root.join("bin");
        let sdk = developer.join("Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk");
        let simulator_sdk =
            developer.join("Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk");
        let clang = developer.join("Toolchains/XcodeDefault.xctoolchain/usr/bin/clang");
        let simctl = developer.join("usr/bin/simctl");
        for directory in [&sdk, &simulator_sdk, &bin] {
            fs::create_dir_all(directory).unwrap();
        }
        for file in [
            bin.join("xcodebuild"),
            bin.join("xcrun"),
            clang.clone(),
            simctl.clone(),
        ] {
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(&file, "").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(file, fs::Permissions::from_mode(0o755)).unwrap();
            }
        }
        let commands = [
            ("apple.developer_dir", developer.display().to_string()),
            ("apple.xcode.version", format!("Xcode {xcode_version}")),
            ("apple.command_line_tools", clang.display().to_string()),
            ("apple.simctl", simctl.display().to_string()),
            ("apple.sdk.iphoneos", sdk.display().to_string()),
            (
                "apple.sdk.iphonesimulator",
                simulator_sdk.display().to_string(),
            ),
        ]
        .into_iter()
        .map(|(id, stdout)| {
            (
                id.into(),
                CommandObservation {
                    success: true,
                    stdout,
                    stderr: String::new(),
                },
            )
        })
        .collect();
        Environment {
            host_os: "macos".into(),
            path_entries: vec![bin],
            commands,
            ..Environment::default()
        }
    }

    #[test]
    fn fixture_discovers_xcode_tools_and_sdks() {
        let temp = tempfile::tempdir().unwrap();
        let environment = fixture_environment(temp.path(), "16.4");
        let report = diagnose(
            &DoctorRequest {
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &environment,
        );
        for id in [
            "apple.xcode.installation",
            "apple.xcode.developer_dir",
            "apple.xcode.version",
            "apple.xcode.command_line_tools",
            "apple.tool.xcodebuild",
            "apple.tool.xcrun",
            "apple.tool.simctl",
            "apple.sdk.iphoneos",
            "apple.sdk.iphonesimulator",
        ] {
            let check = report.checks.iter().find(|check| check.id == id).unwrap();
            assert_eq!(check.status, CheckStatus::Pass, "{id}: {check:?}");
        }
    }

    #[test]
    fn macos_missing_apple_tools_fail() {
        let report = diagnose(
            &DoctorRequest {
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "macos".into(),
                ..Environment::default()
            },
        );
        for id in [
            "apple.xcode.installation",
            "apple.xcode.developer_dir",
            "apple.xcode.version",
            "apple.tool.xcodebuild",
            "apple.tool.xcrun",
            "apple.tool.simctl",
            "apple.sdk.iphoneos",
            "apple.sdk.iphonesimulator",
        ] {
            assert_eq!(
                report
                    .checks
                    .iter()
                    .find(|check| check.id == id)
                    .unwrap()
                    .status,
                CheckStatus::Fail,
                "{id}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn macos_non_executable_apple_tools_fail() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().unwrap();
        let environment = fixture_environment(temp.path(), "16.4");
        for tool in ["xcodebuild", "xcrun"] {
            let path = environment.executable(tool).unwrap();
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o644)).unwrap();
        }
        let report = diagnose(
            &DoctorRequest {
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &environment,
        );
        for id in ["apple.tool.xcodebuild", "apple.tool.xcrun"] {
            assert_eq!(
                report
                    .checks
                    .iter()
                    .find(|check| check.id == id)
                    .unwrap()
                    .status,
                CheckStatus::Fail,
                "{id}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn apple_tool_lookup_skips_non_executable_path_entries() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().unwrap();
        let blocked = temp.path().join("blocked/xcodebuild");
        let usable = temp.path().join("usable/xcodebuild");
        for (path, mode) in [(&blocked, 0o644), (&usable, 0o755)] {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "").unwrap();
            fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
        }
        let environment = Environment {
            path_entries: vec![
                blocked.parent().unwrap().to_owned(),
                usable.parent().unwrap().to_owned(),
            ],
            ..Environment::default()
        };
        assert_eq!(
            tool_check(&environment, None, "apple.tool.xcodebuild", "xcodebuild")
                .found
                .unwrap()
                .path,
            Some(usable)
        );
    }

    #[test]
    fn xcode_versions_follow_compatibility_and_strict_policy() {
        let temp = tempfile::tempdir().unwrap();
        let mut policy = CompatibilityPolicy::embedded();
        policy.apple.insert(
            "xcode".into(),
            super::super::super::VersionPolicy {
                preferred: "16".into(),
                supported: ">=15, <17".into(),
            },
        );
        for (version, strict, expected) in [
            ("16.4", false, CheckStatus::Pass),
            ("15.4", true, CheckStatus::Pass),
            ("17.0", false, CheckStatus::Warn),
            ("17.0", true, CheckStatus::Fail),
            ("unknown", false, CheckStatus::Warn),
            ("unknown", true, CheckStatus::Fail),
        ] {
            let environment = fixture_environment(temp.path(), version);
            assert_eq!(
                xcode_version_check(&environment, &policy, strict).status,
                expected,
                "version={version}, strict={strict}"
            );
        }
    }

    fn write_project(root: &Path, manifest: &str) {
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/lib.rs"), "").unwrap();
        fs::write(root.join("Cargo.toml"), manifest).unwrap();
    }

    #[test]
    fn validates_typed_apple_project_metadata_targets_assets_and_plugins() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        write_project(
            &project,
            r#"
[package]
name = "doctor-fixture"
version = "0.1.0"
edition = "2024"

[package.metadata]
assets = ["assets"]
icon = "icon.png"

[package.metadata.android]
plugins_remote = ["com.example:camera-android:1.0"]

[package.metadata.apple]
resources = ["resources"]
release_build_targets = ["aarch64-apple-ios"]

[package.metadata.apple.info_plist]
CFBundleIdentifier = "com.example.doctor"
MinimumOSVersion = "15.0"
"#,
        );
        for directory in [project.join("assets"), project.join("resources")] {
            fs::create_dir_all(directory).unwrap();
        }
        fs::write(project.join("icon.png"), "icon").unwrap();
        let report = diagnose(
            &DoctorRequest {
                project: Some(project),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        for id in [
            "project.cargo.manifest",
            "project.cargo.package",
            "project.crossbow.metadata",
            "project.apple.metadata",
            "project.apple.bundle_identifier",
            "project.apple.deployment_target",
            "project.apple.target.aarch64-apple-ios",
            "project.apple.assets",
            "project.apple.icon",
        ] {
            assert_eq!(
                report
                    .checks
                    .iter()
                    .find(|check| check.id == id)
                    .unwrap()
                    .status,
                CheckStatus::Pass,
                "{id}"
            );
        }
        assert_eq!(
            report
                .checks
                .iter()
                .find(|check| check.id == "project.apple.plugin.camera-android")
                .unwrap()
                .status,
            CheckStatus::Skip
        );
    }

    #[test]
    fn invalid_apple_target_fails_typed_metadata() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        write_project(
            &project,
            r#"
[package]
name = "invalid-target"
version = "0.1.0"
edition = "2024"

[package.metadata.apple]
release_build_targets = ["sparc-apple-ios"]
"#,
        );
        let report = diagnose(
            &DoctorRequest {
                project: Some(project),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        assert_eq!(
            report
                .checks
                .iter()
                .find(|check| check.id == "project.apple.metadata")
                .unwrap()
                .status,
            CheckStatus::Fail
        );
    }

    #[cfg(feature = "android")]
    #[test]
    fn unrequested_invalid_android_metadata_does_not_break_apple_checks() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        write_project(
            &project,
            r#"
[package]
name = "platform-isolation"
version = "0.1.0"
edition = "2024"

[package.metadata.android]
release_build_targets = ["not-an-android-target"]

[package.metadata.apple.info_plist]
CFBundleIdentifier = "com.example.isolated"
"#,
        );
        let report = diagnose(
            &DoctorRequest {
                project: Some(project),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        for id in ["project.crossbow.metadata", "project.apple.metadata"] {
            assert_eq!(
                report
                    .checks
                    .iter()
                    .find(|check| check.id == id)
                    .unwrap()
                    .status,
                CheckStatus::Pass,
                "{id}"
            );
        }
    }

    #[test]
    fn diagnosis_does_not_mutate_the_project_or_leak_signing_values() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        let secret = "SUPER-SECRET-SIGNING-VALUE";
        write_project(
            &project,
            r#"
[package]
name = "read-only"
version = "0.1.0"
edition = "2024"

[package.metadata.apple]
release_build_targets = ["aarch64-apple-ios"]
"#,
        );
        let before = snapshot(&project);
        let report = diagnose(
            &DoctorRequest {
                project: Some(project.clone()),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "macos".into(),
                commands: [(
                    "apple.signing.identities".into(),
                    CommandObservation {
                        success: true,
                        stdout: format!("1) ABCDEF \"{secret}\"\n  1 valid identities found"),
                        stderr: secret.into(),
                    },
                )]
                .into_iter()
                .collect(),
                ..Environment::default()
            },
        );
        assert_eq!(before, snapshot(&project));
        assert_eq!(
            report
                .checks
                .iter()
                .find(|check| check.id == "apple.signing.identity")
                .unwrap()
                .status,
            CheckStatus::Pass
        );
        assert!(!serde_json::to_string(&report).unwrap().contains(secret));
        assert_eq!(
            report
                .checks
                .iter()
                .find(|check| check.id == "project.apple.signing")
                .unwrap()
                .status,
            CheckStatus::Skip
        );
    }

    #[test]
    fn virtual_workspace_requires_an_explicit_package_path() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(
            temp.path().join("Cargo.toml"),
            "[workspace]\nmembers = [\"app\"]\ndefault-members = [\"app\"]\nresolver = \"2\"\n",
        )
        .unwrap();
        let app = temp.path().join("app");
        write_project(
            &app,
            r#"
[package]
name = "selected-app"
version = "0.1.0"
edition = "2024"

[package.metadata.apple.info_plist]
CFBundleIdentifier = "com.example.selected"
"#,
        );
        let workspace_report = diagnose(
            &DoctorRequest {
                project: Some(temp.path().to_owned()),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        assert_eq!(
            workspace_report
                .checks
                .iter()
                .find(|check| check.id == "project.cargo.manifest")
                .unwrap()
                .status,
            CheckStatus::Fail
        );
        let report = diagnose(
            &DoctorRequest {
                project: Some(app),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        let package = report
            .checks
            .iter()
            .find(|check| check.id == "project.cargo.package")
            .unwrap();
        assert_eq!(package.status, CheckStatus::Pass);
        assert!(package.summary.contains("selected-app"));
    }

    #[test]
    fn apple_asset_and_icon_checks_require_expected_file_types() {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        write_project(
            &project,
            r#"
[package]
name = "wrong-path-types"
version = "0.1.0"
edition = "2024"

[package.metadata]
assets = ["asset-file"]
icon = "icon-directory"
"#,
        );
        fs::write(project.join("asset-file"), "not a directory").unwrap();
        fs::create_dir(project.join("icon-directory")).unwrap();
        let report = diagnose(
            &DoctorRequest {
                project: Some(project),
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        for id in ["project.apple.assets", "project.apple.icon"] {
            assert_eq!(
                report
                    .checks
                    .iter()
                    .find(|check| check.id == id)
                    .unwrap()
                    .status,
                CheckStatus::Fail,
                "{id}"
            );
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct SnapshotEntry {
        path: PathBuf,
        is_dir: bool,
        contents: Vec<u8>,
        readonly: bool,
        modified: Option<std::time::SystemTime>,
        #[cfg(unix)]
        mode: u32,
    }

    fn snapshot(root: &Path) -> Vec<SnapshotEntry> {
        fn visit(root: &Path, path: &Path, entries: &mut Vec<SnapshotEntry>) {
            let mut children = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            children.sort();
            for child in children {
                let metadata = fs::metadata(&child).unwrap();
                entries.push(SnapshotEntry {
                    path: child.strip_prefix(root).unwrap().to_owned(),
                    is_dir: metadata.is_dir(),
                    contents: if metadata.is_file() {
                        fs::read(&child).unwrap()
                    } else {
                        Vec::new()
                    },
                    readonly: metadata.permissions().readonly(),
                    modified: metadata.modified().ok(),
                    #[cfg(unix)]
                    mode: {
                        use std::os::unix::fs::PermissionsExt;
                        metadata.permissions().mode()
                    },
                });
                if metadata.is_dir() {
                    visit(root, &child, entries);
                }
            }
        }
        let mut entries = Vec::new();
        visit(root, root, &mut entries);
        entries
    }

    #[test]
    fn dynamic_id_normalization_is_stable() {
        assert_eq!(normalize_id("Fancy_Plugin++iOS"), "fancy-plugin-ios");
        assert_eq!(normalize_id("---"), "unnamed");
    }

    #[test]
    fn discovery_command_registry_is_read_only() {
        let expected: &[(&str, &str, &[&str])] = &[
            ("apple.developer_dir", "xcode-select", &["--print-path"]),
            ("apple.xcode.version", "xcodebuild", &["-version"]),
            ("apple.command_line_tools", "xcrun", &["--find", "clang"]),
            ("apple.simctl", "xcrun", &["--find", "simctl"]),
            (
                "apple.sdk.iphoneos",
                "xcrun",
                &["--sdk", "iphoneos", "--show-sdk-path"],
            ),
            (
                "apple.sdk.iphonesimulator",
                "xcrun",
                &["--sdk", "iphonesimulator", "--show-sdk-path"],
            ),
        ];
        assert_eq!(READ_ONLY_COMMANDS, expected);
        assert_eq!(read_only_commands(false).collect::<Vec<_>>(), expected);
        assert_eq!(
            read_only_commands(true).last(),
            Some((
                "apple.signing.identities",
                "security",
                &["find-identity", "-v", "-p", "codesigning"][..],
            ))
        );
    }
}
