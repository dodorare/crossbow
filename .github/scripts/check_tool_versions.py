#!/usr/bin/env python3
"""Validate operative platform-tool versions against the canonical manifest."""

from __future__ import annotations

import re
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ANDROID_CONFIGS = (
    "platform/android/java/app/config.gradle",
    "plugins/admob-android/android/config.gradle",
    "plugins/play-billing/android/config.gradle",
    "plugins/play-core/android/config.gradle",
    "plugins/play-games-services/android/config.gradle",
)
ANDROID_GRADLE_VERSION_FIELDS = {
    "androidGradlePlugin": ("android_gradle_plugin", True),
    "compileSdk": ("android_api_level", False),
    "minSdk": ("android_min_sdk", False),
    "targetSdk": ("android_api_level", False),
    "buildTools": ("android_build_tools", True),
    "appcompatVersion": ("androidx_appcompat", True),
    "fragmentVersion": ("androidx_fragment", True),
    "javaVersion": ("java_bytecode", False),
    "ndkVersion": ("android_ndk", True),
}


@dataclass(frozen=True)
class Check:
    path: str
    pattern: str
    expected: str
    label: str
    all_matches: bool = False


def sync_android_gradle_versions(contents: str, versions: dict[str, str]) -> str:
    """Render shared Android stack declarations from the canonical manifest."""
    for gradle_name, (manifest_name, quoted) in ANDROID_GRADLE_VERSION_FIELDS.items():
        pattern = rf"(^\s*{gradle_name}\s*:\s*)(?:\"[0-9.]+\"|[0-9]+)(,?$)"
        if not re.search(pattern, contents, flags=re.MULTILINE):
            continue
        value = versions[manifest_name]
        rendered = f'"{value}"' if quoted else value
        contents = re.sub(
            pattern,
            rf"\g<1>{rendered}\g<2>",
            contents,
            flags=re.MULTILINE,
        )
    return contents


def sync_android_gradle_files(versions: dict[str, str]) -> None:
    for relative_path in ANDROID_CONFIGS:
        path = ROOT / relative_path
        path.write_text(sync_android_gradle_versions(path.read_text(), versions))


