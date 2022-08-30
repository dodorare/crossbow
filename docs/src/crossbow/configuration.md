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
crossbow = "0.2.2"

[package.metadata]
app_name = "Game"
assets = ["assets"]
icon = "path/to/icon.png"

[package.metadata.android]
release_build_targets = ["aarch64-linux-android"]
resources = ["res/android"]

[package.metadata.apple]
release_build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"]
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
