use crate::{error::*, types::android::*};

/// Check whether permission was granted or not
pub fn check_permission(permission: AndroidPermission) -> Result<bool> {
    let (ctx, vm) = super::create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let string_permission = super::get_permission_from_manifest(permission, &java_env)?;

    let class_package_manager = java_env.find_class("android/content/pm/PackageManager")?;
    let field_permission_granted =
        java_env.get_static_field_id(class_package_manager, "PERMISSION_GRANTED", "I")?;
    let int_permission_granted = java_env.get_static_field_unchecked(
        class_package_manager,
        field_permission_granted,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Int),
    )?;

    // Determine whether you have been granted a particular permission.
    let class_context = java_env.find_class("android/content/Context")?;
    let method_check_self_permission = java_env.get_method_id(
        class_context,
        "checkSelfPermission",
        "(Ljava/lang/String;)I",
    )?;
    let ret = java_env.call_method_unchecked(
        ctx.context().cast(),
        method_check_self_permission,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Int),
        &[string_permission],
    )?;
    Ok(ret.i()? == int_permission_granted.i()?)
}
