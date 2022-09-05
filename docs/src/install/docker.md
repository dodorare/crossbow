# Android setup on Docker

## Build Android with Docker

Pre-requirements:
- Installed [Docker](https://docs.docker.com/get-docker/)
- Installed [adb](https://developer.android.com/studio/command-line/adb) (optional)

Clone this repository with this command:

```sh
git clone https://github.com/dodorare/crossbow
cd ./crossbow/
```

To run an example Android application with `docker` you will need to following steps:

Download the Docker image:

```sh
docker pull ghcr.io/dodorare/crossbundle:latest
```

Run the following command at the root of `crossbow` project:

> For unix systems (Bash):
```sh
docker run --rm -it -v "$(pwd)/:/src" -w /src/examples/macroquad-permissions ghcr.io/dodorare/crossbundle build android --release
```

> For Windows (PowerShell):
```sh
docker run --rm -it -v "${pwd}/:/src" -w /src/examples/macroquad-permissions ghcr.io/dodorare/crossbundle build android --release
```

Install APK on connected Android phone via USB:

Follow the link to find out how to set up your device or [android emulator](./android-emulator.md)      
   
```sh
adb install ./target/android/macroquad-permissions/gradle/build/outputs/apk/release/gradle-release-unsigned.apk
```

Or transfer APK file to your phone with any application.
