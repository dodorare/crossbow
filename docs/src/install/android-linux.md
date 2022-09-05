# Setup Android Environment on Linux

## Install necessary packages

1. Use [crossbundle install command](../crossbundle/command-install.md) or download and install [Android Studio](https://developer.android.com/studio).
2. Start Android Studio, and go through the `Android Studio Setup Wizard` with the `Custom` option and install the following (or install them in `SDK Manager`):
   - Android SDK
   - NDK (Side by side)
   - Android SDK Command-line Tools
   - Android SDK Build-Tools
   - Android SDK Platform-tools

## Add environment variables

Take these steps to add Android-related environment variables:

- From the Start search bar, enter ‘env’ and select **Edit environment variables for your account**.
- Add `ANDROID_SDK_ROOT`||`ANDROID_SDK_PATH`||`ANDROID_HOME` and `ANDROID_NDK_ROOT`||`ANDROID_NDK_PATH`||`NDK_HOME`to environment variables.

For that edit **~/.bash_profile** or **~/.bashrc** files so they contain those lines:

```sh
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk/23.1.7779620
```

If u will build application with emulator u should add this environment variables:

```sh
export PATH=<path_to_sdk>\sdk\emulator:$PATH
export PATH=<path_to_sdk>\sdk\tools\bin:$PATH
```

Crossbow default build process requires installed Gradle on your PC. You can download it [here](https://services.gradle.org/distributions/). Set to environment variable.

```sh
export GRADLE_HOME=<path_to_gradle>
```

Also, we need to make sure we have a java runtime environment (JRE) installed. We need a key tool utility from there. <br/>
To make sure it's present type this command: `ls /usr/lib/jvm/default/bin/ | grep keytool`

_But please be aware that your path may vary._ The above path is for arch-based Linux.

If not, install JRE accordingly to your operating system:
Examples:

- Ubuntu: `sudo apt install default-jdk`
- Manjaro (Arch): `sudo pacman -S jre11-openjdk-headless jre11-openjdk jdk11-openjdk openjdk11-doc openjdk11-src`

### If you want to generate AAB (Android App Bundle) u will need to install Bundletool

If you haven't already done so, download bundletool from the [`GitHub repository`](https://github.com/google/bundletool/releases) or use [crossbundle install](../crossbundle/command-install.md).

```sh
export BUNDLETOOL_PATH=<path_to_bundletool>
```

## Install necessary rustup targets

Run the following command:

```sh
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
```

## Set up your Android device

Follow the link to find out how to set up your device or [android emulator](./android-emulator.md)         

## Next step

See [hello-world](../tutorials/hello-world.md) to configure your project

After previous steps you can use crossbundle to build gradle project or native APK/AAB. Go to the links:  

- [Crossbundle build command](../crossbundle/command-build.md)
- [Crossbundle run command](../crossbundle/command-run.md)
- [Crossbundle install command](../crossbundle/command-install.md)
- [Crossbundle new command](../crossbundle/command-new.md)


