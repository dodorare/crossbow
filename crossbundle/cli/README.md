# Crossbundle CLI

![splash](https://github.com/dodorare/crossbow/blob/main/assets/splash.png?raw=true)

The **crossbundle** is a command-line tool that encapsulates boring stuff of **Android** and **iOS** build/packaging processes and helps mobile developers to create and maintain applications written in **rust** programming language.

## Support status

Supported operating systems for build (**iOS** only on **macOS**):

| Name | Status |
| ---- | ------ |
| Windows | ✅ |
| Linux | ✅ |
| macOS | ✅ |

Packaging Strategy status:

| Name | Description | Status |
| ---- | ----------- | ------ |
| Android APK | Supported via `-s=native-apk` flag. | ✅ |
| Android AAB | Supported via `-s=native-aab` flag. | ✅ |
| Android Gradle | Supported via `-s=gradle-apk` flag. | ✅ |
| Apple Debug APP | Default build strategy. Works only on Simulator and could be run on iPhone with Dev Certificate. | ✅ |
| Apple Debug IPA | Works only on Simulator and could be run on iPhone with Dev Certificate. | 🆗 |
| Apple Release IPA | Not supported yet. Crossbundle should generate `xcodeproj`, but user should build and sign IPA manually. | 🛠 |

Supported game engines:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [Bevy](https://github.com/bevyengine/bevy) | Default build method. Injects [ndk-glue](https://github.com/rust-windowing/android-ndk-rs/tree/master/ndk-glue) into generated tmp `lib.rs` file. | 🆗 |
| [Macroquad](https://github.com/not-fl3/macroquad) | Supported via `app_wrapper = "sokol"` inside `Cargo.toml` metadata. Also, can work as [cargo-quad-apk](https://github.com/not-fl3/cargo-quad-apk) but with all `crossbundle` features. | ✅ |
| **placeholder** | Don't find your game engine here? Open an issue! We are happy to add support for new engines. | 🛠 |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development

## Installation

```sh
cargo install --git=https://github.com/dodorare/crossbow crossbundle
```

See [installation documentation](https://crossbow.dodorare.com/install/index.html) for more details on how to setup environment on your platform.

## Cargo.toml Metadata syntax

```toml
[[package.metadata.android]]
# Cross-platform user-friendly application name for your app.
app_name = "Example"
# Cross-platform assets directory path relatively to project path.
assets = "assets"

[[package.metadata.android]]
# Android application wrapper: supports ndk-glue and sokol
app_wrapper = "sokol"
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Example"
# Path to AndroidManifest.xml file
manifest_path = "path/to/AndroidManifest.xml"
# Android resources directory path relatively to project path.
res = "res/android"
# Android assets directory path relatively to project path.
assets = "assets"
# Android targets to build on debug or release.
debug_build_targets = ["aarch64-linux-android"]
release_build_targets = ["aarch64-linux-android"]

# Complete support of ALL AndroidManifest.xml attributes
[package.metadata.android.manifest]
package = "com.example.ExampleProject"

# Adds a uses-permission element to the AndroidManifest.xml.
# Note that android_version 23 and higher, Android requires the application to request permissions at runtime.
[[package.metadata.android.manifest.uses_permission]]
name = "android.permission.INTERNET"

# Specifies that an app wants a particular permission, but only if the app is installed on a device running
# Android 6.0 (API level 23) or higher. If the device is running API level 22 or lower, the app does not have the specified permission.
#
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
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Example"
# Apple targets to build on debug or release.
debug_build_targets = ["aarch64-apple-ios"]
release_build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"]
# Apple resources directory path relatively to project path.
res = "res/apple"
```

## CLI options and flags

To see the complete documentation for each command/subcommand you can write `-h` or `--help`:

```sh
crossbundle -h
crossbundle build android -h
crossbundle run ios -h
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
    new        Creates a new Cargo package in the given directory. Project will be ready to
               build with `crossbundle`
    run        Executes `build` command and then deploy and launches the application on the
               device/emulator
```

Result of `crossbundle run android -h` (this command extends `crossbundle build android`):

```text
Executes `build` command and then deploy and launches the application on the Android device/emulator

USAGE:
    crossbundle run android [OPTIONS]

OPTIONS:
    --all-features
        Activate all available features of selected package

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
        Only compile rust code as a dynamic library. By default: "crossbow-android"

    --log
        Enable logging attach after run

    --no-default-features
        Do not activate the `default` feature of the current directory's package

    --release
        Build optimized artifact with the `release` profile

    -s, --strategy <STRATEGY>
        Build strategy specifies what and how to build Android application: with help of Gradle,
        or with our native approach [default: gradle-apk]

    --sign-key-alias <SIGN_KEY_ALIAS>
        Signing key alias

    --sign-key-pass <SIGN_KEY_PASS>
        Signing key password

    --sign-key-path <SIGN_KEY_PATH>
        Path to the signing key

    -t, --target <TARGET>...
        Build for the given android architecture. Supported targets are:
        `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`,
        `x86_64-linux-android`

    --target-dir <TARGET_DIR>
        Directory for generated artifact and intermediate files
```

Result of `crossbundle build ios -h` (this command extends `crossbundle build ios`):

```text
Executes `build` command and then deploy and launches the application on the iOS device/emulator

USAGE:
    crossbundle run ios [OPTIONS]

OPTIONS:
    --all-features
        Activate all available features of selected package

    --bin <BIN>
        Specify custom cargo binary

    -d, --debug
        Run in debug mode

    -d, --device
        Install and launch on the connected device

    -D, --device-id <DEVICE_ID>
        Connected device id

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

    --release
        Build optimized artifact with the `release` profile

    -s, --strategy <STRATEGY>
        Build strategy specifies what and how to build iOS application: with help of XCode, or
        with our native approach [default: native-ipa]

    -s, --simulator-name <SIMULATOR_NAME>
        Simulator device name [default: "iPhone 13"]

    -t, --target <TARGET>...
        Build for the given apple architecture. Supported targets are: `aarch64-apple-ios`,
        `aarch64-apple-ios-sim`, `armv7-apple-ios`, `armv7s-apple-ios`, `i386-apple-ios`,
        `x86_64-apple-ios`

    --target-dir <TARGET_DIR>
        Directory for generated artifact and intermediate files

    --team-identifier <TEAM_IDENTIFIER>
        The team identifier of your signing identity
```

## Troubleshooting

### Shared library "<lib_name>" not found

If you ran into problem of missing shared library in the `apk/aab` - you can fix this by placing your `.so` file into `target/<rust-triple>/<profile>/tools/libname.so`. The builder will pick the library up and put it in the final package.

## License

Licensed under [Apache-2.0 License](../../LICENSE).
