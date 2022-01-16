# Android setup on Docker

## Build Android with Docker

Pre-requirements:

- Cloned `crossbow` repository
- Installed [Docker](https://docs.docker.com/get-docker/)
- Installed [adb](https://developer.android.com/studio/command-line/adb) (optional)

To run an example Android application with `docker` you will need to following steps:

Download the Docker image:

```sh
docker pull ghcr.io/dodorare/crossbundle:latest
```

Run the following command at the root of this project:

```sh
docker run --rm -it -v "$(pwd)/:/src" -w /src/examples/macroquad-3d ghcr.io/dodorare/crossbundle build android --quad --release
```

Run the following command in powershell if you use Windows:

```sh
docker run --rm -it -v "${pwd}/:/src" -w /src/examples/macroquad-3d ghcr.io/dodorare/crossbundle build android --quad --release
```

Install APK on connected Android phone via USB:

```sh
adb install ./target/android/release/Macroquad 3D.apk
```

Or transfer APK file `./target/android/release/Macroquad 3D.apk` to your phone with any messenger or any file-hosting application.

## Known issues

### Mac M1

There's a bug in docker image that causes the container to crash when running `cargo build` command with one of rust-bindgen dependency. We have an [tracking issue](https://github.com/dodorare/crossbow/issues/58) for this.
