# Android setup on Windows

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
- Add `ANDROID_SDK_ROOT` variable with value `<path_to_sdk>\sdk`.<br/>(ex. `C:\Users\username\AppData\Local\android\sdk`)
- Add `ANDROID_NDK_ROOT` variable with value `<path_to_sdk>\sdk\ndk\<version>`.<br/>(ex. `C:\Users\username\AppData\Local\Android\Sdk\ndk\23.1.7779620`)

Or you can install via command line

```sh
SETX ANDROID_SDK_ROOT "path_to_sdk\sdk" /M
SETX ANDROID_NDK_ROOT "path_to_sdk\sdk\ndk\version" /M
```

Crossbow default build process requires installed Gradle on your PC. You can download it [here](https://services.gradle.org/distributions/). Set to environment variable.

- Add `GRADLE_HOME` variable with value `<path_to_gradle>`.

```sh
SETX GRADLE_HOME "path_to_gradle" /M
```

If you will build application with emulator you should add this to environment variables:

- Add `<path_to_sdk>\sdk\tools\bin` to `PATH` variable.
- Add `<path_to_sdk>\sdk\emulator` to `PATH` variable.

```sh
SETX "<path_to_sdk>\sdk\tools\bin" ~PATH~
SETX "<path_to_sdk>\sdk\emulator" ~PATH~
``` 

Also, we need to make sure we have a [java runtime environment](https://www.oracle.com/java/technologies/downloads/) (JRE) or [Java developer kit](https://www.oracle.com/java/technologies/downloads/) (JDK) installed. We need a key tool utility from there. <br/>
To make sure it's present type this command: `keytool -h`

- If command above fails, add `<path_to_jre>\bin` to `PATH` environment variable.<br/>(ex. `C:\Program Files\Android\Android Studio\jre\bin`) <br/>(ex. `C:\Program Files\java\jdk\bin`)

```sh
SETX JAVA_HOME "path_to_jdk" /M
```

You have to close and reopen any existing console windows for these changes to take effect.

### If you want to generate AAB (Android App Bundle) u will need to install Bundletool

If you haven't already done so, download bundletool from the [`GitHub repository`](https://github.com/google/bundletool/releases) or use [crossbundle install](../crossbundle/command-install.md).

- Add `BUNDLETOOL_PATH` variable with value `<path_to_bundletool>`.

```sh
SETX BUNDLETOOL_PATH "path_to_bundletool" /M
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
