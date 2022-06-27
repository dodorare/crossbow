use crate::permission::{error::*, types::android::*};
use jni::signature as Signature;

/// Create a java VM for executing Java calls
fn create_java_vm() -> Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}

/// Find declared permissions in AndroidManifest.xml and return it as JValue type
fn get_permission_from_manifest<'a>(
    permission: AndroidPermission,
    java_env: &jni::AttachGuard<'a>,
) -> Result<jni::objects::JValue<'a>> {
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
) -> Result<(jni::objects::JValue<'a>, jni::objects::JValue<'a>)> {
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
