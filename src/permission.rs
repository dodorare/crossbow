use crate::error::*;
#[cfg(all(target_os = "android", feature = "android"))]
use crossbow_android::{permission, types::AndroidPermission};
#[cfg(all(target_os = "ios", feature = "ios"))]
use crossbow_ios::{permission, types::IosPermission};

pub enum Permission {
    #[cfg(all(target_os = "android", feature = "android"))]
    Android(AndroidPermission),
    #[cfg(all(target_os = "ios", feature = "ios"))]
    Ios(IosPermission),
}

pub fn request_permission<T: Into<Permission>>(permission: T) -> Result<bool> {
    #[allow(unreachable_code)]
    match permission.into() {
        #[cfg(all(target_os = "android", feature = "android"))]
        Permission::Android(x) => Ok(permission::request_permission(&x)?),
        #[cfg(all(target_os = "ios", feature = "ios"))]
        Permission::Ios(x) => Ok(permission::request_permission(&x)?),
    }
}

#[cfg(all(target_os = "android", feature = "android"))]
impl From<AndroidPermission> for Permission {
    fn from(x: AndroidPermission) -> Self {
        Permission::Android(x)
    }
}

#[cfg(all(target_os = "ios", feature = "ios"))]
impl From<IosPermission> for Permission {
    fn from(x: IosPermission) -> Self {
        Permission::Ios(x)
    }
}
