mod check_permission;
mod request_permission;

pub use check_permission::*;
pub use request_permission::*;

/// Create a java VM for executing Java calls
fn create_java_vm() -> crate::error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}

/// Find declared permissions in AndroidManifest.xml and return it as JValue type
fn get_permission_from_manifest<'a>(
    permission: crate::types::android::AndroidPermission,
    java_env: &jni::AttachGuard<'a>,
) -> crate::error::Result<jni::objects::JValue<'a>> {
    let class_manifest_permission = java_env.find_class("android/Manifest$permission")?;
    let field_permission = java_env.get_static_field_id(
        class_manifest_permission,
        permission.to_string(),
        "Ljava/lang/String;",
    )?;
    let string_permission = java_env
        .get_static_field_unchecked(
            class_manifest_permission,
            field_permission,
            jni::signature::JavaType::Object("java/lang/String".to_owned()),
        )?
        .to_owned();
    Ok(string_permission)
}