def main() -> int:
    versions = tomllib.loads((ROOT / ".github/tool-versions.toml").read_text())
    if sys.argv[1:] == ["--sync-android-gradle"]:
        sync_android_gradle_files(versions)
        print("Android Gradle stack declarations synchronized.")
        return 0
    if sys.argv[1:]:
        print("usage: check_tool_versions.py [--sync-android-gradle]", file=sys.stderr)
        return 2

    checks = [
        Check("rust-toolchain.toml", r'^channel = "([^"]+)"$', versions["rust"], "Rust"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG RUST_VERSION=(\S+)$", versions["rust"], "Rust"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_COMMAND_LINE_TOOLS_VERSION=(\S+)$", versions["android_command_line_tools"], "Android command-line tools"),
        Check("crossbundle/cli/src/commands/install/command_line_tools.rs", r'format!\("commandlinetools-\{\}-([0-9]+)_latest\.zip"', versions["android_command_line_tools"], "Android command-line tools"),
        Check(".github/workflows/ci.yml", r"^  ANDROID_API_LEVEL: '([^']+)'$", versions["android_api_level"], "Android API"),
        Check(".github/workflows/latest-dependencies.yml", r"^  ANDROID_API_LEVEL: '([^']+)'$", versions["android_api_level"], "Android API"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_API_LEVEL=(\S+)$", versions["android_api_level"], "Android API"),
        *[Check(path, r"^    compileSdk\s+: ([0-9]+),$", versions["android_api_level"], "Android API") for path in ANDROID_CONFIGS],
        *[Check(path, r"^    targetSdk\s+: ([0-9]+),$", versions["android_api_level"], "Android API") for path in ANDROID_CONFIGS],
        *[Check(path, r"^    minSdk\s+: ([0-9]+),$", versions["android_min_sdk"], "Android minimum SDK") for path in ANDROID_CONFIGS],
        Check("crossbundle/tools/src/types/android/manifest.rs", r"DEFAULT_ANDROID_MIN_SDK: u32 = ([0-9]+)", versions["android_min_sdk"], "Android minimum SDK"),
        Check("crossbundle/tools/src/types/android/manifest.rs", r"DEFAULT_ANDROID_TARGET_SDK: u32 = ([0-9]+)", versions["android_api_level"], "Android API"),
        Check("examples/crossbow-plugins/Cargo.toml", r"min_sdk_version = ([0-9]+)", versions["android_min_sdk"], "Android minimum SDK"),
        Check("examples/crossbow-plugins/Cargo.toml", r"target_sdk_version = ([0-9]+)", versions["android_api_level"], "Android API"),
        Check("examples/macroquad-permissions/Cargo.toml", r"min_sdk_version = ([0-9]+)", versions["android_min_sdk"], "Android minimum SDK"),
        Check("examples/macroquad-permissions/Cargo.toml", r"target_sdk_version = ([0-9]+)", versions["android_api_level"], "Android API"),
        Check("examples/macroquad-3d/res/AndroidManifest.xml", r'android:minSdkVersion="([0-9]+)"', versions["android_min_sdk"], "Android minimum SDK"),
        Check("examples/macroquad-3d/res/AndroidManifest.xml", r'android:targetSdkVersion="([0-9]+)"', versions["android_api_level"], "Android API"),
        Check("crossbundle/cli/src/commands/install/sdkmanager.rs", r'\.arg\("platforms;android-([0-9]+)"\)', versions["android_api_level"], "Android API"),
        Check("docs/src/crossbundle/command-install.md", r'"platforms;android-([0-9]+)"', versions["android_api_level"], "Android API"),
        Check("docs/src/install/set-up-android-device.md", r'"system-images;android-([0-9]+);', versions["android_api_level"], "Android API", all_matches=True),
        Check("docs/src/install/set-up-android-device.md", r"API level ([0-9]+)\) or higher", versions["android_min_sdk"], "Android minimum SDK"),
        Check("docs/src/crossbow/configuration.md", r'android:targetSdkVersion="([0-9]+)"', versions["android_api_level"], "Android API"),
        Check("crossbundle/cli/tests/cargo_metadata.rs", r'android:targetSdkVersion="([0-9]+)"', versions["android_api_level"], "Android API"),
        Check(".github/workflows/ci.yml", r"^  ANDROID_BUILD_TOOLS_VERSION: '([^']+)'$", versions["android_build_tools"], "Android build tools"),
        Check(".github/workflows/latest-dependencies.yml", r"^  ANDROID_BUILD_TOOLS_VERSION: '([^']+)'$", versions["android_build_tools"], "Android build tools"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_BUILD_TOOLS_VERSION=(\S+)$", versions["android_build_tools"], "Android build tools"),
        Check("crossbundle/cli/src/commands/install/sdkmanager.rs", r'\.arg\("build-tools;([^";]+)"\)', versions["android_build_tools"], "Android build tools"),
        *[Check(path, r'^    buildTools\s+: "([^"]+)",$', versions["android_build_tools"], "Android build tools") for path in ANDROID_CONFIGS],
        Check("docs/src/crossbundle/command-install.md", r'"build-tools;([^";]+)"', versions["android_build_tools"], "Android build tools"),
        Check(".github/workflows/ci.yml", r"^  ANDROID_NDK_VERSION: '([^']+)'$", versions["android_ndk"], "Android NDK"),
        Check(".github/workflows/latest-dependencies.yml", r"^  ANDROID_NDK_VERSION: '([^']+)'$", versions["android_ndk"], "Android NDK"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_NDK_VERSION=(\S+)$", versions["android_ndk"], "Android NDK"),
        Check("crossbundle/cli/src/commands/install/sdkmanager.rs", r'\.arg\("ndk;([^";]+)"\)', versions["android_ndk"], "Android NDK"),
        Check("platform/android/java/app/config.gradle", r'^    ndkVersion\s+: "([^"]+)"$', versions["android_ndk"], "Android NDK"),
        Check("docs/src/crossbundle/command-install.md", r'"ndk;([^";]+)"', versions["android_ndk"], "Android NDK"),
        *[Check(path, r"ndk[/\\]([0-9.]+)", versions["android_ndk"], "Android NDK") for path in ("docs/src/install/android-linux.md", "docs/src/install/android-macos.md", "docs/src/install/android-windows.md")],
        Check(".github/workflows/ci.yml", r"^  BUNDLETOOL_VERSION: '([^']+)'$", versions["bundletool"], "bundletool"),
        Check(".github/workflows/latest-dependencies.yml", r"^  BUNDLETOOL_VERSION: '([^']+)'$", versions["bundletool"], "bundletool"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG BUNDLETOOL_VERSION=(\S+)$", versions["bundletool"], "bundletool"),
        Check("crossbundle/cli/src/commands/install/bundletool.rs", r'default_value = "([^"]+)"', versions["bundletool"], "bundletool"),
        Check("crossbundle/cli/src/commands/install/mod.rs", r'String::from\("([0-9.]+)"\)', versions["bundletool"], "bundletool"),
        Check("docs/src/install/android-windows.md", r"bundletool-all-([0-9.]+)\.jar", versions["bundletool"], "bundletool"),
        Check(".github/workflows/ci.yml", r"^  GRADLE_VERSION: '([^']+)'$", versions["gradle"], "Gradle"),
        Check(".github/workflows/latest-dependencies.yml", r"^  GRADLE_VERSION: '([^']+)'$", versions["gradle"], "Gradle"),
        Check(".github/workflows/publish.yml", r"^        gradle-version: '?([0-9.]+)'?$", versions["gradle"], "Gradle"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG GRADLE_VERSION=(\S+)$", versions["gradle"], "Gradle"),
        Check("platform/android/java/gradle/wrapper/gradle-wrapper.properties", r"gradle-([0-9.]+)-bin\.zip", versions["gradle"], "Gradle"),
        Check("docs/src/install/android-windows.md", r"gradle-([0-9.]+)", versions["gradle"], "Gradle"),
        Check("plugins/play-core/android/gradle/wrapper/gradle-wrapper.properties", r"gradle-([0-9.]+)-bin\.zip", versions["gradle"], "Gradle"),
        Check(".github/workflows/ci.yml", r"^          java-version: '?([0-9]+)'?$", versions["java_runtime"], "Java runtime", all_matches=True),
        Check(".github/workflows/latest-dependencies.yml", r"^          java-version: '?([0-9]+)'?$", versions["java_runtime"], "Java runtime", all_matches=True),
        Check(".github/workflows/publish.yml", r"^        java-version: '?([0-9]+)'?$", versions["java_runtime"], "Java runtime"),
        Check(".github/docker/crossbundle.Dockerfile", r"^        openjdk-([0-9]+)-jdk-headless", versions["java_runtime"], "Java runtime"),
        Check("docs/src/install/android-linux.md", r"openjdk-([0-9]+)-jdk", versions["java_runtime"], "Java runtime"),
        Check("docs/src/install/android-linux.md", r"jdk([0-9]+)-openjdk", versions["java_runtime"], "Java runtime"),
        Check("docs/src/install/android-macos.md", r"openjdk@([0-9]+)", versions["java_runtime"], "Java runtime"),
        Check("docs/src/install/android-windows.md", r"jdk-([0-9]+)", versions["java_runtime"], "Java runtime"),
        *[Check(path, r"^    javaVersion\s+: ([0-9]+),$", versions["java_bytecode"], "Java bytecode target") for path in ANDROID_CONFIGS],
        *[Check(path, r'^    androidGradlePlugin: "([^"]+)",$', versions["android_gradle_plugin"], "Android Gradle plugin") for path in ANDROID_CONFIGS],
        *[Check(path, r'^    appcompatVersion\s+: "([^"]+)",$', versions["androidx_appcompat"], "AndroidX AppCompat") for path in ANDROID_CONFIGS],
        Check("platform/android/java/app/config.gradle", r'^    fragmentVersion\s+: "([^"]+)",$', versions["androidx_fragment"], "AndroidX Fragment"),
        Check("plugins/play-billing/android/build.gradle", r'com\.android\.billingclient:billing:([0-9.]+)', versions["play_billing"], "Play Billing"),
        Check("plugins/play-games-services/android/build.gradle", r'com\.google\.android\.gms:play-services-games-v2:([0-9.]+)', versions["play_games_services"], "Play Games Services"),
        Check("plugins/play-core/android/build.gradle", r'com\.google\.android\.play:app-update-ktx:([0-9.]+)', versions["play_app_update"], "Play In-App Updates"),
        Check("plugins/admob-android/android/build.gradle", r'com\.google\.android\.gms:play-services-ads-lite:([0-9.]+)', versions["google_mobile_ads"], "Google Mobile Ads"),
        Check("plugins/admob-android/android/build.gradle", r'com\.google\.android\.ump:user-messaging-platform:([0-9.]+)', versions["user_messaging_platform"], "User Messaging Platform"),
    ]

    failures: list[str] = []

    ci_workflow = (ROOT / ".github/workflows/ci.yml").read_text()
    for android_path in ("platform/android/**", "plugins/*/android/**"):
        declarations = re.findall(
            rf"^\s+- ['\"]?{re.escape(android_path)}['\"]?$",
            ci_workflow,
            flags=re.MULTILINE,
        )
        if len(declarations) != 2:
            failures.append(
                ".github/workflows/ci.yml: "
                f"{android_path!r} must trigger both push and pull-request CI"
            )

    for workflow_path in (
        ".github/workflows/ci.yml",
        ".github/workflows/latest-dependencies.yml",
    ):
        workflow = (ROOT / workflow_path).read_text()
        if ":play_billing:testDebugUnitTest" not in workflow:
            failures.append(
                f"{workflow_path}: Play Billing contract tests are not executed"
            )

    for check in checks:
        contents = (ROOT / check.path).read_text()
        matches = re.findall(check.pattern, contents, flags=re.MULTILINE)
        if not matches:
            failures.append(f"{check.path}: no operative {check.label} declaration matched")
            continue
        values = matches if check.all_matches else matches[:1]
        if any(value != check.expected for value in values):
            failures.append(
                f"{check.path}: {check.label} is {values!r}; expected {check.expected!r}"
            )

    if failures:
        print("Tool version consistency check failed:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("Operative tool versions are synchronized.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
