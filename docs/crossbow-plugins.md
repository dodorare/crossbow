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

## Crossbow services

Use android services as follows:

```rust
[[package.metadata.android_service]]
name = "UpdateService"
intent_filter = []
meta_data = []
```

The code above will parse android service name and other fields into AndroidManifest.xml. Check all available service [fields](https://github.com/dodorare/android-manifest-rs/blob/main/src/service.rs).

To see how to use services check out: [services](https://developer.android.com/guide/components/services).

Known direct and indirect service subclasses [here](https://developer.android.com/reference/android/app/Service).