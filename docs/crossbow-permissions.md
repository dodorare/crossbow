# Crossbow permissions

To request permissions with Crossbow you will need to add `crossbow` dependency to your Cargo.toml file. Then invoke request_permission function. This function checks the permission status in the application and will request permission if it's not granted yet:

```rust
use crossbow::{request_permission, android::types::*};
request_permission(AndroidPermission::Camera).unwrap();
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
