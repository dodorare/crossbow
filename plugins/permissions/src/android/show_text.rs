// use super::*;
// use crate::{error::*, types::android::*};
// use jni::signature as Signature;
// use jni::sys::_jobject;

// pub fn show_text(
//     permission: AndroidPermission,
//     // request_permission: jni::objects::JValue<'a>,
// ) -> crate::error::Result<bool> {
//     let (ctx, vm) = create_java_vm()?;
//     let java_env = vm.attach_current_thread()?;

//     let string_permission = get_permission_from_manifest(permission, &java_env)?;
//     let array = java_env.new_object_array(
//         ARRAY_LENGTH,
//         java_env.find_class(JAVA_STRING_SIGNATURE)?,
//         java_env.new_string("")?,
//     )?;
//     // let request_permission = invoke_request_permission_method(permission, &java_env)?;
//     java_env.set_object_array_element(array, OBJECT_INDEX.into(), string_permission.l()?)?;
//     let class = java_env.find_class(ANDROID_ACTIVITY)?;
//     let get_on_request_permissions_result = java_env.get_method_id(
//         class,
//         "onRequestPermissionsResult",
//         "(I[Ljava/lang/String;[I)V",
//     )?;
//     let object_on_request_permissions_result = java_env.call_method_unchecked(
//         ctx.context().cast(),
//         get_on_request_permissions_result,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[
//             jni::objects::JValue::Int(1),
//             array.into(),
//             jni::objects::JValue::Int(1),
//         ],
//     )?;

//     let toast_class = java_env.find_class("android/widget/Toast")?;
//     let field_length_short =
//         java_env.get_static_field_id(toast_class, "LENGTH_SHORT", PRIMITIVE_INT_SIGNATURE)?;

//     let length_short = java_env.get_static_field_unchecked(
//         toast_class,
//         field_length_short,
//         Signature::JavaType::Primitive(Signature::Primitive::Int),
//     )?;

//     // let arg_text = java_env.new_string(String::from("Permission granted"))?;
//     let method_make_text = java_env.get_method_id(
//         toast_class,
//         "makeText",
//         "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;",
//     )?;
//     let method_show = java_env.get_method_id(toast_class, "show", "()V")?;

//     let text = java_env.new_object_array(
//         ARRAY_LENGTH,
//         java_env.find_class(JAVA_STRING_SIGNATURE)?,
//         java_env.new_string("Permission granted")?,
//     )?;

//     let object_make_text = java_env.call_method_unchecked(
//         // object_on_request_permissions_result,
//         object_on_request_permissions_result.l()?,
//         method_make_text,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[text.into(), length_short],
//     )?;

//     let object_show = java_env.call_method_unchecked(
//         object_make_text.l()?,
//         method_show,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[],
//     )?;

//     // let show_method = java_env.call_method(toast_class, "show", "()V", &[])?;
//     Ok(true)
// }

// pub fn show_text_from_on_request_permissions_result(permission: AndroidPermission) -> Result<()> {
//     let (ctx, vm) = create_java_vm()?;
//     let java_env = vm.attach_current_thread()?;

//     let string_permission = get_permission_from_manifest(permission, &java_env)?;
//     let array = java_env.new_object_array(
//         ARRAY_LENGTH,
//         java_env.find_class(JAVA_STRING_SIGNATURE)?,
//         java_env.new_string("")?,
//     )?;
//     // let request_permission = invoke_request_permission_method(permission, &java_env)?;
//     java_env.set_object_array_element(array, OBJECT_INDEX.into(), string_permission.l()?)?;
//     let class = java_env.find_class(ANDROID_ACTIVITY)?;
//     let get_on_request_permissions_result = java_env.get_method_id(
//         class,
//         "onRequestPermissionsResult",
//         "(I[Ljava/lang/String;[I)V",
//     )?;
//     let object_on_request_permissions_result = java_env.call_method_unchecked(
//         ctx.context().cast(),
//         get_on_request_permissions_result,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[
//             jni::objects::JValue::Int(1),
//             array.into(),
//             jni::objects::JValue::Int(1),
//         ],
//     )?;

//     let toast_class = java_env.find_class("android/widget/Toast")?;
//     let field_length_short =
//         java_env.get_static_field_id(toast_class, "LENGTH_SHORT", PRIMITIVE_INT_SIGNATURE)?;

//     let length_short = java_env.get_static_field_unchecked(
//         toast_class,
//         field_length_short,
//         Signature::JavaType::Primitive(Signature::Primitive::Int),
//     )?;

//     // let arg_text = java_env.new_string(String::from("Permission granted"))?;
//     let method_make_text = java_env.get_method_id(
//         toast_class,
//         "makeText",
//         "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;",
//     )?;
//     let method_show = java_env.get_method_id(toast_class, "show", "()V")?;

//     let text = java_env.new_object_array(
//         ARRAY_LENGTH,
//         java_env.find_class(JAVA_STRING_SIGNATURE)?,
//         java_env.new_string("Permission granted")?,
//     )?;

//     let object_make_text = java_env.call_method_unchecked(
//         ctx.context().cast(),
//         // object_on_request_permissions_result.l()?,
//         method_make_text,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[text.into(), length_short],
//     )?;

//     let object_show = java_env.call_method_unchecked(
//         ctx.context().cast(),
//         // object_make_text.l()?,
//         method_show,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[],
//     )?;
//     Ok(())
// }

// pub fn show_text_from_main_activity_class(permission: AndroidPermission) -> Result<()> {
//     let (ctx, vm) = create_java_vm()?;
//     let java_env = vm.attach_current_thread()?;

//     let string_permission = get_permission_from_manifest(permission, &java_env)?;
//     let array = java_env.new_object_array(
//         ARRAY_LENGTH,
//         java_env.find_class(JAVA_STRING_SIGNATURE)?,
//         java_env.new_string("")?,
//     )?;
//     let int_array = java_env.new_object_array(
//         ARRAY_LENGTH,
//         java_env.find_class(PRIMITIVE_INT_SIGNATURE)?,
//         java_env.new_int_array(2)?,
//     )?;
//     // let request_permission = invoke_request_permission_method(permission, &java_env)?;
//     java_env.set_object_array_element(array, OBJECT_INDEX.into(), string_permission.l()?)?;
//     let class = java_env.find_class("com/crossbow/permission/MainActivity")?;
//     let get_on_request_permissions_result = java_env.get_method_id(
//         class,
//         "onRequestPermissionsResult",
//         "(I[Ljava/lang/String;[I)V",
//     )?;
//     let object_on_request_permissions_result = java_env.call_method_unchecked(
//         ctx.context().cast(),
//         get_on_request_permissions_result,
//         Signature::JavaType::Primitive(Signature::Primitive::Void),
//         &[jni::objects::JValue::Int(1), array.into(), int_array.into()],
//     )?;
//     Ok(())
// }
