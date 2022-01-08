# Hello world example

## Generate a project

CrossBundle uses [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) to generate a new project. This means that you need to install it before we proceed.

```sh
cargo install cargo-generate
```

Then you can create a new project:

```sh
crossbundle new project-name
# crossbundle new project-name --template bevy
```

All supported templates you can watch [`here`](https://github.com/dodorare/crossbundle-templates) (each branch = template).

### Project overview

Done! The project has been created. Now let's see what the project consists of.

```toml
# Cargo.toml

[package]
name = "project-name"
version = "0.1.0"
authors = ["Example <example@nice.com>"]
edition = "2018"

[lib]
crate-type = ["lib", "cdylib"]

[dependencies]
crossbundle = "*"

[package.metadata]
icon = "ic_launcher"
android_res = "res/android"
apple_res = "res/apple"
```

```rust
// lib.rs

#[crossbow::crossbundle_main]
pub fn main() {
    println!("Hello, project-name!");
}
```

```rust
// main.rs

fn main() {
    project_name::main();
}
```

### Build an application

Let's build and run our first CrossBundle application.

```sh
# cd project-name
crossbundle run android
# or
crossbundle run apple
```

When the application will deploy on your device, you can attach logger

```sh
crossbundle log android
```

and you will see the message: `"Hello, project-name!"`

### Error `unable to find library -lgcc`

Please note, if you are using the `Rust 1.53.0` and `Android NDK r23-beta3` and up, an error may occur during linking.<br/>
This error will most likely be [`fixed`](https://github.com/rust-lang/rust/pull/85806) in a new version of the Rust.

```sh
error: linking with `~/Android/Sdk/ndk/23.0.7272597/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang` failed: exit status: 1
  |
  = note: ld: error: unable to find library -lgcc
          clang-12: error: linker command failed with exit code 1 (use -v to see invocation)
```

For now, the easiest way to fix it is installing an older version of `Android NDK` (ex. r22).
