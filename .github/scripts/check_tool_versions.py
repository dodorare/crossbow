#!/usr/bin/env python3
"""Ensure duplicated platform-tool versions stay synchronized."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def main() -> int:
    versions = tomllib.loads((ROOT / ".github/tool-versions.toml").read_text())
    checks = {
        "rust": {
            "rust-toolchain.toml": [f'channel = "{versions["rust"]}"'],
            ".github/docker/crossbundle.Dockerfile": [
                f'ARG RUST_VERSION={versions["rust"]}'
            ],
        },
        "android command-line tools": {
            ".github/docker/crossbundle.Dockerfile": [
                "ARG ANDROID_COMMAND_LINE_TOOLS_VERSION="
                f'{versions["android_command_line_tools"]}'
            ],
            "crossbundle/cli/src/commands/install/command_line_tools.rs": [
                f'-{versions["android_command_line_tools"]}_latest.zip'
            ],
        },
        "Android API": {
            ".github/workflows/ci.yml": [
                f"ANDROID_API_LEVEL: '{versions['android_api_level']}'"
            ],
            ".github/workflows/latest-dependencies.yml": [
                f"ANDROID_API_LEVEL: '{versions['android_api_level']}'"
            ],
            ".github/docker/crossbundle.Dockerfile": [
                f'ARG ANDROID_API_LEVEL={versions["android_api_level"]}'
            ],
            "platform/android/java/app/config.gradle": [
                f'compileSdk         : {versions["android_api_level"]}',
                f'targetSdk          : {versions["android_api_level"]}',
            ],
        },
        "Android build tools": {
            ".github/workflows/ci.yml": [
                "ANDROID_BUILD_TOOLS_VERSION: "
                f"'{versions['android_build_tools']}'"
            ],
            ".github/workflows/latest-dependencies.yml": [
                "ANDROID_BUILD_TOOLS_VERSION: "
                f"'{versions['android_build_tools']}'"
            ],
            ".github/docker/crossbundle.Dockerfile": [
                f'ARG ANDROID_BUILD_TOOLS_VERSION={versions["android_build_tools"]}'
            ],
            "crossbundle/cli/src/commands/install/sdkmanager.rs": [
                f'build-tools;{versions["android_build_tools"]}'
            ],
            "platform/android/java/app/config.gradle": [
                f'buildTools         : "{versions["android_build_tools"]}"'
            ],
        },
        "Android NDK": {
            ".github/workflows/ci.yml": [
                f"ANDROID_NDK_VERSION: '{versions['android_ndk']}'"
            ],
            ".github/workflows/latest-dependencies.yml": [
                f"ANDROID_NDK_VERSION: '{versions['android_ndk']}'"
            ],
            ".github/docker/crossbundle.Dockerfile": [
                f'ARG ANDROID_NDK_VERSION={versions["android_ndk"]}'
            ],
            "crossbundle/cli/src/commands/install/sdkmanager.rs": [
                f'ndk;{versions["android_ndk"]}'
            ],
            "platform/android/java/app/config.gradle": [
                f'ndkVersion         : "{versions["android_ndk"]}"'
            ],
            "docs/src/install/android-linux.md": [versions["android_ndk"]],
            "docs/src/install/android-macos.md": [versions["android_ndk"]],
            "docs/src/install/android-windows.md": [versions["android_ndk"]],
        },
        "bundletool": {
            ".github/workflows/ci.yml": [
                f"BUNDLETOOL_VERSION: '{versions['bundletool']}'"
            ],
            ".github/workflows/latest-dependencies.yml": [
                f"BUNDLETOOL_VERSION: '{versions['bundletool']}'"
            ],
            ".github/docker/crossbundle.Dockerfile": [
                f'ARG BUNDLETOOL_VERSION={versions["bundletool"]}'
            ],
            "crossbundle/cli/src/commands/install/bundletool.rs": [
                f'default_value = "{versions["bundletool"]}"'
            ],
            "crossbundle/cli/src/commands/install/mod.rs": [versions["bundletool"]],
            "docs/src/install/android-windows.md": [versions["bundletool"]],
        },
        "Gradle": {
            ".github/workflows/ci.yml": [
                f"GRADLE_VERSION: '{versions['gradle']}'"
            ],
            ".github/workflows/latest-dependencies.yml": [
                f"GRADLE_VERSION: '{versions['gradle']}'"
            ],
            ".github/docker/crossbundle.Dockerfile": [
                f'ARG GRADLE_VERSION={versions["gradle"]}'
            ],
            "docs/src/install/android-windows.md": [
                f'gradle-{versions["gradle"]}'
            ],
        },
        "Java": {
            ".github/workflows/ci.yml": [
                f"java-version: '{versions['java']}'"
            ],
            ".github/workflows/latest-dependencies.yml": [
                f"java-version: '{versions['java']}'"
            ],
            "docs/src/install/android-windows.md": [
                f'jdk-{versions["java"]}'
            ],
        },
    }

    failures: list[str] = []
    for label, files in checks.items():
        for relative_path, expected_fragments in files.items():
            contents = (ROOT / relative_path).read_text()
            for fragment in expected_fragments:
                if fragment not in contents:
                    failures.append(
                        f"{relative_path}: {label} is not synchronized; "
                        f"expected {fragment!r}"
                    )

    if failures:
        print("Tool version consistency check failed:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("Tool versions are synchronized.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
