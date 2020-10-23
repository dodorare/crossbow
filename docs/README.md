# Creator

This file will cover some of the basics of Android and iOS development with `creator-rs`.

## Android development

## Build with Docker

Pre-requirements:

* Cloned `creator-rs` repository
* Docker

To run example Android application with `docker` you will need to run following command in `examples/app` folder of this project:

```sh
docker run --rm -v "$(pwd)/../../:/src" docker.pkg.github.com/creator-rs/creator/android cargo apk build
```

## Build with Android NDK and ShaderC

Run this command in root of this project:

```sh
SHADERC_LIB_DIR=~/Desktop/shaderc cargo apk build --manifest-path=examples/app/Cargo.toml
```

## Run it on Android smartphone

To watch logs of the Android application run this:

```sh
adb logcat RustStdoutStderr:D '*:S'
```
