#!/usr/bin/env python3

import importlib.util
import sys
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("check_tool_versions.py")
sys.dont_write_bytecode = True
SPEC = importlib.util.spec_from_file_location("check_tool_versions", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


class AndroidGradleVersionSyncTests(unittest.TestCase):
    def test_canonical_versions_update_gradle_stack_declarations(self) -> None:
        source = '''ext.versions = [
    androidGradlePlugin: "7.0.0",
    compileSdk         : 31,
    minSdk             : 19,
    targetSdk          : 31,
    buildTools         : "30.0.3",
    appcompatVersion   : "1.4.0",
    javaVersion        : 11,
]
ext.libraries = [
    androidGradlePlugin: "com.android.tools.build:gradle:$versions.androidGradlePlugin",
]
'''

        self.assertEqual(
            '''ext.versions = [
    androidGradlePlugin: "9.3.1",
    compileSdk         : 36,
    minSdk             : 23,
    targetSdk          : 36,
    buildTools         : "36.0.0",
    appcompatVersion   : "1.8.0",
    javaVersion        : 17,
]
ext.libraries = [
    androidGradlePlugin: "com.android.tools.build:gradle:$versions.androidGradlePlugin",
]
''',
            MODULE.sync_android_gradle_versions(
                source,
                {
                    "android_gradle_plugin": "9.3.1",
                    "android_api_level": "36",
                    "android_min_sdk": "23",
                    "android_build_tools": "36.0.0",
                    "androidx_appcompat": "1.8.0",
                    "java_bytecode": "17",
                },
            ),
        )


if __name__ == "__main__":
    unittest.main()
