# Hello world example

## Generate a project

Generate new project with [crossbundle new command](../crossbundle/command-new.md)!

## Project overview

The project has been created. Now let's see what the project consists of.

The code below is belong to the native crossbow project with pure rust without [android plugins](../crossbow/android-plugins.md). 
To see all possibilities of `cargo.toml` see [crossbow configutarion tutorial](../crossbow/configuration.md)

```toml
# Cargo.toml

[package]
name = "project-name"
version = "0.1.0"
authors = ["Example <example@example.com>"]
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
crossbow = "*"

[target.'cfg(target_os = "android")'.dependencies]
android-activity = { version = "0.6", features = ["native-activity"] }

[package.metadata]
app_name = "My Project"
icon = "path/to/icon.png"
```

Put the shared application code and Android entry point in `lib.rs`:
```rust
// lib.rs
pub fn main() {
    println!("Hello, project-name!");
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(_app: android_activity::AndroidApp) {
    main();
}
```

Keep the desktop binary as a one-line wrapper:
```rust
// main.rs
fn main() {
    project_name::main();
}
```

## Build an application

Let's build and run our first `crossbundle` application. Android commands below will generate gradle project and install apk on your device. See [crossbundle run command](/docs/src/crossbundle/command-run.md) for additional information.

> cd project-name. To attach a logger when application deploys on your device use `--log` flag. 
```sh
crossbundle run android --log
```

> or
```sh
crossbundle run ios --log
```

If you want to build the application for android as native AAB - add `-s=native-aab` flag or add `-s=native-apk` to build native APK.

You will see the message: `"Hello, project-name!"`
