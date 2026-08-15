# Crossbundle build command

## Crossbundle build gradle

Crossbow default build process requires installed Gradle on your PC.

To create a project go to the example you want to build and use the command below. The command belongs to macroquad engine examples building:

```sh
crossbundle build android

# To specify custom export gradle directory
crossbundle build android --export-path=./gen/
```

By default build directory is `target/android/<project_name>/gradle`. But you can specify your own build directory via `--export-path=<OUT_PATH>` flag. Go to the directory where Gradle project was built and use command below to manually install APK on the device.

```sh
gradle installDebug
```

Also you can replace `build` with `run` subcommand to build and run APK on your device (it uses `installDebug` command under the hood). To see how to set android emulator check install recommendations for [linux-android](./install-linux-android.md), [macos-android](./install-macos-android.md), [windows-android](./install-windows-android.md).

## Crossbundle build native AAB/APK

If you don't want to use gradle you can specify it in strategy native-apk:

```sh
crossbundle build android -s=native-apk
# or do you need AAB:
crossbundle build android -s=native-aab
```

To find out available commands specify the -h flag.

```sh
crossbundle build android -h
```

## Preview a build without side effects

`--dry-run` resolves and prints the same immutable Android build plan used by a real
build, but never generates files, creates a signing key, compiles, downloads, installs,
or launches anything:

```sh
crossbundle build android --dry-run
crossbundle build android --dry-run --json
crossbundle run android --dry-run --json
```

The JSON plan has a versioned envelope and ordered, stable step IDs. Paths and signing
inputs may be reported, but signing passwords and other secret values are never stored
in a plan.

## Standard Cargo projects

Crossbundle uses Cargo's public command-line interface by default and reads Cargo's JSON messages to
locate the resulting Android library. This path is engine-neutral: any application that exposes a
`cdylib` with the appropriate Android entry point can use it.

Binary source rewriting is retained only for compatibility with older integrations. Select it
explicitly with `rust_compiler = "ndk-glue"` or `rust_compiler = "quad"` under
`[package.metadata.android]`.

### Bevy

Expose the application as a library and let Bevy provide the native mobile entry point:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
bevy = { version = "0.19", default-features = false, features = ["2d"] }

[target.'cfg(target_os = "android")'.dependencies]
# NativeActivity keeps this path independent of Gradle and Maven.
bevy = { version = "0.19", default-features = false, features = ["android-native-activity"] }
```

```rust
use bevy::prelude::*;

#[bevy_main]
pub fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

The `rlib` entry keeps the library usable by a small desktop binary when desired:

```rust
fn main() {
    my_game::main();
}
```

Then use the same commands as any other Crossbow project:

```sh
crossbundle run android
crossbundle build android --release -s=native-aab
```

Crossbundle forwards the selected profile and Cargo feature flags, and streams Cargo's progress and
compiler diagnostics while building. If the package does not expose a library `cdylib`, validation
fails before compilation with the manifest change required to fix it.

`android-native-activity` is the recommended default because it keeps the toolchain Rust-native.
Projects that need AndroidX or other JVM integrations can instead choose Bevy's
`android-game-activity` feature and provide the corresponding Java/Gradle integration.
