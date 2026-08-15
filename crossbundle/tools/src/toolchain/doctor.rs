use super::{Compatibility, CompatibilityPolicy};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

#[cfg(feature = "android")]
use crate::types::{AndroidConfig, IntoRustTriple};

#[cfg(feature = "apple")]
mod apple;
mod project;

pub const DOCTOR_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
    Skip,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DoctorScope {
    Host,
    Project,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DoctorPlatform {
    Android,
    Apple,
}

impl DoctorPlatform {
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Apple => "apple",
        }
    }

    pub const fn enabled(self) -> bool {
        match self {
            Self::Android => cfg!(feature = "android"),
            Self::Apple => cfg!(feature = "apple"),
        }
    }
}

impl std::fmt::Display for DoctorPlatform {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.canonical_name())
    }
}

impl FromStr for DoctorPlatform {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "android" => Ok(Self::Android),
            "apple" => Ok(Self::Apple),
            other => Err(format!(
                "unknown platform {other:?}; expected one of: android, apple"
            )),
        }
    }
}

pub fn enabled_platforms() -> Vec<DoctorPlatform> {
    [DoctorPlatform::Android, DoctorPlatform::Apple]
        .into_iter()
        .filter(|platform| platform.enabled())
        .collect()
}

pub fn resolve_platforms(
    requested: &[DoctorPlatform],
) -> Result<Vec<DoctorPlatform>, DoctorPlatform> {
    let mut platforms = if requested.is_empty() {
        enabled_platforms()
    } else {
        requested.to_vec()
    };
    platforms.sort_unstable();
    platforms.dedup();
    if let Some(disabled) = platforms.iter().find(|platform| !platform.enabled()) {
        return Err(*disabled);
    }
    Ok(platforms)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckSummary {
    pub pass: usize,
    pub warn: usize,
    pub fail: usize,
    pub skip: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObservedValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompatibilityExpectation {
    pub preferred: String,
    pub supported: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DoctorCheck {
    pub id: String,
    pub status: CheckStatus,
    pub category: String,
    pub summary: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<ObservedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<CompatibilityExpectation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DoctorReport {
    pub schema_version: u32,
    pub command: String,
    pub scope: DoctorScope,
    pub strict: bool,
    pub platforms: Vec<DoctorPlatform>,
    pub status: ReportStatus,
    pub summary: CheckSummary,
    pub checks: Vec<DoctorCheck>,
}

impl DoctorReport {
    pub fn recompute(&mut self) {
        self.summary = CheckSummary::default();
        for check in &self.checks {
            match check.status {
                CheckStatus::Pass => self.summary.pass += 1,
                CheckStatus::Warn => self.summary.warn += 1,
                CheckStatus::Fail => self.summary.fail += 1,
                CheckStatus::Skip => self.summary.skip += 1,
            }
        }
        self.status = if self.summary.fail > 0 {
            ReportStatus::Fail
        } else if self.summary.warn > 0 {
            ReportStatus::Warn
        } else {
            ReportStatus::Pass
        };
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandObservation {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Environment {
    pub host_os: String,
    pub variables: BTreeMap<String, String>,
    pub path_entries: Vec<PathBuf>,
    /// Captured outputs of the fixed, read-only Apple discovery commands.
    pub commands: BTreeMap<String, CommandObservation>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            host_os: std::env::consts::OS.into(),
            variables: BTreeMap::new(),
            path_entries: Vec::new(),
            commands: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DoctorRequest {
    pub project: Option<PathBuf>,
    pub strict: bool,
    /// Canonically ordered selected platforms. Empty means all compiled platforms.
    pub platforms: Vec<DoctorPlatform>,
    /// Explicit build targets. Empty means use the project's configured targets.
    pub targets: Vec<String>,
}

impl Environment {
    pub fn discover() -> Self {
        Self {
            host_os: std::env::consts::OS.into(),
            variables: std::env::vars().collect(),
            path_entries: std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
                .collect(),
            commands: BTreeMap::new(),
        }
    }

    fn discover_for(
        _request: &DoctorRequest,
        _platforms: &[DoctorPlatform],
        _project: Option<&project::ProjectContext>,
    ) -> Self {
        let environment = Self::discover();
        #[cfg(feature = "apple")]
        let environment = {
            let mut environment = environment;
            if _platforms.contains(&DoctorPlatform::Apple) {
                environment.commands =
                    apple::discover_read_only_commands(apple::signing_relevant(_request, _project));
            }
            environment
        };
        environment
    }

    pub fn executable(&self, name: &str) -> Option<PathBuf> {
        self.executable_matching(name, Path::is_file)
    }

    fn executable_matching(
        &self,
        name: &str,
        predicate: impl Fn(&Path) -> bool,
    ) -> Option<PathBuf> {
        let names = if cfg!(windows) {
            vec![format!("{name}.exe"), format!("{name}.bat"), name.into()]
        } else {
            vec![name.into()]
        };
        self.path_entries
            .iter()
            .flat_map(|dir| names.iter().map(move |name| dir.join(name)))
            .find(|path| predicate(path))
    }

    #[cfg(feature = "android")]
    fn variable_path(&self, names: &[&str]) -> Option<(PathBuf, String)> {
        names.iter().find_map(|name| {
            self.variables
                .get(*name)
                .map(|value| (PathBuf::from(value), (*name).to_owned()))
        })
    }
}

pub fn diagnose_current(request: &DoctorRequest) -> DoctorReport {
    let (platforms, project) = diagnosis_context(request);
    let environment = Environment::discover_for(request, &platforms, project.as_ref());
    diagnose_with_context(request, &environment, platforms, project)
}

pub fn diagnose(request: &DoctorRequest, environment: &Environment) -> DoctorReport {
    let (platforms, project) = diagnosis_context(request);
    diagnose_with_context(request, environment, platforms, project)
}

fn diagnosis_context(
    request: &DoctorRequest,
) -> (Vec<DoctorPlatform>, Option<project::ProjectContext>) {
    let platforms = resolve_platforms(&request.platforms)
        .expect("doctor platforms must be validated before diagnosis");
    let project = request
        .project
        .as_deref()
        .map(|path| project::ProjectContext::load(path, &platforms));
    (platforms, project)
}

fn diagnose_with_context(
    request: &DoctorRequest,
    environment: &Environment,
    platforms: Vec<DoctorPlatform>,
    project: Option<project::ProjectContext>,
) -> DoctorReport {
    let policy = CompatibilityPolicy::embedded();
    let mut checks = vec![
        executable_check(environment, "host.rust.cargo", "Rust", "cargo", true),
        executable_check(environment, "host.rust.rustc", "Rust", "rustc", true),
    ];
    if let Some(project) = &project {
        checks.extend(project.common_checks());
    }
    #[cfg(feature = "android")]
    if platforms.contains(&DoctorPlatform::Android) {
        checks.extend(android_checks(
            request,
            environment,
            &policy,
            project.as_ref(),
        ));
    }
    #[cfg(feature = "apple")]
    if platforms.contains(&DoctorPlatform::Apple) {
        checks.extend(apple::checks(
            request,
            environment,
            &policy,
            project.as_ref(),
        ));
    }
    finish_report(checks, request, platforms)
}

#[cfg(feature = "android")]
fn android_checks(
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
                compatible_child(&sdk_path.join("ndk"), "", policy.tool("ndk"))
                    .map(|p| (p, "Android SDK/ndk".into()))
            });
        checks.push(versioned_path_check(
            "android.ndk",
            "Android",
            ndk,
            true,
            policy.tool("ndk"),
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
        policy.tool("bundletool"),
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
        policy.tool("android-sdk"),
    );
    let build_tools = compatible_child(&sdk.join("build-tools"), "", policy.tool("build-tools"));
    vec![
        versioned_path_check(
            "android.sdk.platform",
            "Android",
            platform.map(|p| (p, source.into())),
            true,
            policy.tool("android-sdk"),
            strict,
            "Install the preferred Android platform with sdkmanager",
        ),
        versioned_path_check(
            "android.sdk.build_tools",
            "Android",
            build_tools.map(|p| (p, source.into())),
            true,
            policy.tool("build-tools"),
            strict,
            "Install the preferred Android build-tools with sdkmanager",
        ),
    ]
}

#[cfg(feature = "android")]
fn compatible_child(
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

fn executable_check(
    env: &Environment,
    id: &str,
    category: &str,
    executable: &str,
    required: bool,
) -> DoctorCheck {
    match env.executable(executable) {
        Some(path) => check(
            id,
            CheckStatus::Pass,
            category,
            format!("Found {executable}"),
            required,
            Some(ObservedValue {
                version: None,
                path: Some(path),
            }),
            None,
            None,
        ),
        None if required => check(
            id,
            CheckStatus::Fail,
            category,
            format!("{executable} was not found"),
            required,
            None,
            None,
            Some(format!("Install {executable} and add it to PATH")),
        ),
        None => skipped(
            id,
            category,
            required,
            &format!("Optional tool {executable} was not found"),
        ),
    }
}

#[cfg(feature = "android")]
fn versioned_executable_check(
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
fn executable_version_from_metadata(environment: &Environment, executable: &str) -> Option<String> {
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
fn bundletool_check(
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

fn classified_observation(
    id: &str,
    category: &str,
    path: PathBuf,
    version: Option<String>,
    required: bool,
    policy: Option<&super::VersionPolicy>,
    strict: bool,
    remediation: String,
) -> DoctorCheck {
    let compatibility = match (&version, policy) {
        (Some(version), Some(policy)) => policy.classify(version),
        _ => Compatibility::Unknown,
    };
    let status = compatibility_status(compatibility, strict);
    let summary = match compatibility {
        Compatibility::Preferred => "Preferred version is installed",
        Compatibility::Supported => "Installed version is supported but not preferred",
        Compatibility::Unsupported => "Installed version is unsupported",
        Compatibility::Unknown => "Installed version compatibility is unknown",
    };
    check(
        id,
        status,
        category,
        summary.into(),
        required,
        Some(ObservedValue {
            version,
            path: Some(path),
        }),
        policy.map(expectation),
        (status != CheckStatus::Pass).then_some(remediation),
    )
}

fn version_in_text(text: &str) -> Option<String> {
    let start = text.find(|c: char| c.is_ascii_digit())?;
    let version: String = text[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let version = version.trim_end_matches('.');
    (!version.is_empty()).then(|| version.to_owned())
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

fn expectation(policy: &super::VersionPolicy) -> CompatibilityExpectation {
    CompatibilityExpectation {
        preferred: policy.preferred.clone(),
        supported: policy.supported.clone(),
    }
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
                .get_android_assets()
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
                .get_android_resources()
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
                policy.tool("android-sdk"),
                strict,
            ));
        }
        if let Some(minimum) = uses_sdk.min_sdk_version {
            checks.push(project_sdk_check(
                "project.android.min_sdk",
                minimum,
                policy.tool("android-min-sdk"),
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

#[cfg(any(feature = "android", feature = "apple"))]
#[derive(Clone, Copy)]
enum PathRequirement {
    #[cfg(feature = "android")]
    Exists,
    #[cfg(feature = "apple")]
    ReadableDirectory,
    #[cfg(feature = "apple")]
    ReadableFile,
}

#[cfg(any(feature = "android", feature = "apple"))]
struct PathMessages {
    valid: &'static str,
    invalid: &'static str,
    invalid_label: &'static str,
}

#[cfg(any(feature = "android", feature = "apple"))]
impl PathRequirement {
    fn is_met(self, path: &Path) -> bool {
        match self {
            #[cfg(feature = "android")]
            Self::Exists => path.exists(),
            #[cfg(feature = "apple")]
            Self::ReadableDirectory => path.is_dir() && std::fs::read_dir(path).is_ok(),
            #[cfg(feature = "apple")]
            Self::ReadableFile => path.is_file() && std::fs::File::open(path).is_ok(),
        }
    }

    fn messages(self) -> PathMessages {
        match self {
            #[cfg(feature = "android")]
            Self::Exists => PathMessages {
                valid: "exist",
                invalid: "do not exist",
                invalid_label: "missing",
            },
            #[cfg(feature = "apple")]
            Self::ReadableDirectory => PathMessages {
                valid: "are readable directories",
                invalid: "are not readable directories",
                invalid_label: "invalid",
            },
            #[cfg(feature = "apple")]
            Self::ReadableFile => PathMessages {
                valid: "is a readable file",
                invalid: "is not a readable file",
                invalid_label: "invalid",
            },
        }
    }

    fn description(self, valid: bool) -> &'static str {
        let messages = self.messages();
        if valid {
            messages.valid
        } else {
            messages.invalid
        }
    }

    fn invalid_label(self) -> &'static str {
        self.messages().invalid_label
    }
}

#[cfg(any(feature = "android", feature = "apple"))]
fn project_paths_check(
    id: &str,
    category: &str,
    label: &str,
    paths: Vec<PathBuf>,
    required_when_present: bool,
    requirement: PathRequirement,
) -> DoctorCheck {
    if paths.is_empty() {
        return skipped(id, category, false, &format!("No {label} are configured"));
    }
    let invalid = paths
        .iter()
        .filter(|path| !requirement.is_met(path))
        .count();
    if invalid == 0 {
        check(
            id,
            CheckStatus::Pass,
            category,
            format!(
                "All configured {label} paths {}",
                requirement.description(true)
            ),
            required_when_present,
            None,
            None,
            None,
        )
    } else {
        check(
            id,
            CheckStatus::Fail,
            category,
            format!(
                "{invalid} configured {label} path(s) {}",
                requirement.description(false)
            ),
            true,
            None,
            None,
            Some(format!(
                "Fix or remove {} {label} paths",
                requirement.invalid_label()
            )),
        )
    }
}

#[cfg(any(feature = "android", feature = "apple"))]
fn project_path_check(
    id: &str,
    category: &str,
    label: &str,
    path: Option<PathBuf>,
    required_when_present: bool,
    requirement: PathRequirement,
) -> DoctorCheck {
    let Some(path) = path else {
        return skipped(id, category, false, &format!("No {label} is configured"));
    };
    let valid = requirement.is_met(&path);
    check(
        id,
        if valid {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        },
        category,
        format!("Configured {label} {}", requirement.description(valid)),
        if valid { required_when_present } else { true },
        Some(ObservedValue {
            version: None,
            path: Some(path),
        }),
        None,
        (!valid).then(|| format!("Fix or remove the {label} path")),
    )
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
fn project_rust_targets_check(
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

fn rust_sysroot(environment: &Environment, project_dir: &Path) -> Option<PathBuf> {
    let rustup_home = environment
        .variables
        .get("RUSTUP_HOME")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".rustup")))?;
    let toolchain = environment
        .variables
        .get("RUSTUP_TOOLCHAIN")
        .cloned()
        .or_else(|| project_toolchain(project_dir))
        .or_else(|| {
            let settings = std::fs::read_to_string(rustup_home.join("settings.toml")).ok()?;
            toml::from_str::<toml::Value>(&settings)
                .ok()?
                .get("default_toolchain")?
                .as_str()
                .map(str::to_owned)
        })?;
    let toolchains = rustup_home.join("toolchains");
    let exact = toolchains.join(&toolchain);
    if exact.is_dir() {
        return Some(exact);
    }
    let prefix = format!("{toolchain}-");
    let mut matches: Vec<_> = std::fs::read_dir(toolchains)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(&prefix))
        })
        .collect();
    matches.sort();
    matches.pop()
}

fn project_toolchain(project_dir: &Path) -> Option<String> {
    project_dir.ancestors().find_map(|directory| {
        let modern = directory.join("rust-toolchain.toml");
        if modern.is_file() {
            let value =
                toml::from_str::<toml::Value>(&std::fs::read_to_string(modern).ok()?).ok()?;
            return value
                .get("toolchain")?
                .get("channel")?
                .as_str()
                .map(str::to_owned);
        }
        let legacy = directory.join("rust-toolchain");
        if legacy.is_file() {
            std::fs::read_to_string(legacy)
                .ok()
                .map(|value| value.trim().to_owned())
        } else {
            None
        }
    })
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

fn skipped(id: &str, category: &str, required: bool, summary: &str) -> DoctorCheck {
    check(
        id,
        CheckStatus::Skip,
        category,
        summary.into(),
        required,
        None,
        None,
        None,
    )
}

fn compatibility_status(compatibility: Compatibility, strict: bool) -> CheckStatus {
    match compatibility {
        Compatibility::Preferred | Compatibility::Supported => CheckStatus::Pass,
        Compatibility::Unsupported | Compatibility::Unknown if strict => CheckStatus::Fail,
        Compatibility::Unsupported | Compatibility::Unknown => CheckStatus::Warn,
    }
}

fn check(
    id: &str,
    status: CheckStatus,
    category: &str,
    summary: String,
    required: bool,
    found: Option<ObservedValue>,
    expected: Option<CompatibilityExpectation>,
    remediation: Option<String>,
) -> DoctorCheck {
    DoctorCheck {
        id: id.into(),
        status,
        category: category.into(),
        summary,
        required,
        found,
        expected,
        source: None,
        remediation,
    }
}

fn finish_report(
    mut checks: Vec<DoctorCheck>,
    request: &DoctorRequest,
    platforms: Vec<DoctorPlatform>,
) -> DoctorReport {
    checks.sort_by(|a, b| a.id.cmp(&b.id));
    let mut report = DoctorReport {
        schema_version: DOCTOR_SCHEMA_VERSION,
        command: "doctor".into(),
        scope: if request.project.is_some() {
            DoctorScope::Project
        } else {
            DoctorScope::Host
        },
        strict: request.strict,
        platforms,
        status: ReportStatus::Pass,
        summary: CheckSummary::default(),
        checks,
    };
    report.recompute();
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "android")]
    use std::fs;
    #[test]
    fn report_is_sorted_and_json_is_versioned() {
        let report = diagnose(&DoctorRequest::default(), &Environment::default());
        assert_eq!(report.schema_version, 1);
        assert!(report.checks.windows(2).all(|w| w[0].id < w[1].id));
        assert_eq!(serde_json::to_value(report).unwrap()["schema_version"], 1);
    }

    #[test]
    fn enabled_platforms_are_canonical_and_feature_accurate() {
        let platforms = enabled_platforms();
        assert!(platforms.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(
            platforms.contains(&DoctorPlatform::Android),
            cfg!(feature = "android")
        );
        assert_eq!(
            platforms.contains(&DoctorPlatform::Apple),
            cfg!(feature = "apple")
        );
    }

    #[cfg(all(feature = "android", feature = "apple"))]
    #[test]
    fn multi_platform_project_runs_common_checks_once() {
        let report = diagnose(
            &DoctorRequest {
                project: Some(PathBuf::from("/definitely/missing/crossbundle-project")),
                platforms: vec![DoctorPlatform::Apple, DoctorPlatform::Android],
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
                .filter(|check| check.id == "project.cargo.manifest")
                .count(),
            1
        );
        assert_eq!(
            report.platforms,
            vec![DoctorPlatform::Android, DoctorPlatform::Apple]
        );
        assert!(report.checks.windows(2).all(|pair| pair[0].id < pair[1].id));
    }

    #[cfg(all(feature = "android", feature = "apple"))]
    #[test]
    fn unrequested_invalid_apple_metadata_does_not_break_android_checks() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join("src")).unwrap();
        fs::write(temp.path().join("src/lib.rs"), "").unwrap();
        fs::write(
            temp.path().join("Cargo.toml"),
            r#"[package]
name = "platform-isolation"
version = "0.1.0"
edition = "2024"

[package.metadata.apple]
release_build_targets = ["not-an-apple-target"]
"#,
        )
        .unwrap();
        let report = diagnose(
            &DoctorRequest {
                project: Some(temp.path().to_owned()),
                platforms: vec![DoctorPlatform::Android],
                ..DoctorRequest::default()
            },
            &Environment::default(),
        );
        assert_eq!(
            report
                .checks
                .iter()
                .find(|check| check.id == "project.crossbow.metadata")
                .unwrap()
                .status,
            CheckStatus::Pass
        );
    }

    #[cfg(feature = "apple")]
    #[test]
    fn non_macos_apple_checks_are_skipped_even_in_strict_mode() {
        let report = diagnose(
            &DoctorRequest {
                strict: true,
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        let apple_checks = report
            .checks
            .iter()
            .filter(|check| check.id.starts_with("apple."))
            .collect::<Vec<_>>();
        assert!(!apple_checks.is_empty());
        assert!(
            apple_checks
                .iter()
                .all(|check| check.status == CheckStatus::Skip)
        );
    }

    #[cfg(feature = "apple")]
    #[test]
    fn apple_json_envelope_and_stable_host_ids_are_golden() {
        let report = diagnose(
            &DoctorRequest {
                strict: true,
                platforms: vec![DoctorPlatform::Apple],
                ..DoctorRequest::default()
            },
            &Environment {
                host_os: "linux".into(),
                ..Environment::default()
            },
        );
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(json["schema_version"], 1);
        assert_eq!(json["command"], "doctor");
        assert_eq!(json["scope"], "host");
        assert_eq!(json["strict"], true);
        assert_eq!(json["platforms"], serde_json::json!(["apple"]));
        assert_eq!(json["status"], "fail");
        assert_eq!(
            json["summary"],
            serde_json::json!({"pass": 0, "warn": 0, "fail": 2, "skip": 12})
        );
        let ids = report
            .checks
            .iter()
            .map(|check| check.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "apple.host.os",
                "apple.rust.target.aarch64-apple-ios-sim",
                "apple.sdk.iphoneos",
                "apple.sdk.iphonesimulator",
                "apple.signing.identity",
                "apple.tool.simctl",
                "apple.tool.xcodebuild",
                "apple.tool.xcrun",
                "apple.xcode.command_line_tools",
                "apple.xcode.developer_dir",
                "apple.xcode.installation",
                "apple.xcode.version",
                "host.rust.cargo",
                "host.rust.rustc",
            ]
        );
    }

    #[test]
    fn strict_mode_does_not_fail_a_supported_version() {
        let policy = super::super::VersionPolicy {
            preferred: "17".into(),
            supported: ">=17, <22".into(),
        };
        let check = classified_observation(
            "host.java.runtime",
            "Java",
            "/java".into(),
            Some("21.0.2".into()),
            true,
            Some(&policy),
            true,
            "Install Java 17".into(),
        );
        assert_eq!(check.status, CheckStatus::Pass);
        assert_eq!(
            check.summary,
            "Installed version is supported but not preferred"
        );
    }

    #[cfg(feature = "android")]
    #[test]
    fn gradle_version_is_discovered_without_running_gradle() {
        let temp = tempfile::tempdir().unwrap();
        let bin = temp.path().join("bin");
        let home = temp.path().join("gradle");
        fs::create_dir_all(&bin).unwrap();
        fs::create_dir_all(home.join("lib")).unwrap();
        fs::write(bin.join("gradle"), "").unwrap();
        fs::write(home.join("lib/gradle-core-api-99.0.jar"), "").unwrap();
        fs::write(home.join("lib/gradle-core-7.5.1.jar"), "").unwrap();
        let environment = Environment {
            variables: [("GRADLE_HOME".into(), home.display().to_string())]
                .into_iter()
                .collect(),
            path_entries: vec![bin],
            ..Environment::default()
        };
        assert_eq!(
            executable_version_from_metadata(&environment, "gradle").as_deref(),
            Some("7.5.1")
        );
    }

    #[cfg(feature = "android")]
    #[test]
    fn gradle_metadata_wins_over_unrelated_numeric_path_components() {
        let temp = tempfile::tempdir().unwrap();
        let bin = temp.path().join("runner-123/bin");
        let home = temp.path().join("gradle");
        fs::create_dir_all(&bin).unwrap();
        fs::create_dir_all(home.join("lib")).unwrap();
        fs::write(bin.join("gradle"), "").unwrap();
        fs::write(home.join("lib/gradle-core-9.5.0.jar"), "").unwrap();
        let environment = Environment {
            variables: [("GRADLE_HOME".into(), home.display().to_string())]
                .into_iter()
                .collect(),
            path_entries: vec![bin],
            ..Environment::default()
        };
        let policy = super::super::VersionPolicy {
            preferred: "9.5.0".into(),
            supported: ">=9.3, <10".into(),
        };
        assert_eq!(
            versioned_executable_check(
                &environment,
                "host.gradle",
                "Gradle",
                "gradle",
                true,
                Some(&policy),
                true,
            )
            .status,
            CheckStatus::Pass
        );
    }

    #[cfg(feature = "android")]
    #[test]
    fn selection_prefers_supported_over_newer_unsupported_installations() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join("28.1.0")).unwrap();
        fs::create_dir(temp.path().join("29.0.0")).unwrap();
        let policy = super::super::VersionPolicy {
            preferred: "27.0.0".into(),
            supported: ">=27, <29".into(),
        };
        assert_eq!(
            compatible_child(temp.path(), "", Some(&policy)),
            Some(temp.path().join("28.1.0"))
        );
    }

    #[cfg(feature = "android")]
    #[test]
    fn rust_target_discovery_uses_fixture_toolchain_without_processes() {
        let temp = tempfile::tempdir().unwrap();
        let rustup = temp.path().join("rustup");
        let project = temp.path().join("workspace/example");
        let sysroot = rustup.join("toolchains/1.97.1-test-host");
        fs::create_dir_all(&project).unwrap();
        fs::create_dir_all(sysroot.join("lib/rustlib/aarch64-linux-android/lib")).unwrap();
        fs::create_dir_all(rustup.join("toolchains/stable-test-host")).unwrap();
        fs::write(
            temp.path().join("workspace/rust-toolchain.toml"),
            "[toolchain]\nchannel = \"1.97.1\"\n",
        )
        .unwrap();
        fs::write(
            rustup.join("settings.toml"),
            "default_toolchain = \"stable-test-host\"\n",
        )
        .unwrap();
        let environment = Environment {
            variables: [("RUSTUP_HOME".into(), rustup.display().to_string())]
                .into_iter()
                .collect(),
            path_entries: vec![],
            ..Environment::default()
        };
        let android = AndroidConfig::default();
        let check = project_rust_targets_check(&android, &environment, &project, false, &[]);
        assert_eq!(check.status, CheckStatus::Pass);
        assert_eq!(
            check.found.unwrap().path.as_deref(),
            Some(sysroot.as_path())
        );
        let requested = vec!["x86_64-linux-android".to_owned()];
        let check = project_rust_targets_check(&android, &environment, &project, false, &requested);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check.summary.contains("x86_64-linux-android"));
        fs::remove_dir_all(sysroot.join("lib/rustlib/aarch64-linux-android")).unwrap();
        let check = project_rust_targets_check(&android, &environment, &project, false, &[]);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check.remediation.unwrap().contains("rustup target add"));
    }

    #[cfg(feature = "android")]
    #[test]
    fn strict_mode_rejects_a_configured_missing_bundletool() {
        let check = bundletool_check(
            &Environment::default(),
            Some(PathBuf::from("/missing/bundletool.jar")),
            None,
            true,
        );
        assert_eq!(check.status, CheckStatus::Fail);
    }
}
