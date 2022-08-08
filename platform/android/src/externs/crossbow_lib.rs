use crate::crossbow::*;
use jni::{
    objects::{JClass, JObject, JString},
    sys::jboolean,
    JNIEnv,
};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_CrossbowLib_initialize(
    env: JNIEnv,
    _class: JClass,
    activity: JObject,
    crossbow_instance: JObject,
    asset_manager: JObject,
) {
    CrossbowInstance::crossbow_on_initialize(env, activity, crossbow_instance, asset_manager)
        .unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_CrossbowLib_onBackPressed(env: JNIEnv, _class: JClass) {
    CrossbowInstance::crossbow_on_back_pressed(env).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_CrossbowLib_onDestroy(env: JNIEnv, _class: JClass) {
    CrossbowInstance::crossbow_on_destroy(env).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_CrossbowLib_focusIn(env: JNIEnv, _class: JClass) {
    CrossbowInstance::crossbow_on_focus_in(env).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_CrossbowLib_focusOut(env: JNIEnv, _class: JClass) {
    CrossbowInstance::crossbow_on_focus_out(env).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_crossbow_library_CrossbowLib_requestPermissionResult(
    env: JNIEnv,
    _class: JClass,
    permission: JString,
    result: jboolean,
) {
    CrossbowInstance::on_request_permission_result(env, permission, result).unwrap();
}
