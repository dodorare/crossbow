#!/usr/bin/env python3
"""Validate operative platform-tool versions against the canonical manifest."""

from __future__ import annotations

import re
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


@dataclass(frozen=True)
class Check:
    path: str
    pattern: str
    expected: str
    label: str
    all_matches: bool = False


def main() -> int:
    versions = tomllib.loads((ROOT / ".github/tool-versions.toml").read_text())
    checks = [
        Check("rust-toolchain.toml", r'^channel = "([^"]+)"$', versions["rust"], "Rust"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG RUST_VERSION=(\S+)$", versions["rust"], "Rust"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_COMMAND_LINE_TOOLS_VERSION=(\S+)$", versions["android_command_line_tools"], "Android command-line tools"),
        Check("crossbundle/cli/src/commands/install/command_line_tools.rs", r'format!\("commandlinetools-\{\}-([0-9]+)_latest\.zip"', versions["android_command_line_tools"], "Android command-line tools"),
        Check(".github/workflows/ci.yml", r"^  ANDROID_API_LEVEL: '([^']+)'$", versions["android_api_level"], "Android API"),
        Check(".github/workflows/latest-dependencies.yml", r"^  ANDROID_API_LEVEL: '([^']+)'$", versions["android_api_level"], "Android API"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_API_LEVEL=(\S+)$", versions["android_api_level"], "Android API"),
        Check("platform/android/java/app/config.gradle", r"^    compileSdk\s+: ([0-9]+),$", versions["android_api_level"], "Android API"),
        Check("platform/android/java/app/config.gradle", r"^    targetSdk\s+: ([0-9]+),$", versions["android_api_level"], "Android API"),
        Check("crossbundle/cli/src/commands/install/sdkmanager.rs", r'\.arg\("platforms;android-([0-9]+)"\)', versions["android_api_level"], "Android API"),
        Check("docs/src/crossbundle/command-install.md", r'"platforms;android-([0-9]+)"', versions["android_api_level"], "Android API"),
        Check("docs/src/install/set-up-android-device.md", r'"system-images;android-([0-9]+);', versions["android_api_level"], "Android API", all_matches=True),
        Check("docs/src/crossbow/configuration.md", r'android:targetSdkVersion="([0-9]+)"', versions["android_api_level"], "Android API"),
        Check("crossbundle/cli/tests/cargo_metadata.rs", r'android:targetSdkVersion="([0-9]+)"', versions["android_api_level"], "Android API"),
        Check(".github/workflows/ci.yml", r"^  ANDROID_BUILD_TOOLS_VERSION: '([^']+)'$", versions["android_build_tools"], "Android build tools"),
        Check(".github/workflows/latest-dependencies.yml", r"^  ANDROID_BUILD_TOOLS_VERSION: '([^']+)'$", versions["android_build_tools"], "Android build tools"),
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG ANDROID_BUILD_TOOLS_VERSION=(\S+)$", versions["android_build_tools"], "Android build tools"),
        Check("crossbundle/cli/src/commands/install/sdkmanager.rs", r'\.arg\("build-tools;([^";]+)"\)', versions["android_build_tools"], "Android build tools"),
        Check("platform/android/java/app/config.gradle", r'^    buildTools\s+: "([^"]+)",$', versions["android_build_tools"], "Android build tools"),
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
        Check(".github/docker/crossbundle.Dockerfile", r"^ARG GRADLE_VERSION=(\S+)$", versions["gradle"], "Gradle"),
        Check("platform/android/java/gradle/wrapper/gradle-wrapper.properties", r"gradle-([0-9.]+)-bin\.zip", versions["gradle"], "Gradle"),
        Check("docs/src/install/android-windows.md", r"gradle-([0-9.]+)", versions["gradle"], "Gradle"),
        Check(".github/workflows/ci.yml", r"^          java-version: '([^']+)'$", versions["java"], "Java", all_matches=True),
        Check(".github/workflows/latest-dependencies.yml", r"^          java-version: '([^']+)'$", versions["java"], "Java", all_matches=True),
        Check("docs/src/install/android-windows.md", r"jdk-([0-9]+)", versions["java"], "Java"),
    ]

    failures: list[str] = []
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
