# Crossbow permissions

To request permissions with Crossbow you will need to add `crossbow` dependency to your Cargo.toml file. Then invoke request_permission function. This function checks the permission status in the application and will request permission if it's not granted yet:

```rust
use crossbow::Permission;
let res = Permission::Camera.request_async().await?;
match res {
    Permission::Unknown => println!("Permission is in an unknown state"),
    Permission::Denied => println!("Denied by user"),
    Permission::Disabled => println!("Feature is disabled on device."),
    Permission::Granted => println!("Granted by user."),
    Permission::Restricted => println!("Restricted (only iOS)."),
}
```

Also, remember to set permissions in through `Cargo.toml` or `Info.plist`/`AndroidManifest.xml` files. List of required permissions for Cross-Platform Permission you can find in `Permission` enum.

See usage [example](https://github.com/dodorare/crossbow/blob/main/examples/macroquad-permissions/src/main.rs).

Also, it's possible to request more permissions with this API:

```
#[cfg(target_os = "android")]
crossbow::android::permission::*;
let res = request_permission(&AndroidPermission::ReadCalendar).await?;

// or this for iOS:

#[cfg(target_os = "ios")]
crossbow::ios::permission::*;
let res = request_permission(&IosPermission::CaptureDevice(MediaType::Audio)).await;
```

## Maybe useful

Useful commands to debug permission status in Android application using [adb](https://developer.android.com/studio/command-line/adb):

```sh
adb shell pm reset-permissions
adb shell pm list permission-groups
adb shell pm list permissions

adb shell pm grant <app package> <permission name>
adb shell pm revoke <app package> <permission name>
```
