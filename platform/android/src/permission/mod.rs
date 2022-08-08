mod android_permission;
mod request_permission;

pub use android_permission::*;

use crate::error::*;
use std::sync::{
    mpsc::{sync_channel, SyncSender},
    RwLock,
};

lazy_static::lazy_static! {
    static ref PERMISSION_SENDER: RwLock<Option<SyncSender<RequestPermissionResult>>> = Default::default();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestPermissionResult {
    pub granted: bool,
    pub permission: String,
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

pub(crate) fn on_request_permission_result(permission: String, granted: bool) -> Result<()> {
    let sender = PERMISSION_SENDER.read().unwrap();
    if let Some(sender) = sender.as_ref() {
        let permission_result = RequestPermissionResult {
            granted,
            permission,
        };
        let res = sender.try_send(permission_result);
        if let Err(err) = res {
            println!(
                "Received permission result but no one is listening: {:?}",
                err
            );
        }
    } else {
        println!("Received permission result but no one is listening");
    }
    Ok(())
}
