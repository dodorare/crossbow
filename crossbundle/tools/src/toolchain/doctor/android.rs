use super::*;

pub(super) fn checks(
    request: &DoctorRequest,
    environment: &Environment,
    policy: &CompatibilityPolicy,
    project: Option<&project::ProjectContext>,
) -> Vec<DoctorCheck> {
    let mut checks = vec![
        java_check(
            environment,
            policy.host_tool("java-runtime"),
            request.strict,
        ),
        executable_check(
            environment,
            "host.java.jarsigner",
            "Java",
            "jarsigner",
            false,
        ),
        versioned_executable_check(
            environment,
            "host.gradle",
            "Gradle",
            "gradle",
            false,
            policy.host_tool("gradle"),
            request.strict,
        ),
    ];

    let sdk = environment
        .variable_path(&["ANDROID_SDK_ROOT", "ANDROID_SDK_PATH", "ANDROID_HOME"])
        .or_else(|| {
            android_tools::sdk_install_path()
                .ok()
                .map(|path| (path, "default Android SDK path".into()))
        });
    checks.push(path_check(
        "android.sdk.root",
        "Android",
        sdk.clone(),
        true,
        "Set ANDROID_SDK_ROOT to an installed Android SDK",
    ));
    if let Some((sdk_path, source)) = sdk.filter(|(path, _)| path.is_dir()) {
        checks.extend(sdk_checks(&sdk_path, &source, policy, request.strict));
        let ndk = environment
            .variable_path(&[
                "ANDROID_NDK_ROOT",
                "ANDROID_NDK_PATH",
                "ANDROID_NDK_HOME",
                "NDK_HOME",
            ])
            .or_else(|| {
                compatible_child(&sdk_path.join("ndk"), "", policy.android_tool("ndk"))
                    .map(|p| (p, "Android SDK/ndk".into()))
            });
        checks.push(versioned_path_check(
            "android.ndk",
            "Android",
            ndk,
            true,
            policy.android_tool("ndk"),
            request.strict,
            "Install the preferred NDK with sdkmanager",
        ));
        let adb =
            sdk_path
                .join("platform-tools")
                .join(if cfg!(windows) { "adb.exe" } else { "adb" });
        checks.push(if adb.is_file() {
            check(
                "android.adb",
                CheckStatus::Pass,
                "Android",
                "Found adb in the Android SDK".into(),
                false,
                Some(ObservedValue {
                    version: None,
                    path: Some(adb),
                }),
                None,
                None,
            )
        } else {
            skipped(
                "android.adb",
                "Android",
                false,
                "Optional tool adb was not found",
            )
        });
    } else {
        checks.push(skipped(
            "android.sdk.platform",
            "Android",
            true,
            "Android SDK root is unavailable",
        ));
        checks.push(skipped(
            "android.adb",
            "Android",
            false,
            "Android SDK root is unavailable",
        ));
        checks.push(skipped(
            "android.sdk.build_tools",
            "Android",
            true,
            "Android SDK root is unavailable",
        ));
        checks.push(skipped(
            "android.ndk",
            "Android",
            true,
            "Android SDK root is unavailable",
        ));
    }
    let bundletool = environment
        .variables
        .get("BUNDLETOOL_PATH")
        .map(PathBuf::from);
    checks.push(bundletool_check(
        environment,
        bundletool,
        policy.android_tool("bundletool"),
        request.strict,
    ));
    if let Some(project) = project {
        checks.extend(project_checks(
            project,
            policy,
            request.strict,
            environment,
            &request.targets,
        ));
    }
    checks
}

#[cfg(feature = "android")]
fn sdk_checks(
    sdk: &Path,
    source: &str,
    policy: &CompatibilityPolicy,
    strict: bool,
) -> Vec<DoctorCheck> {
    let platform = compatible_child(
        &sdk.join("platforms"),
        "android-",
        policy.android_tool("android-sdk"),
    );
    let build_tools = compatible_child(
        &sdk.join("build-tools"),
        "",
        policy.android_tool("build-tools"),
    );
    vec![
        versioned_path_check(
            "android.sdk.platform",
            "Android",
            platform.map(|p| (p, source.into())),
            true,
            policy.android_tool("android-sdk"),
            strict,
            "Install the preferred Android platform with sdkmanager",
        ),
        versioned_path_check(
            "android.sdk.build_tools",
            "Android",
            build_tools.map(|p| (p, source.into())),
            true,
            policy.android_tool("build-tools"),
            strict,
            "Install the preferred Android build-tools with sdkmanager",
        ),
    ]
}

