use super::*;
use crate::{error::*, types::android::*};
use jni::signature as Signature;
use jni::sys::_jobject;

pub fn show_text(permission: AndroidPermission) -> crate::error::Result<()> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    // makeText() 1st arg
    // let context_class = java_env.find_class("android/content/Context")?;
    // let method_request_permissions = java_env.get_method_id(
    //     class_activity,
    //     REQUEST_PERMISSIONS_METHOD,
    //     REQUEST_PERMISSIONS_SIGNATURE,
    // )?;
    let array = java_env.new_object_array(
        ARRAY_LENGTH,
        java_env.find_class(JAVA_STRING_SIGNATURE)?,
        java_env.new_string(String::new())?,
    )?;
    let request_permission = invoke_request_permission_method(permission, &java_env)?;
    let class = java_env.find_class(ANDROID_ACTIVITY)?;
    let get_on_request_permissions_result = java_env.get_method_id(
        class,
        "onRequestPermissionsResult",
        "(I[Ljava/lang/String;[I)V",
    )?;
    let object_on_request_permissions_result = java_env
        .call_method_unchecked(
            ctx.context().cast(),
            get_on_request_permissions_result,
            Signature::JavaType::Primitive(Signature::Primitive::Void),
            &[
                request_permission.into(),
                array.into(),
                jni::objects::JValue::Int(1),
            ],
        )?
        .l()?;

    let context_class = java_env.find_class(ANDROID_CONTEXT)?;

    // Get activity as argument for Toast.makeText() method
    let get_application_activity =
        java_env.get_method_id(context_class, "getApplicationContext", "()V")?;
    let application_activity = java_env.call_method_unchecked(
        ctx.context().cast(),
        get_application_activity,
        Signature::JavaType::Primitive(Signature::Primitive::Void),
        &[],
    )?;

    let toast_class = java_env.find_class("android/widget/Toast")?;
    let field_length_short =
        java_env.get_static_field_id(toast_class, "LENGTH_SHORT", PRIMITIVE_INT_SIGNATURE)?;

    let length_short = java_env.get_static_field_unchecked(
        toast_class,
        field_length_short,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
    )?;

    // let arg_text = java_env.new_string(String::from("Permission granted"))?;
    let method_make_text = java_env.get_method_id(
        toast_class,
        "makeText",
        "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;",
    )?;
    let method_show = java_env.get_method_id(toast_class, "show", "()V")?;

    let text = java_env.new_object_array(
        ARRAY_LENGTH,
        java_env.find_class(JAVA_STRING_SIGNATURE)?,
        java_env.new_string(String::from("Permission granted"))?,
    )?;

    let object_make_text = java_env
        .call_method_unchecked(
            object_on_request_permissions_result,
            method_make_text,
            Signature::JavaType::Primitive(Signature::Primitive::Void),
            &[application_activity.into(), text.into(), length_short],
        )?
        .l()?;

    let object_show = java_env
        .call_method_unchecked(
            object_make_text,
            method_make_text,
            Signature::JavaType::Primitive(Signature::Primitive::Void),
            &[application_activity.into(), text.into(), length_short],
        )?
        .l()?;

    // let show_method = java_env.call_method(toast_class, "show", "()V", &[])?;
    Ok(())
}
