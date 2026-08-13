use crate::plugin::*;
use jni::{
    EnvUnowned,
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray, JString},
};

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeRegisterSingleton<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
    obj: JObject<'local>,
) {
    env.with_env(|env| on_native_register_singleton(env, &name, &obj))
        .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeRegisterMethod<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    sname: JString<'local>,
    name: JString<'local>,
    sig: JString<'local>,
) {
    env.with_env(|env| on_native_register_method(env, &sname, &name, &sig))
        .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeRegisterSignal<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    plugin_name: JString<'local>,
    signal_name: JString<'local>,
    signal_param_types: JObjectArray<'local, JString<'local>>,
) {
    env.with_env(|env| {
        on_native_register_signal(env, &plugin_name, &signal_name, &signal_param_types)
    })
    .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_plugin_CrossbowPlugin_nativeEmitSignal<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    plugin_name: JString<'local>,
    signal_name: JString<'local>,
    signal_params: JObjectArray<'local>,
) {
    env.with_env(|env| on_native_emit_signal(env, &plugin_name, &signal_name, &signal_params))
        .resolve::<ThrowRuntimeExAndDefault>();
}
