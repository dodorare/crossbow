use crate::crossbow::*;
use jni::{
    EnvUnowned,
    errors::ThrowRuntimeExAndDefault,
    objects::{Global, JClass, JObject, JString},
    sys::jboolean,
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref JAVA_ACTIVITY: Mutex<Option<Global<JObject<'static>>>> = Mutex::new(None);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_initializeAndroidContext<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    activity: JObject<'local>,
) {
    env.with_env(|env| -> jni::errors::Result<()> {
        let vm = env.get_java_vm()?;
        let activity = env.new_global_ref(&activity)?;
        unsafe {
            ndk_context::initialize_android_context(
                vm.get_raw().cast(),
                activity.as_ref().as_raw().cast(),
            );
        }
        *JAVA_ACTIVITY.lock().expect("Java activity mutex poisoned") = Some(activity);
        Ok(())
    })
    .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_releaseAndroidContext<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    env.with_env(|_| -> jni::errors::Result<()> {
        unsafe { ndk_context::release_android_context() };
        JAVA_ACTIVITY
            .lock()
            .expect("Java activity mutex poisoned")
            .take();
        Ok(())
    })
    .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_initialize<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    activity: JObject<'local>,
    crossbow_instance: JObject<'local>,
    asset_manager: JObject<'local>,
) {
    env.with_env(|env| {
        CrossbowInstance::crossbow_on_initialize(env, &activity, &crossbow_instance, &asset_manager)
    })
    .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_onBackPressed<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    env.with_env(CrossbowInstance::crossbow_on_back_pressed)
        .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_onDestroy<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    env.with_env(CrossbowInstance::crossbow_on_destroy)
        .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_focusIn<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    env.with_env(CrossbowInstance::crossbow_on_focus_in)
        .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_focusOut<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    env.with_env(CrossbowInstance::crossbow_on_focus_out)
        .resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_crossbow_library_CrossbowLib_requestPermissionResult<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    permission: JString<'local>,
    result: jboolean,
) {
    env.with_env(|env| CrossbowInstance::on_request_permission_result(env, &permission, result))
        .resolve::<ThrowRuntimeExAndDefault>();
}
