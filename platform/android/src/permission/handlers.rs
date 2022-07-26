use super::AndroidPermission;
use crate::error::*;
use async_channel::Sender;
use jni::{objects::JString, sys::jboolean, JNIEnv};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref PERMISSION_SENDER: Mutex<Option<Sender<RequestPermissionResult>>> = Default::default();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestPermissionResult {
    pub granted: bool,
    pub permission: AndroidPermission,
}

pub(crate) fn on_request_permission_result(
    _env: JNIEnv,
    _permission: JString,
    _result: jboolean,
) -> Result<()> {
    Ok(())
}
