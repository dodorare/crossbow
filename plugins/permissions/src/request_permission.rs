#[cfg(target_os = "android")]
pub fn check_permission(permission: &str) -> crate::error::Result<bool> {
    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let string_permission = get_permission_from_context(permission, &java_env)?;

    let class_package_manager = java_env.find_class("android/content/pm/PackageManager")?;
    let field_permission_granted =
        java_env.get_static_field_id(class_package_manager, "PERMISSION_GRANTED", "I")?;
    let int_permission_granted = java_env.get_static_field_unchecked(
        class_package_manager,
        field_permission_granted,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Int),
    )?;

    let class_context = java_env.find_class("android/content/Context")?;
    let method_check_self_permission = java_env.get_method_id(
        class_context,
        "checkSelfPermission",
        "(Ljava/lang/String;)I",
    )?;
    let ret = java_env.call_method_unchecked(
        ctx.context().cast(),
        method_check_self_permission,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Int),
        &[string_permission],
    )?;
    Ok(ret.i()? == int_permission_granted.i()?)
}

#[cfg(target_os = "android")]
pub fn request_permission(permission: &str) -> crate::error::Result<bool> {
    if check_permission(permission)? {
        return Ok(true);
    }

    let (ctx, vm) = create_java_vm()?;
    let java_env = vm.attach_current_thread()?;

    let array_permissions = java_env.new_object_array(
        1,
        java_env.find_class("java/lang/String")?,
        java_env.new_string("")?,
    )?;

    let string_permission = get_permission_from_context(permission, &java_env)?;

    java_env.set_object_array_element(array_permissions, 0, string_permission.l()?)?;
    let class_activity = java_env.find_class("android/app/Activity")?;
    let method_request_permissions = java_env.get_method_id(
        class_activity,
        "requestPermissions",
        "([Ljava/lang/String;I)V",
    )?;

    java_env.call_method_unchecked(
        ctx.context().cast(),
        method_request_permissions,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
        &[array_permissions.into(), jni::objects::JValue::Int(0)],
    )?;
    // /* TODO: How to create a native callback for a Java class for last argument (0) */
    // env->CallVoidMethod(mApp->activity->clazz, MethodrequestPermissions, ArrayPermissions, 0);

    Ok(true)
}

#[cfg(target_os = "android")]
pub fn ask_for_permission() -> crate::error::Result<()> {
    // println!("Has INTERNET permission: {}", check_permission("INTERNET")?,);
    // println!(
    //     "Has READ_EXTERNAL_STORAGE permission: {}",
    //     check_permission("READ_EXTERNAL_STORAGE")?,
    // );
    request_permission("CAMERA")?;

    Ok(())
}

#[cfg(target_os = "android")]
/// Create a java VM for executing Java calls
pub fn create_java_vm() -> crate::error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}

#[cfg(target_os = "android")]
pub fn get_permission_from_context<'a>(
    permission: &'a str,
    java_env: &jni::AttachGuard<'a>,
) -> crate::error::Result<jni::objects::JValue<'a>> {
    let class_manifest_permission = java_env.find_class("android/Manifest$permission")?;
    let field_permission = java_env.get_static_field_id(
        class_manifest_permission,
        permission,
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
