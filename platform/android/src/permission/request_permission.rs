use super::AndroidPermission;
use crate::error::*;
use jni::{
    jni_sig, jni_str,
    objects::{JObject, JObjectArray, JString, JValue, JValueOwned},
    strings::JNIString,
    Env,
};
use std::mem::ManuallyDrop;

// TODO: Replace this implementation with one from Crossbow instance.

/// Find declared permissions in AndroidManifest.xml and return it as JValue type.
fn get_permission_from_manifest<'a>(
    permission: &AndroidPermission,
    env: &mut Env<'a>,
) -> Result<JValueOwned<'a>> {
    Ok(env.get_static_field(
        jni_str!("android/Manifest$permission"),
        JNIString::new(permission.to_string()),
        jni_sig!("Ljava/lang/String;"),
    )?)
}

/// Get `PERMISSION_GRANTED` and `PERMISSION_DENIED` statuses.
pub fn permission_status(env: &mut Env) -> Result<(i32, i32)> {
    let permission_denied = env.get_static_field(
        jni_str!("android/content/pm/PackageManager"),
        jni_str!("PERMISSION_DENIED"),
        jni_sig!("I"),
    )?;
    let permission_granted = env.get_static_field(
        jni_str!("android/content/pm/PackageManager"),
        jni_str!("PERMISSION_GRANTED"),
        jni_sig!("I"),
    )?;

    Ok((permission_granted.i()?, permission_denied.i()?))
}

/// Provides checking permission status in the application and will request permission if
/// it is denied.
pub fn request_permission(permission: &AndroidPermission) -> Result<bool> {
    let (_, vm) = crate::get_java_vm()?;
    vm.attach_current_thread(|env| {
        let string_permission = get_permission_from_manifest(permission, env)?.l()?;
        let (permission_granted, _permission_denied) = permission_status(env)?;
        let context = unsafe {
            ManuallyDrop::new(JObject::from_raw(
                env,
                ndk_context::android_context().context().cast(),
            ))
        };

        let ret = env.call_method(
            &*context,
            jni_str!("checkSelfPermission"),
            jni_sig!("(Ljava/lang/String;)I"),
            &[JValue::Object(&string_permission)],
        )?;

        if ret.i()? == permission_granted {
            return Ok(true);
        }

        let empty = JString::from_str(env, "")?;
        let array_permissions = JObjectArray::<JString>::new(env, 1, &empty)?;
        let string_permission = get_permission_from_manifest(permission, env)?.l()?;
        let string_permission = env.cast_local::<JString>(string_permission)?;
        array_permissions.set_element(env, 0, &string_permission)?;

        env.call_method(
            &*context,
            jni_str!("requestPermissions"),
            jni_sig!("([Ljava/lang/String;I)V"),
            &[JValue::Object(array_permissions.as_ref()), JValue::Int(0)],
        )?;
        Ok(false)
    })
}
