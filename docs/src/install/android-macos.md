# Android setup on MacOS

## Install necessary packages

1. Use [crossbundle install command](../crossbundle/command-install.md) or download and install [Android Studio](https://developer.android.com/studio).
2. Start Android Studio, and go through the `Android Studio Setup Wizard` with the `Custom` option and install the following (or install them in `SDK Manager`):
   - Android SDK
   - NDK (Side by side)
   - Android SDK Command-line Tools
   - Android SDK Build-Tools
   - Android SDK Platform-tools
3. Download and install java. Can be installed with `brew` through:

```sh
brew tap adoptopenjdk/openjdk
brew install --cask adoptopenjdk8
```

## Install necessary rustup targets

Run the following command:

```sh
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
```

## Add environment variables

From the Start search bar, enter ‘env’ and select **Edit environment variables for your account**.

| Building strategy | Key  | Value       | Description |
| ----------------- | ---- | ----------- | ------------|
| Gradle project, native APK/AAB| ANDROID_SDK_ROOT | <path_to_sdk>\Sdk |  Can be replaced with ANDROID_SDK_PATH and ANDROID_HOME. You might not install this env var if you used [crossbundle install](../crossbundle/command-install.md) to set up required packages |
|                               |                  |                   |  or just want to build native APK or native AAB |
| Native APK/AAB        | ANDROID_NDK_ROOT | <path_to_sdk>\Sdk\ndk\<version> | Can be replaced with ANDROID_NDK_PATH and NDK_HOME. You might not install this env var if you used [crossbundle install](../crossbundle/command-install.md) to set up required packages |
| Gradle project | GRADLE_HOME | <path_to_gradle> | Crossbow default build process requires installed Gradle on your PC. You can download it [here](https://services.gradle.org/distributions/) |
| Native AAB | BUNDLETOOL_PATH | <path_to_bundletool> | Download bundletool from the [`GitHub repository`](https://github.com/google/bundletool/releases) or use [crossbundle install](../crossbundle/command-install.md) |

For that edit **~/.bash_profile**/**~/.bashrc** or **~/.zshrc** files so they contain those lines:

```sh
export ANDROID_SDK_ROOT=$HOME/android/sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/23.1.7779620
export GRADLE_HOME=<path_to_gradle>
export BUNDLETOOL_PATH=<path_to_bundletool>
```

Also, we need to make sure we have a java runtime environment (JRE) installed. We will need a key tool utility from there. <br/>
To make sure it's present type this command: `ls /usr/lib/jvm/default/bin/ | grep keytool` or add to your `PATH` env var.

## Set up your Android device

Follow the link to find out how to set up your device or [android emulator](./set-up-android-device.md)         

## Next step

See [hello-world](../tutorials/hello-world.md) to configure your project

After previous steps you can use crossbundle to build gradle project or native APK/AAB. Go to the links:  

- [Crossbundle build command](../crossbundle/command-build.md)
- [Crossbundle run command](../crossbundle/command-run.md)
- [Crossbundle install command](../crossbundle/command-install.md)
- [Crossbundle new command](../crossbundle/command-new.md)


