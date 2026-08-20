# Project configuration

Crossbow's Android baseline is API 36 with a minimum SDK of 23, AGP 9.3.1,
Gradle 9.5.0, Java 17, and NDK 28.2. New Google Play submissions must target
API 36 from August 31, 2026; see the official
[target API requirements](https://developer.android.com/google/play/requirements/target-sdk).

## Configuration through metadata

The easiest way to configure a project is with metadata. Here's an example of `Cargo.toml`:

```toml
[package]
name = "game"
version = "0.1.0"
authors = ["Example <example@example.com>"]
edition = "2024"

[dependencies]
crossbow = "0.2.3"

[package.metadata]
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Game"
# Android assets directory path relative to the project path
assets = ["assets"]
# Path to icon with `.png` format that will be provided to generate mipmap resources
icon = "path/to/icon.png"

# Explicitly import build-time environment variables. Undeclared environment variables are never
# available to configuration templates.
[package.metadata.build_variables]
API_HOST = { env = "API_HOST" }
BUILD_NUMBER = { env = "CI_BUILD_NUMBER", type = "integer" }
APP_CHANNEL = { env = "APP_CHANNEL", default = "development" }
FEATURE_ENABLED = { env = "FEATURE_ENABLED", type = "boolean", default = false }

[package.metadata.android]
# Optional activity integration. The default is "native-activity"; use
# "miniquad" for Macroquad projects built with the Gradle strategy.
runtime = "native-activity"
# Android targets to build on debug or release.
debug_build_targets = ["aarch64-linux-android"]
release_build_targets = ["aarch64-linux-android"]
# Android resources directory path relatively to project path
resources = ["res/android"]

# Complete support of all AndroidManifest.xml attributes
[package.metadata.android.manifest]
package = "com.example.ExampleProject"

# Adds a uses-permission element to the AndroidManifest.xml.
# Note that android_version 23 and higher, Android requires the application to request permissions at runtime
[[package.metadata.android.manifest.uses_permission]]
name = "android.permission.INTERNET"
# Specifies that an app wants a particular permission, but only if the app is installed on a device running
# Android 6.0 (API level 23) or higher. If the device is running API level 22 or lower, the app does not have the specified permission.

# See https://developer.android.com/guide/topics/manifest/uses-permission-sdk-23-element
[[package.metadata.android.manifest.uses_permission_sdk_23]]
name = "android.permission.WRITE_EXTERNAL_STORAGE"
max_sdk_version = 30

# See https://developer.android.com/guide/topics/manifest/service-element
[[package.metadata.android.manifest.service]]
name = "UpdateService"
intent_filter = []
meta_data = []

# See https://developer.android.com/guide/topics/manifest/queries-element#provider
[[package.metadata.android.manifest.queries.provider]]
authorities = "org.khronos.openxr.runtime_broker;org.khronos.openxr.system_runtime_broker"
# The `android-manifest` model currently requires `name` even though Android queries providers
# normally require only `authorities`.
name = "org.khronos.openxr"

# See https://developer.android.com/guide/topics/manifest/uses-feature-element
#
# Note: there can be multiple .uses_feature entries.
[[package.metadata.android.manifest.features]]
name = "android.hardware.vulkan.level"
required = true
version = 1

# See https://developer.android.com/guide/topics/manifest/meta-data-element
[[package.metadata.android.manifest.application.meta_data]]
name = "com.oculus.vr.focusaware"
value = "true"

[package.metadata.apple]
# iOS targets to build in debug or release mode.
debug_build_targets = ["aarch64-apple-ios-sim"]
release_build_targets = ["aarch64-apple-ios"]
# Apple resources directory path relatively to project path.
resources = ["res/apple"]
```

### Build variables

Build variables let the same checked-in configuration produce environment-specific Android and
Apple bundles. Declare every imported value under `package.metadata.build_variables`, then use it
as `{{crossbow.NAME}}` in an inline platform document, an external `AndroidManifest.xml`, or an
external `Info.plist`:

```xml
<!-- AndroidManifest.xml -->
<meta-data
    android:name="com.example.api_host"
    android:value="https://{{crossbow.API_HOST}}/v1" />
```

```xml
<!-- Info.plist -->
<key>APIHost</key>
<string>{{crossbow.API_HOST}}</string>
```

Crossbundle reads the named environment variable first and uses `default` only when it is absent;
an empty environment value therefore overrides the default. A missing value without a default
stops the build with the declaration name. The default type is `string`; `type = "integer"` and
`type = "boolean"` validate environment input and preserve the native type when the placeholder is
the complete metadata or plist value. A placeholder embedded inside a larger string is formatted
as text. Variable values cannot contain other build-variable placeholders.

Only allow-listed variables are readable. The syntax intentionally does not conflict with Android
`${applicationId}` placeholders or Xcode `$(PRODUCT_BUNDLE_IDENTIFIER)` build settings. XML special
characters and Unicode are escaped by the platform serializers, and both XML and binary plists are
supported.

> Build variables are public application configuration, not secrets. Values embedded in
> `AndroidManifest.xml` or `Info.plist` can be inspected by anyone with the built application. Do
> not use this feature for passwords, signing credentials, private keys, or API secrets.

### Configuration through separate files

For more complex configuration, use separate `AndroidManifest.xml` and/or `Info.plist` files.

To enable this feature, you just need to add this to your `Cargo.toml`:

```toml
[package.metadata.android]
manifest_path = "/path/to/file"

[package.metadata.apple]
info_plist_path = "/path/to/file"
```

and then place `AndroidManifest.xml` and/or `Info.plist` near `Cargo.toml`.

```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.rust.game"
    android:versionCode="1"
    android:versionName="1.0">
    <uses-sdk android:minSdkVersion="23"
        android:targetSdkVersion="36" />
    <uses-permission android:name="android.permission.ACCESS_WIFI_STATE"/>
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION"/>
    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION"/>
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    <application android:allowBackup="true"
        android:hasCode="false"
        android:icon="@mipmap/ic_launcher"
        android:label="Game"
        android:theme="@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen">
        <activity android:name="com.rust.game.MainActivity"
            android:label="Game"
            android:configChanges="orientation|keyboardHidden|screenSize">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
```

That's it, this config file will be used for your mobile application.