#[cfg(feature = "android")]
pub(super) fn compatible_child(
    parent: &Path,
    prefix: &str,
    policy: Option<&super::VersionPolicy>,
) -> Option<PathBuf> {
    if let Some(policy) = policy {
        let exact = parent.join(format!("{prefix}{}", policy.preferred));
        if exact.is_dir() {
            return Some(exact);
        }
    }
    let mut children: Vec<_> = std::fs::read_dir(parent)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    children.sort_by_key(|path| numeric_name(path));
    if let Some(policy) = policy
        && let Some(path) = children.iter().rev().find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    matches!(
                        policy.classify(name.trim_start_matches(prefix)),
                        Compatibility::Preferred | Compatibility::Supported
                    )
                })
        })
    {
        return Some(path.clone());
    }
    children.pop()
}

#[cfg(feature = "android")]
fn numeric_name(path: &Path) -> Vec<u64> {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .trim_start_matches("android-")
        .split('.')
        .map(|p| p.parse().unwrap_or(0))
        .collect()
}

#[cfg(feature = "android")]
pub(super) fn versioned_executable_check(
    environment: &Environment,
    id: &str,
    category: &str,
    executable: &str,
    required: bool,
    policy: Option<&super::VersionPolicy>,
    strict: bool,
) -> DoctorCheck {
    let Some(path) = environment.executable(executable) else {
        return if required {
            check(
                id,
                CheckStatus::Fail,
                category,
                format!("{executable} was not found"),
                true,
                None,
                policy.map(expectation),
                Some(format!("Install {executable} and add it to PATH")),
            )
        } else {
            skipped(
                id,
                category,
                false,
                &format!("Optional tool {executable} was not found"),
            )
        };
    };
    let version = executable_version_from_metadata(environment, executable).or_else(|| {
        path.ancestors()
            .filter_map(|path| path.file_name()?.to_str())
            .find_map(version_in_text)
    });
    classified_observation(
        id,
        category,
        path,
        version,
        required,
        policy,
        strict,
        format!("Install the preferred {executable} version"),
    )
}

#[cfg(feature = "android")]
pub(super) fn executable_version_from_metadata(
    environment: &Environment,
    executable: &str,
) -> Option<String> {
    if executable != "gradle" {
        return None;
    }
    let home = environment.variables.get("GRADLE_HOME")?;
    let mut versions: Vec<_> = std::fs::read_dir(Path::new(home).join("lib"))
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter_map(|name| {
            name.strip_prefix("gradle-core-")
                .and_then(|name| name.strip_suffix(".jar"))
                .filter(|version| version.as_bytes().first().is_some_and(u8::is_ascii_digit))
                .map(str::to_owned)
        })
        .collect();
    versions.sort_by_key(|version| {
        version
            .split('.')
            .map(|part| part.parse::<u64>().unwrap_or_default())
            .collect::<Vec<_>>()
    });
    versions.pop()
}

#[cfg(feature = "android")]
pub(super) fn bundletool_check(
    environment: &Environment,
    path: Option<PathBuf>,
    policy: Option<&super::VersionPolicy>,
    strict: bool,
) -> DoctorCheck {
    let Some(path) = path else {
        return skipped(
            "android.bundletool",
            "Android",
            false,
            "Optional installation was not found",
        );
    };
    if !path.is_file() {
        return check(
            "android.bundletool",
            if strict {
                CheckStatus::Fail
            } else {
                CheckStatus::Warn
            },
            "Android",
            "Configured bundletool path does not exist".into(),
            false,
            Some(ObservedValue {
                version: None,
                path: Some(path),
            }),
            policy.map(expectation),
            Some("Fix BUNDLETOOL_PATH or install bundletool".into()),
        );
    }
    let version = environment
        .variables
        .get("BUNDLETOOL_VERSION")
        .cloned()
        .or_else(|| {
            path.file_name()
                .and_then(|name| name.to_str())
                .and_then(version_in_text)
        });
    classified_observation(
        "android.bundletool",
        "Android",
        path,
        version,
        false,
        policy,
        strict,
        "Set BUNDLETOOL_VERSION or install the preferred bundletool".into(),
    )
}

#[cfg(feature = "android")]
fn java_check(
    environment: &Environment,
    policy: Option<&super::VersionPolicy>,
    strict: bool,
) -> DoctorCheck {
    let Some(java) = environment.executable("java") else {
        return check(
            "host.java.runtime",
            CheckStatus::Fail,
            "Java",
            "java was not found".into(),
            true,
            None,
            policy.map(expectation),
            Some("Install a supported JDK and add java to PATH".into()),
        );
    };
    let version = environment
        .variables
        .get("JAVA_HOME")
        .and_then(|home| std::fs::read_to_string(Path::new(home).join("release")).ok())
        .and_then(|release| {
            release.lines().find_map(|line| {
                line.strip_prefix("JAVA_VERSION=")
                    .map(|v| v.trim_matches('"').to_owned())
            })
        })
        .or_else(|| {
            let java = std::fs::canonicalize(&java).ok()?;
            let release = std::fs::read_to_string(java.parent()?.parent()?.join("release")).ok()?;
            release.lines().find_map(|line| {
                line.strip_prefix("JAVA_VERSION=")
                    .map(|v| v.trim_matches('"').to_owned())
            })
        });
    classified_observation(
        "host.java.runtime",
        "Java",
        java,
        version,
        true,
        policy,
        strict,
        "Install the preferred JDK".into(),
    )
}

