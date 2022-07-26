use crate::{error::*, types::*};
use jni::signature::{JavaType, Primitive};

// JNI classes
const ANDROID_MANIFEST_PERMISSION: &str = "android/Manifest$permission";
const ANDROID_CONTEXT: &str = "android/content/Context";
const ANDROID_PACKAGE_MANAGER: &str = "android/content/pm/PackageManager";
const ANDROID_ACTIVITY: &str = "android/app/Activity";

// JNI methods
const REQUEST_PERMISSIONS_METHOD: &str = "requestPermissions";
const CHECK_SELF_PERMISSION_METHOD: &str = "checkSelfPermission";

// JNI signatures
const JAVA_STRING_SIGNATURE: &str = "java/lang/String";
const MANIFEST_PERMISSION_SIGNATURE: &str = "Ljava/lang/String;";
const REQUEST_PERMISSIONS_SIGNATURE: &str = "([Ljava/lang/String;I)V";
const CHECK_SELF_PERMISSION_SIGNATURE: &str = "(Ljava/lang/String;)I";
const PRIMITIVE_INT_SIGNATURE: &str = "I";

// JNI static fields
const PERMISSIONS_GRANTED: &str = "PERMISSION_GRANTED";
const PERMISSION_DENIED: &str = "PERMISSION_DENIED";

// JNI types
const ARRAY_LENGTH: i32 = 1;
const OBJECT_INDEX: i32 = 0;

/// Find declared permissions in AndroidManifest.xml and return it as JValue type
fn get_permission_from_manifest<'a>(
    permission: &AndroidPermission,
    jnienv: &jni::JNIEnv<'a>,
) -> Result<jni::objects::JValue<'a>> {
    // Find the android manifest class and get the permission
    let class_manifest_permission = jnienv.find_class(ANDROID_MANIFEST_PERMISSION)?;
    let field_permission = jnienv.get_static_field_id(
        class_manifest_permission,
        permission.to_string(),
        MANIFEST_PERMISSION_SIGNATURE,
    )?;

    // Convert the permission to the JValue type
    let string_permission = jnienv
        .get_static_field_unchecked(
            class_manifest_permission,
            field_permission,
            JavaType::Object(JAVA_STRING_SIGNATURE.to_owned()),
        )?
        .to_owned();
    Ok(string_permission)
}

/// Get `PERMISSION_GRANTED` and `PERMISSION_DENIED` status
pub fn permission_status<'a>(
    jnienv: &jni::JNIEnv<'a>,
) -> Result<(jni::objects::JValue<'a>, jni::objects::JValue<'a>)> {
    let class_package_manager = jnienv.find_class(ANDROID_PACKAGE_MANAGER)?;
    let field_permission_granted = jnienv.get_static_field_id(
        class_package_manager,
        PERMISSIONS_GRANTED,
        PRIMITIVE_INT_SIGNATURE,
    )?;

    let field_permission_denied = jnienv.get_static_field_id(
        class_package_manager,
        PERMISSION_DENIED,
        PRIMITIVE_INT_SIGNATURE,
    )?;

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
    let (_, vm) = super::create_java_vm()?;
    let jnienv = vm.attach_current_thread()?;

    let string_permission = get_permission_from_manifest(permission, &jnienv)?;

    let (_permission_granted, permission_denied) = permission_status(&jnienv)?;

    // Determine whether you have been granted a particular permission.
    let class_context = jnienv.find_class(ANDROID_CONTEXT)?;
    let method_check_self_permission = jnienv.get_method_id(
        class_context,
        CHECK_SELF_PERMISSION_METHOD,
        CHECK_SELF_PERMISSION_SIGNATURE,
    )?;

    let ret = jnienv.call_method_unchecked(
        ndk_context::android_context().context().cast(),
        method_check_self_permission,
        JavaType::Primitive(Primitive::Int),
        &[string_permission],
    )?;

    if ret.i()? == permission_denied.i()? {
        let array_permissions = jnienv.new_object_array(
            ARRAY_LENGTH,
            jnienv.find_class(JAVA_STRING_SIGNATURE)?,
            jnienv.new_string(String::new())?,
        )?;

        let string_permission = get_permission_from_manifest(permission, &jnienv)?;

        jnienv.set_object_array_element(array_permissions, OBJECT_INDEX, string_permission.l()?)?;
        let class_activity = jnienv.find_class(ANDROID_ACTIVITY)?;
        let method_request_permissions = jnienv.get_method_id(
            class_activity,
            REQUEST_PERMISSIONS_METHOD,
            REQUEST_PERMISSIONS_SIGNATURE,
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
