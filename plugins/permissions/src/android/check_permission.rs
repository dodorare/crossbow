use super::*;
use crate::{error::*, types::android::*};
use jni::signature as Signature;

/// Check whether permission was granted or not
pub fn check_permission(permission: AndroidPermission) -> Result<bool> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let string_permission = get_permission_from_manifest(permission, &java_env)?;

    let (permission_granted, _permission_denied) = permission_status(&java_env)?;

    // Determine whether you have been granted a particular permission.
    let class_context = java_env.find_class(ANDROID_CONTEXT)?;
    let method_check_self_permission = java_env.get_method_id(
        class_context,
        CHECK_SELF_PERMISSION_METHOD,
        CHECK_SELF_PERMISSION_SIGNATURE,
    )?;

    let ret = java_env.call_method_unchecked(
        ctx.context().cast(),
        method_check_self_permission,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
        &[string_permission],
    )?;

    if ret.i()? == permission_granted.i()? {
        return Ok(true);
    }

    Ok(true)
}
