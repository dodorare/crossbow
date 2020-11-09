# Creator

This file will cover some of the basics of Android and iOS development with `creator-rs`.

## Android development

## Build with Docker

Pre-requirements:

* Cloned `creator-rs` repository
* Docker

To run example Android application with `docker` you will need to run following command in root of this project:

```sh
docker run --rm -it -v "$(pwd)/:/src" -w /src/examples/app docker.pkg.github.com/creator-rs/creator/android cargo creator build
```

Install APK on connected Android phone via USB:

```sh
adb install ./target/debug/apk/Creator.apk
```

Or transfer APK file `./target/debug/apk/Creator.apk` to your phone with messanger or any file-hosting application.

## Build with installed Android NDK

Install `cargo-creator` tool:

```sh
cargo install cargo-creator
```

Run this command in `examples/app` folder of this project:

```sh
cargo creator build
```

## Run it on Android smartphone

To watch logs of the Android application run this:

```sh
adb logcat RustStdoutStderr:D '*:S'
```
