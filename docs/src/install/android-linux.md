# Setup Android Environment on Linux

## Install necessary packages

1. Use [crossbundle install command](https://github.com/dodorare/crossbow/blob/main/docs/crossbundle-install-command.md) or download and install [Android Studio](https://developer.android.com/studio).
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

If you haven't already done so, download bundletool from the [`GitHub repository`](https://github.com/google/bundletool/releases).

```sh
export BUNDLETOOL_PATH=<path_to_bundletool>
```

## Set up your Android device

To prepare to run your `Crossbow` app on an Android device, you need an Android device running Android 4.1 (API level 16) or higher.

1. Enable **Developer options** and **USB debugging** on your device. Detailed instructions are available in the [Android documentation](https://developer.android.com/studio/debug/dev-options).
2. Using a USB cable, plug your phone into your computer. If prompted on your device, authorize your computer to access your device.

## Set up the Android emulator

To prepare to run and test your Crossbow app on the Android emulator, follow these steps if you want to install it from the console:

```sh
# Run following command to install System Image for Android SDK 31
crossbundle install sdkmanager --install "system-images;android-31;google_apis;x86_64"
# Run this command to create a new emulator
avdmanager create avd -n Phone -k "system-images;android-31;google_apis;x86_64"
# And finally run this command to start the emulator
emulator -avd=Phone
```

If you want to install it from the GUI, follow these instructions:

1. Enable [`VM acceleration`](https://developer.android.com/studio/run/emulator-acceleration) on your machine.
2. Launch **Android Studio**, click the **AVD Manager** icon, and select **Create Virtual Device**.
3. Choose a device definition and select **Next**.
4. Select one or more system images for the Android versions you want to emulate, and select **Next**. An x86 or x86_64 image is recommended.
5. Under Emulated Performance, select **Hardware - GLES 2.0** to enable [`hardware acceleration`](https://developer.android.com/studio/run/emulator-acceleration).
6. Verify the AVD configuration is correct, and select **Finish**. (For details on the above steps, see [`Managing AVDs`](https://developer.android.com/studio/run/managing-avds))
7. In Android Virtual Device Manager, click Run in the toolbar. The emulator starts up and displays the default canvas for your selected OS version and device.

## Install necessary rustup targets

Run the following command:

```sh
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
```

## Next step

[`Hello World! application`](https://github.com/dodorare/crossbow/wiki/Hello-World!) with Crossbow
