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

Game engines supported:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [Bevy](https://github.com/bevyengine/bevy) | Default build method. Injects [ndk-glue](https://github.com/rust-windowing/android-ndk-rs/tree/master/ndk-glue) into generated tmp `lib.rs` file. | ✅ |
| [Macroquad](https://github.com/not-fl3/macroquad) | Supported via `--quad` flag. Works as [cargo-quad-apk](https://github.com/not-fl3/cargo-quad-apk) but with all `crossbundle` features. | ✅ |

P.S: If you don't find your engine here, open an issue! We are happy to add support for new engines.

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
# The user-friendly application name for your app. Displayed in the applications menu
app_name = "Example"
# The version number shown to users
version_name = "0.1.0"
# Internal version number used to determine whether one version is more recent than another
# See https://developer.android.com/guide/topics/manifest/manifest-element
version_code = 1
# Min SDK version
min_sdk_version = 21
# Target SDK version
target_sdk_version = 30
# Max SDK version
max_sdk_version = 31
# Virtual path your application's icon as mipmap resource.
icon = "ic_launcher"

# Use Android.manifest file or generate from Cargo.toml
use_android_manifest = true
# Path to Android.manifest file
android_manifest_path = "path/to/AndroidManifest.xml"

# Use Info.plist file or generate from Cargo.toml
use_info_plist = true
# Path to Info.plist file
info_plist_path = "path/to/Info.plist"

# Android package to place in AndroidManifest.xml.
android_package = "com.example.ExampleProject"
# Android resources directory path relatively to project path.
android_res = "res/android"
# Android assets directory path relatively to project path.
android_assets = "assets"
# Android build targets.
android_build_targets = ["aarch64-linux-android"]

# Apple build targets.
apple_build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"]
# Apple resources directory path relatively to project path.
apple_res = "res/apple"
# Apple assets directory path relatively to project path.
apple_assets = "assets"

# Adds a uses-permission element to the AndroidManifest.xml.
# Note that android_version 23 and higher, Android requires the application to request permissions at runtime.
[[package.metadata.android_permissions]]
name = "android.permission.INTERNET"
max_sdk_version = 21
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

## ❌ Troubleshooting

### Shared library "<lib_name>" not found

If you ran into problem of missing shared library in the `apk/aab` - you can fix this by placing your `.so` file into `target/<rust-triple>/<profile>/tools/libname.so`. The builder will pick the library up and put it in the final package.

## 📑 License

Licensed under [Apache-2.0 License](../../LICENSE).
