use super::*;
use crate::{error::*, types::android::*};
use jni::signature as Signature;

/// Provides checking permission status in the application and will request permission if it is denied.
pub fn request_permission(permission: AndroidPermission) -> Result<bool> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let string_permission = get_permission_from_manifest(permission, &java_env)?;

    let (_permission_granted, permission_denied) = permission_status(&java_env)?;

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

    if ret.i()? == permission_denied.i()? {
        let array_permissions = java_env.new_object_array(
            ARRAY_LENGTH.into(),
            java_env.find_class(JAVA_STRING_SIGNATURE)?,
            java_env.new_string(String::new())?,
        )?;

        let string_permission = get_permission_from_manifest(permission, &java_env)?;

        java_env.set_object_array_element(
            array_permissions,
            OBJECT_INDEX.into(),
            string_permission.l()?,
        )?;
        let class_activity = java_env.find_class(ANDROID_ACTIVITY)?;
        let method_request_permissions = java_env.get_method_id(
            class_activity,
            REQUEST_PERMISSIONS_METHOD,
            REQUEST_PERMISSIONS_SIGNATURE,
        )?;

        java_env.call_method_unchecked(
            ctx.context().cast(),
            method_request_permissions,
            Signature::JavaType::Primitive(Signature::Primitive::Void),
            &[array_permissions.into(), jni::objects::JValue::Int(0)],
        )?;
    }

    Ok(true)
}
