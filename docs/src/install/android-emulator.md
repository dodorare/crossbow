# Set up your Android device

To prepare to run your `Crossbow` app on an Android device, you need an Android device running Android 4.1 (API level 16) or higher.

1. Enable **Developer options** and **USB debugging** on your device. Detailed instructions are available in the [Android documentation](https://developer.android.com/studio/debug/dev-options).
2. Using a USB cable, plug your phone into your computer. If prompted on your device, authorize your computer to access your device.

# Set up the Android emulator

To prepare to run and test your Crossbow app on the Android emulator, follow these steps if you want to install it from the console:

```sh
# Run following command to install System Image for Android SDK 31
crossbundle install sdkmanager --install "system-images;android-31;google_apis;x86_64"
# Run this command to create a new Pnone emulator
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
