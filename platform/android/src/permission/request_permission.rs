use super::AndroidPermission;
use crate::error::*;
use jni::signature::{JavaType, Primitive};

// TODO: Replace this implementation with one from Crossbow instance.

/// Find declared permissions in AndroidManifest.xml and return it as JValue type
fn get_permission_from_manifest<'a>(
    permission: &AndroidPermission,
    jnienv: &jni::JNIEnv<'a>,
) -> Result<jni::objects::JValue<'a>> {
    // Find the android manifest class and get the permission
    let class_manifest_permission = jnienv.find_class("android/Manifest$permission")?;
    let field_permission = jnienv.get_static_field_id(
        class_manifest_permission,
        permission.to_string(),
        "Ljava/lang/String;",
    )?;

    // Convert the permission to the JValue type
    let string_permission = jnienv
        .get_static_field_unchecked(
            class_manifest_permission,
            field_permission,
            JavaType::Object("java/lang/String".to_owned()),
        )?
        .to_owned();
    Ok(string_permission)
}

/// Get `PERMISSION_GRANTED` and `PERMISSION_DENIED` status
pub fn permission_status<'a>(
    jnienv: &jni::JNIEnv<'a>,
) -> Result<(jni::objects::JValue<'a>, jni::objects::JValue<'a>)> {
    let class_package_manager = jnienv.find_class("android/content/pm/PackageManager")?;
    let field_permission_granted =
        jnienv.get_static_field_id(class_package_manager, "PERMISSION_GRANTED", "I")?;

    let field_permission_denied =
        jnienv.get_static_field_id(class_package_manager, "PERMISSION_DENIED", "I")?;

    let permission_denied = jnienv.get_static_field_unchecked(
        class_package_manager,
        field_permission_denied,
        JavaType::Primitive(Primitive::Int),
    )?;

    let permission_granted = jnienv.get_static_field_unchecked(
        class_package_manager,
        field_permission_granted,
        JavaType::Primitive(Primitive::Int),
    )?;

    Ok((permission_granted, permission_denied))
}

/// Provides checking permission status in the application and will request permission if it is denied.
pub fn request_permission(permission: &AndroidPermission) -> Result<()> {
    let (_, vm) = crate::create_java_vm()?;
    let jnienv = vm.attach_current_thread()?;

    let string_permission = get_permission_from_manifest(permission, &jnienv)?;

    let (_permission_granted, permission_denied) = permission_status(&jnienv)?;

    // Determine whether you have been granted a particular permission.
    let class_context = jnienv.find_class("android/content/Context")?;
    let method_check_self_permission = jnienv.get_method_id(
        class_context,
        "checkSelfPermission",
        "(Ljava/lang/String;)I",
    )?;

    let ret = jnienv.call_method_unchecked(
        ndk_context::android_context().context().cast(),
        method_check_self_permission,
        JavaType::Primitive(Primitive::Int),
        &[string_permission],
    )?;

    if ret.i()? == permission_denied.i()? {
        let array_permissions = jnienv.new_object_array(
            1,
            jnienv.find_class("java/lang/String")?,
            jnienv.new_string(String::new())?,
        )?;

        let string_permission = get_permission_from_manifest(permission, &jnienv)?;

        jnienv.set_object_array_element(array_permissions, 0, string_permission.l()?)?;
        let class_activity = jnienv.find_class("android/app/Activity")?;
        let method_request_permissions = jnienv.get_method_id(
            class_activity,
            "requestPermissions",
            "([Ljava/lang/String;I)V",
        )?;

        jnienv.call_method_unchecked(
            ndk_context::android_context().context().cast(),
            method_request_permissions,
            JavaType::Primitive(Primitive::Void),
            &[array_permissions.into(), jni::objects::JValue::Int(0)],
        )?;
    }

    Ok(())
}
