mod check_permission;
mod request_permission;
mod show_text;

pub use check_permission::*;
pub use request_permission::*;
pub use show_text::*;

use crate::types::android::*;
use jni::signature as Signature;

/// Create a java VM for executing Java calls
fn create_java_vm() -> crate::error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}

/// Find declared permissions in AndroidManifest.xml and return it as JValue type
fn get_permission_from_manifest<'a>(
    permission: AndroidPermission,
    java_env: &jni::AttachGuard<'a>,
) -> crate::error::Result<jni::objects::JValue<'a>> {
    // Find the android manifest class and get the permission
    let class_manifest_permission = java_env.find_class(ANDROID_MANIFEST_PERMISSION)?;
    let field_permission = java_env.get_static_field_id(
        class_manifest_permission,
        permission.to_string(),
        MANIFEST_PERMISSION_SIGNATURE,
    )?;

    // Convert the permission to the JValue type
    let string_permission = java_env
        .get_static_field_unchecked(
            class_manifest_permission,
            field_permission,
            Signature::JavaType::Object(JAVA_STRING_SIGNATURE.to_owned()),
        )?
        .to_owned();
    Ok(string_permission)
}

/// Get `PERMISSION_GRANTED` and `PERMISSION_DENIED` status
pub fn permission_status<'a>(
    java_env: &jni::AttachGuard<'a>,
) -> crate::error::Result<(jni::objects::JValue<'a>, jni::objects::JValue<'a>)> {
    let class_package_manager = java_env.find_class(ANDROID_PACKAGE_MANAGER)?;
    let field_permission_granted = java_env.get_static_field_id(
        class_package_manager,
        PERMISSIONS_GRANTED,
        PRIMITIVE_INT_SIGNATURE,
    )?;

    let field_permission_denied = java_env.get_static_field_id(
        class_package_manager,
        PERMISSION_DENIED,
        PRIMITIVE_INT_SIGNATURE,
    )?;

    let permission_denied = java_env.get_static_field_unchecked(
        class_package_manager,
        field_permission_denied,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
    )?;

    let permission_granted = java_env.get_static_field_unchecked(
        class_package_manager,
        field_permission_granted,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
    )?;

    Ok((permission_granted, permission_denied))
}
