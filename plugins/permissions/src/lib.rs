#[cfg(target_os = "android")]
pub mod android;
pub mod error;
pub mod types;

pub enum Permission {
    AndroidPermission(types::android::AndroidPermission),
    ApplePermission,
}

pub fn request_permission(permission: Permission) -> error::Result<bool> {
    match permission {
        Permission::AndroidPermission(_p) => {
            #[cfg(target_os = "android")]
            return android::request_permission(_p);

            #[cfg(not(target_os = "android"))]
            Err(error::PermissionError::PermissionWrongPlatform)
        }
        Permission::ApplePermission => Ok(false),
    }
}

pub fn check_permission(permission: Permission) -> error::Result<bool> {
    match permission {
        Permission::AndroidPermission(_p) => {
            #[cfg(target_os = "android")]
            return android::check_permission(_p);

            #[cfg(not(target_os = "android"))]
            Err(error::PermissionError::PermissionWrongPlatform)
        }
        Permission::ApplePermission => Ok(false),
    }
}

pub mod prelude {
    #[cfg(target_os = "android")]
    pub use super::android::*;
    pub use super::types::android::*;

    pub use super::{check_permission, request_permission, Permission};
}
