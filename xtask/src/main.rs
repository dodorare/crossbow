use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

const ANDROID_CONFIGS: &[&str] = &[
    "platform/android/java/app/config.gradle",
    "plugins/admob-android/android/config.gradle",
    "plugins/play-billing/android/config.gradle",
    "plugins/play-core/android/config.gradle",
    "plugins/play-games-services/android/config.gradle",
];

const GRADLE_VERSION_FIELDS: &[(&str, &str, bool)] = &[
    ("androidGradlePlugin", "android_gradle_plugin", true),
    ("compileSdk", "android_api_level", false),
    ("minSdk", "android_min_sdk", false),
    ("targetSdk", "android_api_level", false),
    ("buildTools", "android_build_tools", true),
    ("appcompatVersion", "androidx_appcompat", true),
    ("fragmentVersion", "androidx_fragment", true),
    ("javaVersion", "java_bytecode", false),
    ("ndkVersion", "android_ndk", true),
];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the repository")
        .to_owned()
}

fn read_versions(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let path = root.join(".github/tool-versions.toml");
    let contents = fs::read_to_string(&path).map_err(|error| format!("{path:?}: {error}"))?;
    contents
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .map(|line| {
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("invalid tool version declaration: {line}"))?;
            Ok((
                key.trim().to_owned(),
                value.trim().trim_matches('"').to_owned(),
            ))
        })
        .collect()
}

fn version<'a>(versions: &'a BTreeMap<String, String>, key: &str) -> &'a str {
    versions
        .get(key)
        .unwrap_or_else(|| panic!("missing {key} in .github/tool-versions.toml"))
}

fn sync_gradle_versions(source: &str, versions: &BTreeMap<String, String>) -> String {
    let mut in_versions = false;
    let mut rendered = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "ext.versions = [" {
            in_versions = true;
            rendered.push(line.to_owned());
            continue;
        }
        if in_versions && trimmed == "]" {
            in_versions = false;
            rendered.push(line.to_owned());
            continue;
        }

        let replacement = in_versions.then(|| {
            let colon = line.find(':')?;
            let field = line[..colon].trim();
            let (_, manifest_key, quoted) = GRADLE_VERSION_FIELDS
                .iter()
                .find(|(gradle_field, _, _)| *gradle_field == field)?;
            let value = version(versions, manifest_key);
            let comma = if line.trim_end().ends_with(',') {
                ","
            } else {
                ""
            };
            let value = if *quoted {
                format!("\"{value}\"")
            } else {
                value.to_owned()
            };
            Some(format!("{} {value}{comma}", &line[..=colon]))
        });
        rendered.push(replacement.flatten().unwrap_or_else(|| line.to_owned()));
    }

    let mut output = rendered.join("\n");
    if source.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn sync(root: &Path, versions: &BTreeMap<String, String>) -> Result<(), String> {
    for relative_path in ANDROID_CONFIGS {
        let path = root.join(relative_path);
        let source = fs::read_to_string(&path).map_err(|error| format!("{path:?}: {error}"))?;
        let rendered = sync_gradle_versions(&source, versions);
        fs::write(&path, rendered).map_err(|error| format!("{path:?}: {error}"))?;
    }
    Ok(())
}

