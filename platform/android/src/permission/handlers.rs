use crate::error::*;
use crate::utils::jstring_to_string;
use jni::sys::JNI_TRUE;
use jni::{objects::JString, sys::jboolean, JNIEnv};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestPermissionResult {
    pub granted: bool,
    pub permission: String,
}

pub(crate) fn on_request_permission_result(
    env: JNIEnv,
    permission: JString,
    result: jboolean,
) -> Result<()> {
    let sender = super::PERMISSION_SENDER.read().unwrap();
    if let Some(sender) = sender.as_ref() {
        let permission_result = RequestPermissionResult {
            granted: result == JNI_TRUE,
            permission: jstring_to_string(&env, permission)?,
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
