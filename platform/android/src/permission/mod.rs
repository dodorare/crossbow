mod android_permission;
mod handlers;
mod request_permission;

pub use android_permission::*;
pub(crate) use handlers::*;

use std::sync::{
    mpsc::{sync_channel, SyncSender},
    RwLock,
};

lazy_static::lazy_static! {
    static ref PERMISSION_SENDER: RwLock<Option<SyncSender<RequestPermissionResult>>> = Default::default();
}

pub async fn request_permission(permission: &AndroidPermission) -> crate::error::Result<bool> {
    let receiver = {
        let mut sender_guard = PERMISSION_SENDER.write().unwrap();
        let (sender, receiver) = sync_channel(1);
        sender_guard.replace(sender);
        receiver
    };
    let res = request_permission::request_permission(permission)?;
    if res {
        Ok(true)
    } else {
        let result = receiver.recv().unwrap();
        Ok(result.granted)
    }
}
