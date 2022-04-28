mod check_permission;
mod request_permission;

pub use check_permission::*;
pub use request_permission::*;

const ANDROID_PACKAGE_MANAGER: &'static str = "android/content/pm/PackageManager";
const ANDROID_ACTIVITY: &'static str = "android/app/Activity";
const JAVA_STRING: &'static str = "java/lang/String";
const REQUEST_PERMISSIONS_METHOD: &'static str = "requestPermissions";
const CHECK_SELF_PERMISSION_METHOD: &'static str = "checkSelfPermission";
const REQUEST_PERMISSIONS_SIGNATURE: &'static str = "([Ljava/lang/String;I)V";
const CHECK_SELF_PERMISSION_SIGNATURE: &'static str = "(Ljava/lang/String;)I";
const ARRAY_LENGTH: i32 = 1;
const OBJECT_INDEX: i32 = 0;
const PRIMITIVE_INT: &'static str = "I";
const PERMISSIONS_GRANTED: &'static str = "PERMISSION_GRANTED";
const ANDROID_CONTEXT: &'static str = "android/content/Context";

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