#[cfg(feature = "android")]
fn path_check(
    id: &str,
    category: &str,
    value: Option<(PathBuf, String)>,
    required: bool,
    remediation: &str,
) -> DoctorCheck {
    match value {
        Some((path, source)) if path.is_dir() => {
            let mut c = check(
                id,
                CheckStatus::Pass,
                category,
                "Path exists".into(),
                required,
                Some(ObservedValue {
                    version: None,
                    path: Some(path),
                }),
                None,
                None,
            );
            c.source = Some(source);
            c
        }
        Some((path, source)) => {
            let mut c = check(
                id,
                CheckStatus::Fail,
                category,
                "Configured path does not exist".into(),
                required,
                Some(ObservedValue {
                    version: None,
                    path: Some(path),
                }),
                None,
                Some(remediation.into()),
            );
            c.source = Some(source);
            c
        }
        None => check(
            id,
            CheckStatus::Fail,
            category,
            "Path is not configured".into(),
            required,
            None,
            None,
            Some(remediation.into()),
        ),
    }
}

#[cfg(feature = "android")]
fn versioned_path_check(
    id: &str,
    category: &str,
    value: Option<(PathBuf, String)>,
    required: bool,
    policy: Option<&super::VersionPolicy>,
    strict: bool,
    remediation: &str,
) -> DoctorCheck {
    let Some((path, source)) = value else {
        return if required {
            check(
                id,
                CheckStatus::Fail,
                category,
                "Required installation was not found".into(),
                true,
                None,
                policy.map(expectation),
                Some(remediation.into()),
            )
        } else {
            skipped(id, category, false, "Optional installation was not found")
        };
    };
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .trim_start_matches("android-");
    let version = version_in_text(name).unwrap_or_else(|| name.to_owned());
    let mut check = classified_observation(
        id,
        category,
        path,
        Some(version),
        required,
        policy,
        strict,
        remediation.into(),
    );
    check.source = Some(source);
    check
}

#[cfg(feature = "android")]
fn project_checks(
    context: &project::ProjectContext,
    policy: &CompatibilityPolicy,
    strict: bool,
    environment: &Environment,
    requested_targets: &[String],
) -> Vec<DoctorCheck> {
    let Some(project) = context.project() else {
        return Vec::new();
    };
    let Ok(metadata) = &project.metadata else {
        return Vec::new();
    };
    let android = &metadata.android;
    let base = context.base_dir();
    let mut checks = vec![
        project_paths_check(
            "project.android.assets",
            "Project",
            "assets",
            metadata
                .android_assets()
                .iter()
                .map(|path| base.join(path))
                .collect(),
            false,
            PathRequirement::Exists,
        ),
        project_paths_check(
            "project.android.resources",
            "Project",
            "resources",
            metadata
                .android_resources()
                .iter()
                .map(|path| base.join(path))
                .collect(),
            false,
            PathRequirement::Exists,
        ),
        project_path_check(
            "project.android.icon",
            "Project",
            "icon",
            metadata.icon.as_ref().map(|path| base.join(path)),
            false,
            PathRequirement::Exists,
        ),
        project_path_check(
            "project.android.manifest",
            "Project",
            "manifest_path",
            android.manifest_path.as_ref().map(|path| base.join(path)),
            false,
            PathRequirement::Exists,
        ),
        project_targets_check(android),
        project_rust_targets_check(android, environment, base, strict, requested_targets),
        project_plugins_check(android, base),
    ];
    if let Some(uses_sdk) = android
        .manifest
        .as_ref()
        .and_then(|manifest| manifest.uses_sdk.as_ref())
    {
        if let Some(target) = uses_sdk.target_sdk_version {
            checks.push(project_sdk_check(
                "project.android.target_sdk",
                target,
                policy.android_tool("android-sdk"),
                strict,
            ));
        }
        if let Some(minimum) = uses_sdk.min_sdk_version {
            checks.push(project_sdk_check(
                "project.android.min_sdk",
                minimum,
                policy.android_tool("android-min-sdk"),
                strict,
            ));
        }
    }
    checks
}

