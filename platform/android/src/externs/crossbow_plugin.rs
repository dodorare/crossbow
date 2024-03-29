use crate::plugin::*;
use jni::{
    objects::{JClass, JObject, JString},
    sys::jobjectArray,
    JNIEnv,
};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeRegisterSingleton(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    obj: JObject,
) {
    on_native_register_singleton(env, name, obj).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeRegisterMethod(
    env: JNIEnv,
    _class: JClass,
    sname: JString,
    name: JString,
    sig: JString,
) {
    on_native_register_method(env, sname, name, sig).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeRegisterSignal(
    env: JNIEnv,
    _class: JClass,
    plugin_name: JString,
    signal_name: JString,
    signal_param_types: jobjectArray,
) {
    on_native_register_signal(env, plugin_name, signal_name, signal_param_types).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeEmitSignal(
    env: JNIEnv,
    _class: JClass,
    plugin_name: JString,
    signal_name: JString,
    signal_params: jobjectArray,
) {
    on_native_emit_signal(env, plugin_name, signal_name, signal_params).unwrap();
}
