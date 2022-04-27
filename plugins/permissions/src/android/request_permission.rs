use crate::{error::*, types::android::*};

/// Request permission
pub fn request_permission(permission: AndroidPermission) -> Result<bool> {
    if super::check_permission(permission)? {
        return Ok(true);
    }

    let (ctx, vm) = super::create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let array_permissions = java_env.new_object_array(
        1,
        java_env.find_class("java/lang/String")?,
        java_env.new_string("")?,
    )?;

    let string_permission = super::get_permission_from_manifest(permission, &java_env)?;

    java_env.set_object_array_element(array_permissions, 0, string_permission.l()?)?;
    let class_activity = java_env.find_class("android/app/Activity")?;
    let method_request_permissions = java_env.get_method_id(
        class_activity,
        "requestPermissions",
        "([Ljava/lang/String;I)V",
    )?;

    java_env.call_method_unchecked(
        ctx.context().cast(),
        method_request_permissions,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
        &[array_permissions.into(), jni::objects::JValue::Int(0)],
    )?;
    // /* TODO: How to create a native callback for a Java class for last argument (0) */
    // env->CallVoidMethod(mApp->activity->clazz, MethodrequestPermissions, ArrayPermissions, 0);

    Ok(true)
}
