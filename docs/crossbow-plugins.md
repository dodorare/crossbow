# Crossbow plugins
## Crossbow permissions

You can import `crossbow permissions` features as follows:

```rust
use crossbow::crossbow_permissions::prelude::*;
```

Invoke request_permission function. This function provides checking permission status in the application and will request permission if it is denied.

```rust
request_permission(permission: AndroidPermission)
```

See usage [example](https://github.com/dodorare/crossbow/blob/main/examples/macroquad-permissions/src/main.rs).

Useful commands to debug permission status in the application using [adb](https://developer.android.com/studio/command-line/adb).

```sh
adb shell pm grant <app package> <permission name>
adb shell pm revoke <app package> <permission name>
```
```sh
adb shell pm reset-permissions
adb shell pm list permission-groups
adb shell pm list permissions
```

## Crossbow gradle

Crossbow gradle project requires installed Gradle on your PC. Check or download it [here](https://gradle.org/).

To create a project go to the example you want to build and use the command below. The command belongs to macroquad engine examples: 

```rust
crossbundle build android --quad --gradle
```

By default build directory is `target/android/<project_name>/gradle`. You can assign your own build directory via `--export_path` flag. Go to the directory where Gradle project was built and use 

```sh 
gradle installDebug
``` 
to manually install APK on the device.

Also you can replace `build` with `run` subcommand to build and run APK on your device. To see how to set android emulator check install recommendations for [linux-android](./install-linux-android.md), 
[macos-android](./install-macos-android.md), [windows-android](./install-windows-android.md). 