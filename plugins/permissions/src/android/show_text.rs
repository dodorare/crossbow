use jni::sys::_jobject;

use super::*;

pub fn show_text(status: &str) -> crate::error::Result<()> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    // makeText() 1st arg
    let context_class = java_env.find_class("android/content/Context")?;
    // let method_request_permissions = java_env.get_method_id(
    //     class_activity,
    //     REQUEST_PERMISSIONS_METHOD,
    //     REQUEST_PERMISSIONS_SIGNATURE,
    // )?;

    let get_application_activity =
        java_env.get_method_id(context_class, "getApplicationContext", "()V")?;

    let application_activity = java_env.call_method_unchecked(
        ctx.context().cast(),
        get_application_activity,
        Signature::JavaType::Primitive(Signature::Primitive::Void),
        &[],
    )?;
    println!("application_activity {:?}", application_activity);

    let toast_class = java_env.find_class("android/widget/Toast")?;
    let field_length_short =
        java_env.get_static_field_id(toast_class, "LENGTH_SHORT", PRIMITIVE_INT_SIGNATURE)?;

    let length_short = java_env.get_static_field_unchecked(
        toast_class,
        field_length_short,
        Signature::JavaType::Primitive(Signature::Primitive::Int),
    )?;
    // let text = java_env.new_string(String::from("Permission granted"))?;
    let make_text = java_env.get_method_id(
        toast_class,
        "makeText",
        "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;",
    )?;
    let show = java_env.get_method_id(toast_class, "show", "()V")?;

    let text = java_env.new_object_array(
        ARRAY_LENGTH,
        java_env.find_class(JAVA_STRING_SIGNATURE)?,
        java_env.new_string(String::from(status))?,
    )?;

    let make_text_method = java_env
        .call_method_unchecked(
            ctx.context().cast(),
            make_text,
            Signature::JavaType::Primitive(Signature::Primitive::Void),
            &[application_activity.into(), text.into(), length_short],
        )?
        .l()?;

    let show_method = java_env.call_method_unchecked(
        make_text_method,
        show,
        Signature::JavaType::Primitive(Signature::Primitive::Void),
        &[],
    )?;
    Ok(())
}
