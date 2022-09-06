# Android setup on Windows

## Install necessary packages

1. Use [crossbundle install command](../crossbundle/command-install.md) or download and install [Android Studio](https://developer.android.com/studio).
2. Start Android Studio, and go through the `Android Studio Setup Wizard` with the `Custom` option and install the following (or install them in `SDK Manager`):
   - Android SDK
   - NDK (Side by side)
   - Android SDK Command-line Tools
   - Android SDK Build-Tools
   - Android SDK Platform-tools

## Install necessary rustup targets

Run the following command:

```sh
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
```

## Add environment variables

From the Start search bar, enter `env` and select **Edit environment variables for your account**.

| Building strategy | Key  | Value       | Description |
| ----------------- | ---- | ----------- | ------------|
| Gradle project, native APK/AAB| ANDROID_SDK_ROOT | <path_to_sdk>\Sdk |  Can be replaced with ANDROID_SDK_PATH and ANDROID_HOME. You might not install this env var if you used [crossbundle install](../crossbundle/command-install.md) to set up required packages |
|                               |                  |                   |  or just want to build native APK or native AAB |
| Native APK/AAB        | ANDROID_NDK_ROOT | <path_to_sdk>\Sdk\ndk\<version> | Can be replaced with ANDROID_NDK_PATH and NDK_HOME. You might not install this env var if you used [crossbundle install](../crossbundle/command-install.md) to set up required packages |
| Gradle project, native AAB| JAVA_HOME | <path_to_jdk>\bin | Also, we need to make sure we have a [java runtime environment](https://www.oracle.com/java/technologies/downloads/) (JRE)   |
|                           |           |                   | or [Java developer kit](https://www.oracle.com/java/technologies/downloads/) (JDK) installed. We need jarsigner utility from there |
| Gradle project | GRADLE_HOME | <path_to_gradle> | Crossbow default build process requires installed Gradle on your PC. You can download it [here](https://services.gradle.org/distributions/) |
| Native AAB | BUNDLETOOL_PATH | <path_to_bundletool> | Download bundletool from the [`GitHub repository`](https://github.com/google/bundletool/releases) or use [crossbundle install](../crossbundle/command-install.md) |

Or you can install required env via command line accordingly table above. Arguments were provided for example

```sh
SETX ANDROID_SDK_ROOT "C:\Users\Username\AppData\Local\Android\Sdk" /M
SETX ANDROID_NDK_ROOT "C:\Users\Username\AppData\Local\Android\Sdk\ndk\23.1.7779620" /M
SETX JAVA_HOME "C:\Program Files\Java\jdk-11.0.15+10\bin" /M
SETX GRADLE_HOME "C:\Gradle\gradle-7.4.2" /M
SETX BUNDLETOOL_PATH "C:\Users\Username\bundletool-all-1.8.2.jar" /M
```

> You have to close and reopen any existing console windows for these changes to take effect.

## Set up your Android device

Follow the link to find out how to set up your device or [android emulator](./set-up-android-device.md)         

## Next step

See [hello-world](../tutorials/hello-world.md) to configure your project

After previous steps you can use crossbundle to build gradle project or native APK/AAB. Go to the links:  

- [Crossbundle build command](../crossbundle/command-build.md)
- [Crossbundle run command](../crossbundle/command-run.md)
- [Crossbundle install command](../crossbundle/command-install.md)
- [Crossbundle new command](../crossbundle/command-new.md)
