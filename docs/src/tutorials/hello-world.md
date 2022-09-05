# Hello world example

## Generate a project

Generate new project with [crossbundle new command](../crossbundle/command-new.md)!

## Project overview

The project has been created. Now let's see what the project consists of.

```toml
# Cargo.toml

[package]
name = "project-name"
version = "0.1.0"
authors = ["Example <example@example.com>"]
edition = "2021"

[dependencies]
crossbow = "*"

[package.metadata]
app_name = "My Project"
icon = "path/to/icon.png"
```

```rust
// main.rs

fn main() {
    println!("Hello, project-name!");
}
```

## Build an application

Let's build and run our first `crossbundle` application. Android commands below will generate gradle project and install apk on your device. See [crossbundle run command](/docs/src/crossbundle/command-run.md) for additional information.

```sh
# cd project-name
crossbundle run android
# or
crossbundle run ios
```

If you want to build the application for android as native AAB - add `-s=native-aab` flag or add `-s=native-apk` to build native APK.

When the application deploys on your device, you can attach a logger.

```sh
crossbundle log android
```

and you will see the message: `"Hello, project-name!"`
