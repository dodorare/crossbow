# Android setup on MacOS

## Install Android Studio

1. Download and install [Android Studio](https://developer.android.com/studio).
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

Please note, `Android NDK r23-beta3` and up [`do not include libgcc anymore`](https://github.com/android/ndk/wiki/Changelog-r23#changes).
Since Rust 1.53.0 still needs `libgcc` to support `Android NDK r23`, an error may occur during linking. This error will most likely be [`fixed`](https://github.com/rust-lang/rust/pull/85806) in a new version of the Rust.

```sh
error: linking with `~/Android/Sdk/ndk/23.0.7272597/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang`
failed: exit status: 1
  |
  = note: ld: error: unable to find library -lgcc
          clang-12: error: linker command failed with exit code 1 (use -v to see invocation)
```

For now, the easiest way to fix it is by installing an older version of `Android NDK` (ex. r22).

### Add environment variables

We need to make sure that Android-related environment variables are set in `PATH`, `ANDROID_SDK_ROOT`, and `ANDROID_NDK_ROOT`.

For that edit **~/.bash_profile**/**~/.bashrc** or **~/.zshrc** files so they contain those lines:

```sh
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/22.0.7026061
export PATH=$PATH:$ANDROID_SDK_ROOT/emulator
export PATH=$PATH:$ANDROID_SDK_ROOT/tools
export PATH=$PATH:$ANDROID_SDK_ROOT/tools/bin
export PATH=$PATH:$ANDROID_SDK_ROOT/platform-tools
```

Also, we need to make sure we have a java runtime environment (JRE) installed. We will need a key tool utility from there. <br/>
To make sure it's present type this command: `ls /usr/lib/jvm/default/bin/ | grep keytool` or add to your `PATH` env var.

### Set up your Android device

To prepare to run your `Crossbow` app on an Android device, you need an Android device running Android 4.1 (API level 16) or higher.

1. Enable **Developer options** and **USB debugging** on your device. Detailed instructions are available in the [Android documentation](https://developer.android.com/studio/debug/dev-options).
2. Using a USB cable, plug your phone into your computer. If prompted on your device, authorize your computer to access your device.

### Set up the Android emulator

To prepare to run and test your Flutter app on the Android emulator, follow these steps:

1. Enable [`VM acceleration`](https://developer.android.com/studio/run/emulator-acceleration) on your machine.
2. Launch **Android Studio**, click the **AVD Manager** icon, and select **Create Virtual Device**.
3. Choose a device definition and select **Next**.
4. Select one or more system images for the Android versions you want to emulate, and select **Next**. An x86 or x86_64 image is recommended.
5. Under Emulated Performance, select **Hardware - GLES 2.0** to enable [`hardware acceleration`](https://developer.android.com/studio/run/emulator-acceleration).
6. Verify the AVD configuration is correct, and select **Finish**. (For details on the above steps, see [`Managing AVDs`](https://developer.android.com/studio/run/managing-avds))
7. In Android Virtual Device Manager, click Run in the toolbar. The emulator starts up and displays the default canvas for your selected OS version and device.

### Install necessary rustup targets

Run the following command:

```sh
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
```

### Next step

[`Hello World! application`](https://github.com/dodorare/crossbow/wiki/Hello-World!) with Crossbow
