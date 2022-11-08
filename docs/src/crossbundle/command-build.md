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

## Crossbundle build native AAB/APK with `--bevy-compile` flag for bevy projects

Alternative build opportunity avoid using cargo Executor trait. For correct build and app working process need to provide several steps: 

1. Add lib section to `Cargo.toml` with cdylib crate type:

```toml
[lib]
crate-type = ["cdylib"]
path = "src/main.rs"
```

Note: Our examples using only main.rs as bin and as lib file so we need to set path to lib. You can create lib.rs file and set only crate-type  

2. Add crossbundle derive crate to `Cargo.toml` to dependencies section: 

```toml
[dependencies]
crossbundle-derive = { path = "./../../crossbundle/derive", version = "0.2.3" }
```

3. Put crossbundle derive macro above main function to init Android Native Activity.

```rust
#[crossbundle_derive::crossbundle_main]
fn main() {
   // code
}
```

Use command below to build apk or aab: 

```sh
crossbundle build android -s=native-apk --bevy-compile
# or do you need AAB:
crossbundle build android -s=native-aab --bevy-compile
```