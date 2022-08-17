# Hello world example

## Generate a project

`crossbundle` uses [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) to generate a new project. This means that you need to install it before we proceed.

```sh
cargo install cargo-generate
```

Then you can create a new project:

```sh
crossbundle new project-name
# crossbundle new project-name --template bevy
# crossbundle new project-name --template quad
```

All supported templates you can watch [`here`](https://github.com/dodorare/crossbundle-templates) (each branch = template).

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

[package.metadata.android]
icon = "ic_launcher"
res = "res/android"

[package.metadata.apple]
icon = "ic_launcher"
res = "res/apple"
```

```rust
// main.rs

fn main() {
    println!("Hello, project-name!");
}
```

## Build an application

Let's build and run our first `crossbundle` application. Android commands below will generate gradle project and install apk on your device.

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
