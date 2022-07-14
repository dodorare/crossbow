# Crossbow gradle

Crossbow gradle project requires installed Gradle on your PC. Check or download it [here](https://gradle.org/). Don't forget to install `GRADLE_HOME` environment variable.

To create a project go to the example you want to build and use the command below. The command belongs to macroquad engine examples:

```sh
crossbundle build android --quad --gradle

# To specify custom export gradle directory
# crossbundle build android --quad --gradle=./gen/
```

By default build directory is `target/android/<project_name>/gradle`. But you can specify your own build directory via `--gradle=<OUT_PATH>` flag. Go to the directory where Gradle project was built and use command below to manually install APK on the device.

```sh
gradle installDebug
```

Also you can replace `build` with `run` subcommand to build and run APK on your device. To see how to set android emulator check install recommendations for [linux-android](./install-linux-android.md), [macos-android](./install-macos-android.md), [windows-android](./install-windows-android.md).