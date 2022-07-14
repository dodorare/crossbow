use crossbow_android::{error::Result, permission, types::AndroidPermission};

pub enum Permission {
    Android(AndroidPermission),
    Apple,
}

pub fn request_permission<T: Into<Permission>>(permission: T) -> Result<bool> {
    match permission.into() {
        Permission::Android(x) => permission::request_permission(&x),
        Permission::Apple => {
            println!("iOS permissions not supported yet");
            Ok(false)
        }
    }
}

impl From<AndroidPermission> for Permission {
    fn from(x: AndroidPermission) -> Self {
        Permission::Android(x)
    }
}
