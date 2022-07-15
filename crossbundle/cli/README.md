# 🏹 CrossBundle CLI

![splash](https://github.com/dodorare/crossbow/blob/main/.github/assets/splash.png?raw=true)

The **crossbundle** is a command-line tool that encapsulates boring stuff of **Android** and **iOS** build/packaging processes and helps mobile developers to create and maintain applications written in **rust** programming language.

## 👁️‍🗨️ Support status

Packaging status:

| Name | Description | Status |
| ---- | ----------- | ------ |
| Android APK | Default build result method. | ✅ |
| Android AAB | Supported via `--aab` flag. | ✅ |
| Apple Debug IPA | Default build result method. Works only on Simulator and could be run on iPhone with Dev Certificate. | 🆗 |
| Apple Release IPA | Not supported yet. Crossbundle should generate `xcodeproj`, but user should build and sign IPA manually. | 🛠 |

Supported game engines:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [Bevy](https://github.com/bevyengine/bevy) | Default build method. Injects [ndk-glue](https://github.com/rust-windowing/android-ndk-rs/tree/master/ndk-glue) into generated tmp `lib.rs` file. | 🆗 |
| [Macroquad](https://github.com/not-fl3/macroquad) | Supported via `--quad` flag. Works as [cargo-quad-apk](https://github.com/not-fl3/cargo-quad-apk) but with all `crossbundle` features. | ✅ |
| **placeholder** | Don't find your game engine here? Open an issue! We are happy to add support for new engines. | 🛠 |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development

## 🌀 Installation

```sh
cargo install --git=https://github.com/dodorare/crossbow crossbundle
```

See [installation documentation](../../docs/README.md) for more details on how to setup environment on your platform.

---

**_NOTE_**

For the correct work of the tool, you need to set up a development environment (ex. install some libraries and tools - such as Android SDK, Android NDK, XCode, etc).
More information about how to set up the environment in the **Android setup** and **iOS setup** wiki pages.

---

## ⚙️ Cargo.toml Metadata syntax

```toml
[[package.metadata.android]]
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Example"
# The version number shown to users
version_name = "0.1.0"
# Internal version number used to determine whether one version is more recent than another
#
# See https://developer.android.com/guide/topics/manifest/manifest-element
version_code = 1
# Min SDK version
min_sdk_version = 19
# Target SDK version
target_sdk_version = 30
# Max SDK version
max_sdk_version = 31
# Virtual path your application's icon as mipmap resource.
icon = "ic_launcher"

# Path to AndroidManifest.xml file
manifest_path = "path/to/AndroidManifest.xml"

# Android package to place in AndroidManifest.xml.
package = "com.example.ExampleProject"
# Android resources directory path relatively to project path.
res = "res/android"
# Android assets directory path relatively to project path.
assets = "assets"
# Android build targets.
build_targets = ["aarch64-linux-android"]

# Adds a uses-permission element to the AndroidManifest.xml.
# Note that android_version 23 and higher, Android requires the application to request permissions at runtime.
[[package.metadata.android.permissions]]
name = "android.permission.INTERNET"

# Specifies that an app wants a particular permission, but only if the app is installed on a device running 
# Android 6.0 (API level 23) or higher. If the device is running API level 22 or lower, the app does not have the specified permission.
#
# See https://developer.android.com/guide/topics/manifest/uses-permission-sdk-23-element
[[package.metadata.android.permissions_sdk_23]]
name = "android.permission.WRITE_EXTERNAL_STORAGE"
max_sdk_version = 30

# See https://developer.android.com/guide/topics/manifest/service-element
[[package.metadata.android.service]]
name = "UpdateService"
intent_filter = []
meta_data = []

# See https://developer.android.com/guide/topics/manifest/queries-element#provider
[[package.metadata.android.queries.provider]]
authorities = "org.khronos.openxr.runtime_broker;org.khronos.openxr.system_runtime_broker"
# Note: The `name` attribute is normally not required for a queries provider, but is non-optional
# as a workaround for aapt throwing errors about missing `android:name` attribute.
# This will be made optional if/when cargo-apk migrates to aapt2.
name = "org.khronos.openxr"

# See https://developer.android.com/guide/topics/manifest/uses-feature-element
#
# Note: there can be multiple .uses_feature entries.
[[package.metadata.android.features]]
name = "android.hardware.vulkan.level"
required = true
version = 1

# See https://developer.android.com/guide/topics/manifest/meta-data-element
[[package.metadata.android.meta_data]]
name = "com.oculus.vr.focusaware"
value = "true"

[package.metadata.apple]
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Example"
# Apple build targets.
build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"]
# Apple resources directory path relatively to project path.
res = "res/apple"
# Apple assets directory path relatively to project path.
assets = "assets"
```

## 🎏 CLI options and flags

To see the complete documentation for each command/subcommand you can write `-h` or `--help`:

```sh
crossbundle -h
crossbundle build android -h
crossbundle run apple -h
crossbundle install -h
# ...
```

Result of `crossbundle -h`:

```text
USAGE:
    crossbundle [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --current-dir <CURRENT_DIR>    The current directory where to run all commands
    -h, --help                         Print help information
    -q, --quiet                        No output printed to stdout
    -v, --verbose                      A level of verbosity, and can be used multiple times
    -V, --version                      Print version information

SUBCOMMANDS:
    build      Starts the process of building/packaging/signing of the rust crate
    help       Print this message or the help of the given subcommand(s)
    install    Installs bundletool and Android Studio's sdkmanager
    log        Attach logger to device with running application
    new        Creates a new Cargo package in the given directory. Project will be ready to
               build with `crossbundle`
    run        Executes `build` command and then deploy and launches the application on the
               device/emulator
```

Result of `crossbundle build android -h`:

```text
Starts the process of building/packaging/signing of the rust crate for Android

USAGE:
    crossbundle build android [OPTIONS]

OPTIONS:
    --aab
        Generating native aab without Java. By default crossbow generating gradle project

    --all-features
        Activate all available features of selected package

    --apk
        Generating native apk without Java. By default crossbow generating gradle project

    --example <EXAMPLE>
        Build the specified example

    --export-path <EXPORT_PATH>
        Path to export Gradle project. By default exports to `target/android/` folder

    --features <FEATURES>
        Space or comma separated list of features to activate. These features only apply to the
        current directory's package. Features of direct dependencies may be enabled with
        `<dep-name>/<feature-name>` syntax. This flag may be specified multiple times, which
        enables all specified features

    -h, --help
        Print help information

    --lib <LIB>
        Compile rust code as a dynamic library [default: crossbow-android]

    --no-default-features
        Do not activate the `default` feature of the current directory's package

    --quad
        Specifies to build macroquad-based game with Sokol application wrapper

    --release
        Build optimized artifact with the `release` profile

    --sign-key-alias <SIGN_KEY_ALIAS>
        Signing key alias

    --sign-key-pass <SIGN_KEY_PASS>
        Signing key password

    --sign-key-path <SIGN_KEY_PATH>
        Path to the signing key

    --target <TARGET>
        Build for the given android architecture. Supported targets are:
        `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`,
        `x86_64-linux-android` [default: aarch64-linux-android]

    --target-dir <TARGET_DIR>
        Directory for generated artifact and intermediate files
```

Result of `crossbundle build apple -h`:

```text
Starts the process of building/packaging/signing of the rust crate for iOS

USAGE:
    crossbundle build apple [OPTIONS]

OPTIONS:
    --all-features
        Activate all available features of selected package

    --bin <BIN>
        Specify custom cargo binary

    --example <EXAMPLE>
        Build the specified example

    --features <FEATURES>
        Space or comma separated list of features to activate. These features only apply to the
        current directory's package. Features of direct dependencies may be enabled with
        `<dep-name>/<feature-name>` syntax. This flag may be specified multiple times, which
        enables all specified features

    -h, --help
        Print help information

    --identity <IDENTITY>
        The id of the identity used for signing. It won't start the signing process until you
        provide this flag

    --no-default-features
        Do not activate the `default` feature of the current directory's package

    --profile-name <PROFILE_NAME>
        Provisioning profile name to find in this directory:
        `~/Library/MobileDevice/Provisioning\ Profiles/`

    --profile-path <PROFILE_PATH>
        Absolute path to provisioning profile

    --quad
        Specifies to build macroquad-based game with Sokol application wrapper

    --release
        Build optimized artifact with the `release` profile

    --target <TARGET>
        Build for the given apple architecture. Supported targets are: `aarch64-apple-ios`,
        `aarch64-apple-ios-sim`, `armv7-apple-ios`, `armv7s-apple-ios`, `i386-apple-ios`,
        `x86_64-apple-ios` [default: aarch64-apple-ios-sim]

    --target-dir <TARGET_DIR>
        Directory for generated artifact and intermediate files

    --team-identifier <TEAM_IDENTIFIER>
        The team identifier of your signing identity
```

## ❌ Troubleshooting

### Shared library "<lib_name>" not found

If you ran into problem of missing shared library in the `apk/aab` - you can fix this by placing your `.so` file into `target/<rust-triple>/<profile>/tools/libname.so`. The builder will pick the library up and put it in the final package.

## 📑 License

Licensed under [Apache-2.0 License](../../LICENSE).
