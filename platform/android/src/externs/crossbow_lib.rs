use jni::{
    objects::{JClass, JObject, JString},
    sys::jboolean,
    JNIEnv,
};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_CrossbowLib_initialize(
    env: JNIEnv,
    _class: JClass,
    activity: JObject,
    crossbow_instance: JObject,
    _asset_manager: JObject,
) {
    println!("CrossbowLib_initialize: {:?}", activity);

    env.call_method(crossbow_instance, "onRenderInit", "()V", &[])
        .unwrap();

    // TODO: Create wrapper around CrossbowInstance
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_CrossbowLib_requestPermissionResult(
    _env: JNIEnv,
    _class: JClass,
    _permission: JString,
    result: jboolean,
) {
    println!("requestPermissionResult: {:?}", result);
}
