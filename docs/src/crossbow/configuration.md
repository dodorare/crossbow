# Project configuration

## Сonfiguration through metadata

The easiest way to configure a project is with metadata. Here's an example of `Cargo.toml`:

```toml
[package]
name = "game"
version = "0.1.0"
authors = ["Example <example@example.com>"]
edition = "2021"

[dependencies]
crossbow = "0.2.3"

[package.metadata]
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Game"
# Android assets directory path relatively to project path
assets = ["assets"]
# Path to icon with `.png` format that will be provided to generate mipmap resources
icon = "path/to/icon.png"

[package.metadata.android]
# Android application wrapper: supports ndk-glue and sokol. Now ndk-glue used by bevy engine and sokol used by macroquad 
app_wrapper = "quad"
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
# Note: The `name` attribute is normally not required for a queries provider, but is non-optional
# as a workaround for aapt throwing errors about missing `android:name` attribute.
# This will be made optional if/when cargo-apk migrates to aapt2.
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
release_build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"]
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Example"
# Apple targets to build on debug or release.
debug_build_targets = ["aarch64-apple-ios"]
release_build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"].
# Apple resources directory path relatively to project path.
resources = ["res/apple"]
```

### Сonfiguration through separate files

But sometimes you need to configure something more complex. For such cases, a more suitable way is to use separate `AndroidManifest.xml` or/and `Info.plist` files.

To enable this feature, you just need to add this to your `Cargo.toml`:

```toml
[package.metadata.android]
manifest_path = "/path/to/file"

[package.metadata.apple]
info_plist_path = "/path/to/file"
```

and then place `AndroidManifest.xml` or/and `Info.plist` near `Cargo.toml`

```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.rust.game"
    android:versionCode="1"
    android:versionName="1.0">
    <uses-sdk android:minSdkVersion="16"
        android:targetSdkVersion="31" />
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
