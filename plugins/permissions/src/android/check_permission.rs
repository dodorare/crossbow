use super::*;
use crate::{error::*, types::android::*};
use jni::signature as Signature;

/// Check whether permission was granted or not
pub fn check_permission(permission: AndroidPermission) -> Result<bool> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let string_permission = get_permission_from_manifest(permission, &java_env)?;

    let class_package_manager = java_env.find_class(ANDROID_PACKAGE_MANAGER)?;
    let field_permission_granted = java_env.get_static_field_id(
        class_package_manager,
        PERMISSIONS_GRANTED,
        PRIMITIVE_INT_SIGNATURE,
    )?;

    let field_permission_denied = java_env.get_static_field_id(
        class_package_manager,
        "PERMISSION_DENIED",
        PRIMITIVE_INT_SIGNATURE,
    )?;

    let int_permission_denied = java_env.get_static_field_unchecked(
        class_package_manager,
        field_permission_denied,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
    )?;

    let int_permission_granted = java_env.get_static_field_unchecked(
        class_package_manager,
        field_permission_granted,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
    )?;

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

    if ret.i()? == int_permission_granted.i()? {
        return Ok(true);
    }

    Ok(true)
}
