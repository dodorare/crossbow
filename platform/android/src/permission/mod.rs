mod android_permission;
mod handlers;
mod request_permission;
mod status;

pub use android_permission::*;
pub(crate) use handlers::*;
pub use status::*;

pub async fn request_permission(permission: &AndroidPermission) -> crate::error::Result<bool> {
    request_permission::request_permission(permission)?;
    // TODO: Check if permission granted.
    Ok(false)
}
