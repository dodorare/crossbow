use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[macroquad::main("Macroquad UI")]
async fn main() -> anyhow::Result<()> {
    let skin = {
        let label_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(0., 0., 0., 0.))
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color_selected(Color::from_rgba(190, 190, 190, 255))
            .font_size(50)
            .build();

        Skin {
            editbox_style,
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };

    let window_skin = skin.clone();
    loop {
        clear_background(BROWN);

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(150.0, 250.0), vec2(300., 300.), |ui| {
            if ui.button(vec2(-25.0, 100.0), "Ask permission") {
                draw_text_ex("Permission asked!", 100.0, 200.0, TextParams::default());

                #[cfg(target_os = "android")]
                ask_for_permission();
            }
        });
        root_ui().pop_skin();
        next_frame().await;
    }
}

fn has_permission(permission: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Create a VM for executing Java calls
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    let java_env = vm.attach_current_thread()?;
    // TODO: Replace with android context?
    // unsafe { ndk::native_activity::NativeActivity::from_ptr(ctx.vm()) };
    let native_activity = crossbow::ndk_glue::native_activity();

    let class_manifest_permission = java_env.find_class("android/Manifest$permission")?;
    let field_permission = java_env.get_static_field_id(
        class_manifest_permission,
        permission,
        "Ljava/lang/String;",
    )?;
    let string_permission = java_env.get_static_field_unchecked(
        class_manifest_permission,
        field_permission,
        jni::signature::JavaType::Object("java/lang/String".to_owned()),
    )?;

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
        native_activity.activity(),
        method_check_self_permission,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Int),
        &[string_permission],
    )?;
    Ok(ret.i()? == int_permission_granted.i()?)
}

fn request_permission(permission: &str) -> Result<bool, Box<dyn std::error::Error>>
// where <F>, on_request_done: F
//     F: FnOnce(&str, bool) -> (),
{
    if has_permission(permission)? {
        return Ok(true);
    }

    // Create a VM for executing Java calls
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    let java_env = vm.attach_current_thread()?;
    let native_activity = crossbow::ndk_glue::native_activity();

    let array_permissions = java_env.new_object_array(
        1,
        java_env.find_class("java/lang/String")?,
        java_env.new_string("")?,
    )?;
    let class_manifest_permission = java_env.find_class("android/Manifest$permission")?;
    let field_permission = java_env.get_static_field_id(
        class_manifest_permission,
        permission,
        "Ljava/lang/String;",
    )?;
    let string_permission = java_env.get_static_field_unchecked(
        class_manifest_permission,
        field_permission,
        jni::signature::JavaType::Object("java/lang/String".to_owned()),
    )?;

    java_env.set_object_array_element(array_permissions, 0, string_permission.l()?)?;
    let class_activity = java_env.find_class("android/app/Activity")?;
    let method_request_permissions = java_env.get_method_id(
        class_activity,
        "requestPermissions",
        "([Ljava/lang/String;I)V",
    )?;

    java_env.call_method_unchecked(
        native_activity.activity(),
        method_request_permissions,
        jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
        &[array_permissions.into(), jni::objects::JValue::Int(0)],
    )?;
    // /* TODO: How to create a native callback for a Java class for last argument (0) */
    // env->CallVoidMethod(mApp->activity->clazz, MethodrequestPermissions, ArrayPermissions, 0);

    Ok(true)
}

// #[cfg(target_os = "android")]
fn ask_for_permission() -> Result<(), Box<dyn std::error::Error>> {
    println!("Has INTERNET permission: {}", has_permission("INTERNET")?,);
    println!(
        "Has READ_EXTERNAL_STORAGE permission: {}",
        has_permission("READ_EXTERNAL_STORAGE")?,
    );
    request_permission("CAMERA")?;

    // const GET_DEVICES_OUTPUTS: jni::sys::jint = 2;

    // // Query the global Audio Service
    // let class_ctxt = env.find_class("android/content/Context")?;
    // let audio_service = env.get_static_field(class_ctxt, "AUDIO_SERVICE", "Ljava/lang/String;")?;

    // let audio_manager = env
    //     .call_method(
    //         ctx.context().cast(),
    //         "getSystemService",
    //         // JNI type signature needs to be derived from the Java API
    //         // (ArgTys)ResultTy
    //         "(Ljava/lang/String;)Ljava/lang/Object;",
    //         &[audio_service],
    //     )?
    //     .l()?;

    // // Enumerate output devices
    // let devices = env.call_method(
    //     audio_manager,
    //     "getDevices",
    //     "(I)[Landroid/media/AudioDeviceInfo;",
    //     &[GET_DEVICES_OUTPUTS.into()],
    // )?;

    // let device_array = devices.l()?.into_inner();
    // let len = env.get_array_length(device_array)?;
    // for i in 0..len {
    //     let device = env.get_object_array_element(device_array, i)?;

    //     // Collect device information
    //     // See https://developer.android.com/reference/android/media/AudioDeviceInfo
    //     let product_name: String = {
    //         let name =
    //             env.call_method(device, "getProductName", "()Ljava/lang/CharSequence;", &[])?;
    //         let name = env.call_method(name.l()?, "toString", "()Ljava/lang/String;", &[])?;
    //         env.get_string(name.l()?.into())?.into()
    //     };
    //     let id = env.call_method(device, "getId", "()I", &[])?.i()?;
    //     let ty = env.call_method(device, "getType", "()I", &[])?.i()?;

    //     let sample_rates = {
    //         let sample_array = env
    //             .call_method(device, "getSampleRates", "()[I", &[])?
    //             .l()?
    //             .into_inner();
    //         let len = env.get_array_length(sample_array)?;

    //         let mut sample_rates = vec![0; len as usize];
    //         env.get_int_array_region(sample_array, 0, &mut sample_rates)?;
    //         sample_rates
    //     };

    //     println!("Device {}: Id {}, Type {}", product_name, id, ty);
    //     println!("sample rates: {:#?}", sample_rates);
    // }

    Ok(())
}
