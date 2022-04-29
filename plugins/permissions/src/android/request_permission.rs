use super::*;
use crate::{error::*, types::android::*};
use jni::signature as Signature;

/// Request permission
pub fn request_permission(permission: AndroidPermission) -> Result<bool> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    if check_permission(permission)? {
        return Ok(true);
        // TODO: Show UI text to notify a user about permission status
    }

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
    // /* TODO: How to create a native callback for a Java class for last argument (0) */
    // env->CallVoidMethod(mApp->activity->clazz, MethodrequestPermissions, ArrayPermissions, 0);

    Ok(true)
}
