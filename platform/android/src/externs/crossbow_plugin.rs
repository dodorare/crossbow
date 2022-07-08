use crate::plugin::*;
use jni::{
    objects::{JClass, JObject, JString},
    sys::jobjectArray,
    JNIEnv,
};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterSingleton(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    obj: JObject,
) {
    native_register_singleton(env, name, obj).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterMethod(
    env: JNIEnv,
    _class: JClass,
    sname: JString,
    name: JString,
    sig: JString,
) {
    native_register_method(env, sname, name, sig).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterSignal(
    env: JNIEnv,
    _class: JClass,
    plugin_name: JString,
    signal_name: JString,
    signal_param_types: jobjectArray,
) {
    native_register_signal(env, plugin_name, signal_name, signal_param_types).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeEmitSignal(
    env: JNIEnv,
    _class: JClass,
    plugin_name: JString,
    signal_name: JString,
    signal_params: jobjectArray,
) {
    native_emit_signal(env, plugin_name, signal_name, signal_params).unwrap();
}