fn check_gradle_config(
    path: &Path,
    versions: &BTreeMap<String, String>,
) -> Result<(), Vec<String>> {
    let source = fs::read_to_string(path).map_err(|error| vec![format!("{path:?}: {error}")])?;
    let mut failures = Vec::new();
    let mut found = BTreeSet::new();
    let mut in_versions = false;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "ext.versions = [" {
            in_versions = true;
            continue;
        }
        if in_versions && trimmed == "]" {
            break;
        }
        if !in_versions {
            continue;
        }
        let Some(colon) = line.find(':') else {
            continue;
        };
        let field = line[..colon].trim();
        let Some((_, manifest_key, quoted)) = GRADLE_VERSION_FIELDS
            .iter()
            .find(|(gradle_field, _, _)| *gradle_field == field)
        else {
            continue;
        };
        found.insert(field);
        let value = version(versions, manifest_key);
        let expected = if *quoted {
            format!("\"{value}\"")
        } else {
            value.to_owned()
        };
        let actual = line[colon + 1..].trim().trim_end_matches(',');
        if actual != expected {
            failures.push(format!(
                "{}: {field} is {actual:?}; expected {expected:?}",
                path.display()
            ));
        }
    }

    for required in [
        "androidGradlePlugin",
        "compileSdk",
        "minSdk",
        "targetSdk",
        "buildTools",
        "appcompatVersion",
        "javaVersion",
    ] {
        if !found.contains(required) {
            failures.push(format!("{}: missing {required}", path.display()));
        }
    }
    if path.ends_with("platform/android/java/app/config.gradle") {
        for required in ["fragmentVersion", "ndkVersion"] {
            if !found.contains(required) {
                failures.push(format!("{}: missing {required}", path.display()));
            }
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn check(root: &Path, versions: &BTreeMap<String, String>) -> Result<(), Vec<String>> {
    let mut failures = Vec::new();
    let api = version(versions, "android_api_level");
    let min_sdk = version(versions, "android_min_sdk");
    let build_tools = version(versions, "android_build_tools");
    let ndk = version(versions, "android_ndk");
    let gradle = version(versions, "gradle");
    let java_runtime = version(versions, "java_runtime");
    let bundletool = version(versions, "bundletool");

    for path in ANDROID_CONFIGS {
        if let Err(config_failures) = check_gradle_config(&root.join(path), versions) {
            failures.extend(config_failures);
        }
    }

    let mut expect = |path: &str, declaration: String, label: &str| {
        let path = root.join(path);
        match fs::read_to_string(&path) {
            Ok(contents) if contents.contains(&declaration) => {}
            Ok(_) => failures.push(format!(
                "{}: expected {label} declaration {declaration:?}",
                path.display()
            )),
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    };

    let declarations = [
        (
            "rust-toolchain.toml",
            format!("channel = \"{}\"", version(versions, "rust")),
            "Rust",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("ARG RUST_VERSION={}", version(versions, "rust")),
            "Rust",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!(
                "ARG ANDROID_COMMAND_LINE_TOOLS_VERSION={}",
                version(versions, "android_command_line_tools")
            ),
            "Android command-line tools",
        ),
        (
            "crossbundle/cli/src/commands/install/command_line_tools.rs",
            version(versions, "android_command_line_tools").to_owned(),
            "Android command-line tools",
        ),
        (
            ".github/workflows/ci.yml",
            format!("ANDROID_API_LEVEL: '{api}'"),
            "Android API",
        ),
        (
            ".github/workflows/latest-dependencies.yml",
            format!("ANDROID_API_LEVEL: '{api}'"),
            "Android API",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("ARG ANDROID_API_LEVEL={api}"),
            "Android API",
        ),
        (
            "crossbundle/tools/src/types/android/manifest.rs",
            format!("DEFAULT_ANDROID_MIN_SDK: u32 = {min_sdk}"),
            "Android minimum SDK",
        ),
        (
            "crossbundle/tools/src/types/android/manifest.rs",
            format!("DEFAULT_ANDROID_TARGET_SDK: u32 = {api}"),
            "Android API",
        ),
        (
            "examples/crossbow-plugins/Cargo.toml",
            format!("min_sdk_version = {min_sdk}"),
            "Android minimum SDK",
        ),
        (
            "examples/crossbow-plugins/Cargo.toml",
            format!("target_sdk_version = {api}"),
            "Android API",
        ),
        (
            "examples/macroquad-permissions/Cargo.toml",
            format!("min_sdk_version = {min_sdk}"),
            "Android minimum SDK",
        ),
        (
            "examples/macroquad-permissions/Cargo.toml",
            format!("target_sdk_version = {api}"),
            "Android API",
        ),
        (
            "examples/macroquad-3d/res/AndroidManifest.xml",
            format!("android:minSdkVersion=\"{min_sdk}\""),
            "Android minimum SDK",
        ),
        (
            "examples/macroquad-3d/res/AndroidManifest.xml",
            format!("android:targetSdkVersion=\"{api}\""),
            "Android API",
        ),
        (
            "crossbundle/cli/src/commands/install/sdkmanager.rs",
            format!("platforms;android-{api}"),
            "Android API",
        ),
        (
            "docs/src/crossbundle/command-install.md",
            format!("platforms;android-{api}"),
            "Android API",
        ),
        (
            "docs/src/install/set-up-android-device.md",
            format!("system-images;android-{api}"),
            "Android API",
        ),
        (
            "docs/src/install/set-up-android-device.md",
            format!("API level {min_sdk}) or higher"),
            "Android minimum SDK",
        ),
        (
            "docs/src/crossbow/configuration.md",
            format!("android:targetSdkVersion=\"{api}\""),
            "Android API",
        ),
        (
            "crossbundle/cli/tests/cargo_metadata.rs",
            format!("android:targetSdkVersion=\"{api}\""),
            "Android API",
        ),
        (
            ".github/workflows/ci.yml",
            format!("ANDROID_BUILD_TOOLS_VERSION: '{build_tools}'"),
            "Android build tools",
        ),
        (
            ".github/workflows/latest-dependencies.yml",
            format!("ANDROID_BUILD_TOOLS_VERSION: '{build_tools}'"),
            "Android build tools",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("ARG ANDROID_BUILD_TOOLS_VERSION={build_tools}"),
            "Android build tools",
        ),
        (
            "crossbundle/cli/src/commands/install/sdkmanager.rs",
            format!("build-tools;{build_tools}"),
            "Android build tools",
        ),
        (
            "docs/src/crossbundle/command-install.md",
            format!("build-tools;{build_tools}"),
            "Android build tools",
        ),
        (
            ".github/workflows/ci.yml",
            format!("ANDROID_NDK_VERSION: '{ndk}'"),
            "Android NDK",
        ),
        (
            ".github/workflows/latest-dependencies.yml",
            format!("ANDROID_NDK_VERSION: '{ndk}'"),
            "Android NDK",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("ARG ANDROID_NDK_VERSION={ndk}"),
            "Android NDK",
        ),
        (
            "crossbundle/cli/src/commands/install/sdkmanager.rs",
            format!("ndk;{ndk}"),
            "Android NDK",
        ),
        (
            "docs/src/crossbundle/command-install.md",
            format!("ndk;{ndk}"),
            "Android NDK",
        ),
        (
            ".github/workflows/ci.yml",
            format!("BUNDLETOOL_VERSION: '{bundletool}'"),
            "bundletool",
        ),
        (
            ".github/workflows/latest-dependencies.yml",
            format!("BUNDLETOOL_VERSION: '{bundletool}'"),
            "bundletool",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("ARG BUNDLETOOL_VERSION={bundletool}"),
            "bundletool",
        ),
        (
            "crossbundle/cli/src/commands/install/bundletool.rs",
            bundletool.to_owned(),
            "bundletool",
        ),
        (
            "crossbundle/cli/src/commands/install/mod.rs",
            bundletool.to_owned(),
            "bundletool",
        ),
        (
            "docs/src/install/android-windows.md",
            format!("bundletool-all-{bundletool}.jar"),
            "bundletool",
        ),
        (
            ".github/workflows/ci.yml",
            format!("GRADLE_VERSION: '{gradle}'"),
            "Gradle",
        ),
        (
            ".github/workflows/latest-dependencies.yml",
            format!("GRADLE_VERSION: '{gradle}'"),
            "Gradle",
        ),
        (
            ".github/workflows/publish.yml",
            format!("gradle-version: {gradle}"),
            "Gradle",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("ARG GRADLE_VERSION={gradle}"),
            "Gradle",
        ),
        (
            "platform/android/java/gradle/wrapper/gradle-wrapper.properties",
            format!("gradle-{gradle}-bin.zip"),
            "Gradle",
        ),
        (
            "docs/src/install/android-windows.md",
            format!("gradle-{gradle}"),
            "Gradle",
        ),
        (
            "plugins/play-core/android/gradle/wrapper/gradle-wrapper.properties",
            format!("gradle-{gradle}-bin.zip"),
            "Gradle",
        ),
        (
            ".github/workflows/ci.yml",
            format!("java-version: '{java_runtime}'"),
            "Java runtime",
        ),
        (
            ".github/workflows/latest-dependencies.yml",
            format!("java-version: '{java_runtime}'"),
            "Java runtime",
        ),
        (
            ".github/workflows/publish.yml",
            format!("java-version: {java_runtime}"),
            "Java runtime",
        ),
        (
            ".github/docker/crossbundle.Dockerfile",
            format!("openjdk-{java_runtime}-jdk-headless"),
            "Java runtime",
        ),
        (
            "docs/src/install/android-linux.md",
            format!("openjdk-{java_runtime}-jdk"),
            "Java runtime",
        ),
        (
            "docs/src/install/android-linux.md",
            format!("jdk{java_runtime}-openjdk"),
            "Java runtime",
        ),
        (
            "docs/src/install/android-macos.md",
            format!("openjdk@{java_runtime}"),
            "Java runtime",
        ),
        (
            "docs/src/install/android-windows.md",
            format!("jdk-{java_runtime}"),
            "Java runtime",
        ),
        (
            "plugins/play-billing/android/build.gradle",
            format!(
                "com.android.billingclient:billing:{}",
                version(versions, "play_billing")
            ),
            "Play Billing",
        ),
        (
            "plugins/play-games-services/android/build.gradle",
            format!(
                "com.google.android.gms:play-services-games-v2:{}",
                version(versions, "play_games_services")
            ),
            "Play Games Services",
        ),
        (
            "plugins/play-core/android/build.gradle",
            format!(
                "com.google.android.play:app-update-ktx:{}",
                version(versions, "play_app_update")
            ),
            "Play In-App Updates",
        ),
        (
            "plugins/admob-android/android/build.gradle",
            format!(
                "com.google.android.gms:play-services-ads-lite:{}",
                version(versions, "google_mobile_ads")
            ),
            "Google Mobile Ads",
        ),
        (
            "plugins/admob-android/android/build.gradle",
            format!(
                "com.google.android.ump:user-messaging-platform:{}",
                version(versions, "user_messaging_platform")
            ),
            "User Messaging Platform",
        ),
    ];
    for (path, declaration, label) in declarations {
        expect(path, declaration, label);
    }

    for path in [
        "docs/src/install/android-linux.md",
        "docs/src/install/android-macos.md",
        "docs/src/install/android-windows.md",
    ] {
        expect(path, ndk.to_owned(), "Android NDK");
    }
    for path in [
        ".github/workflows/ci.yml",
        ".github/workflows/latest-dependencies.yml",
    ] {
        expect(
            path,
            ":play_billing:testDebugUnitTest".to_owned(),
            "Play Billing contract tests",
        );
    }

    let ci = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap_or_default();
    for path in ["platform/android/**", "plugins/*/android/**"] {
        if ci.matches(path).count() != 2 {
            failures.push(format!(
                ".github/workflows/ci.yml: {path:?} must trigger push and pull-request CI"
            ));
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn run() -> Result<(), String> {
    let root = repository_root();
    let versions = read_versions(&root)?;
    let args: Vec<_> = env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["android-stack", "check"] => {
            check(&root, &versions).map_err(|failures| failures.join("\n- "))?
        }
        ["android-stack", "sync"] => sync(&root, &versions)?,
        _ => {
            return Err("usage: cargo run -p xtask -- android-stack <check|sync>".to_owned());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Android stack consistency failed:\n- {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_only_updates_version_declarations() {
        let source = r#"ext.versions = [
    androidGradlePlugin: "7.0.0",
    compileSdk         : 31,
    minSdk             : 19,
]
ext.libraries = [
    androidGradlePlugin: "com.android.tools.build:gradle:$versions.androidGradlePlugin",
]
"#;
        let versions = BTreeMap::from([
            ("android_gradle_plugin".to_owned(), "9.3.1".to_owned()),
            ("android_api_level".to_owned(), "36".to_owned()),
            ("android_min_sdk".to_owned(), "23".to_owned()),
        ]);

        assert_eq!(
            sync_gradle_versions(source, &versions),
            r#"ext.versions = [
    androidGradlePlugin: "9.3.1",
    compileSdk         : 36,
    minSdk             : 23,
]
ext.libraries = [
    androidGradlePlugin: "com.android.tools.build:gradle:$versions.androidGradlePlugin",
]
"#
        );
    }
}