#[cfg(feature = "android")]
fn configured_targets(android: &AndroidConfig) -> Vec<&str> {
    android
        .debug_build_targets
        .iter()
        .chain(&android.release_build_targets)
        .map(IntoRustTriple::rust_triple)
        .collect()
}

#[cfg(feature = "android")]
fn project_targets_check(android: &AndroidConfig) -> DoctorCheck {
    let targets = configured_targets(android);
    if targets.is_empty() {
        check(
            "project.android.targets",
            CheckStatus::Warn,
            "Project",
            "No Android targets configured; aarch64-linux-android will be used".into(),
            false,
            None,
            None,
            None,
        )
    } else {
        check(
            "project.android.targets",
            CheckStatus::Pass,
            "Project",
            "Configured Android targets are supported".into(),
            true,
            None,
            None,
            None,
        )
    }
}

#[cfg(feature = "android")]
pub(super) fn project_rust_targets_check(
    android: &AndroidConfig,
    environment: &Environment,
    project_dir: &Path,
    strict: bool,
    requested_targets: &[String],
) -> DoctorCheck {
    let mut targets: Vec<_> = if requested_targets.is_empty() {
        configured_targets(android)
    } else {
        requested_targets.iter().map(String::as_str).collect()
    };
    if targets.is_empty() {
        targets.push("aarch64-linux-android");
    }
    targets.sort_unstable();
    targets.dedup();
    let Some(sysroot) = rust_sysroot(environment, project_dir) else {
        return check(
            "project.android.rust_targets",
            if strict {
                CheckStatus::Fail
            } else {
                CheckStatus::Warn
            },
            "Project",
            "Could not determine the active Rust toolchain target directory".into(),
            true,
            None,
            None,
            Some(
                "Set RUSTUP_TOOLCHAIN/RUSTUP_HOME or install targets with rustup target add".into(),
            ),
        );
    };
    let missing: Vec<_> = targets
        .iter()
        .filter(|target| {
            !sysroot
                .join("lib/rustlib")
                .join(target)
                .join("lib")
                .is_dir()
        })
        .copied()
        .collect();
    if missing.is_empty() {
        check(
            "project.android.rust_targets",
            CheckStatus::Pass,
            "Project",
            "Configured Rust Android targets are installed".into(),
            true,
            Some(ObservedValue {
                version: None,
                path: Some(sysroot),
            }),
            None,
            None,
        )
    } else {
        check(
            "project.android.rust_targets",
            CheckStatus::Fail,
            "Project",
            format!("Rust target(s) are not installed: {}", missing.join(", ")),
            true,
            Some(ObservedValue {
                version: None,
                path: Some(sysroot),
            }),
            None,
            Some(format!("Run rustup target add {}", missing.join(" "))),
        )
    }
}

#[cfg(feature = "android")]
fn project_plugins_check(android: &AndroidConfig, base: &Path) -> DoctorCheck {
    let paths: Vec<_> = android
        .plugins
        .local_projects
        .iter()
        .filter_map(|plugin| plugin.project_dir())
        .map(|path| base.join(path))
        .collect();
    let invalid: Vec<_> = paths
        .iter()
        .filter(|p| !p.is_dir() || !p.join("build.gradle").is_file())
        .collect();
    if !invalid.is_empty() {
        check(
            "project.android.plugins",
            CheckStatus::Fail,
            "Project",
            format!("{} local plugin project(s) are invalid", invalid.len()),
            true,
            None,
            None,
            Some("Each plugin project_dir must exist and contain build.gradle".into()),
        )
    } else if paths.is_empty() {
        skipped(
            "project.android.plugins",
            "Project",
            false,
            "No local Android plugin projects are configured",
        )
    } else {
        check(
            "project.android.plugins",
            CheckStatus::Pass,
            "Project",
            "Local Android plugin projects are valid".into(),
            true,
            None,
            None,
            None,
        )
    }
}

#[cfg(feature = "android")]
fn project_sdk_check(
    id: &str,
    value: u32,
    policy: Option<&super::VersionPolicy>,
    strict: bool,
) -> DoctorCheck {
    let version = value.to_string();
    let compatibility = policy
        .map(|p| p.classify(&version))
        .unwrap_or(Compatibility::Unknown);
    let status = compatibility_status(compatibility, strict);
    check(
        id,
        status,
        "Project",
        match compatibility {
            Compatibility::Preferred => "Project uses the preferred SDK level",
            Compatibility::Supported => "Project SDK level is supported but not preferred",
            Compatibility::Unsupported => "Project SDK level is unsupported",
            Compatibility::Unknown => "Project SDK compatibility is unknown",
        }
        .into(),
        true,
        Some(ObservedValue {
            version: Some(version),
            path: None,
        }),
        policy.map(expectation),
        (status != CheckStatus::Pass).then(|| "Use the preferred Android SDK level".into()),
    )
}
