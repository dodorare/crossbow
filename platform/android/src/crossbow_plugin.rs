use crate::{types::JniSingleton, utils::*};
use jni::{
    objects::{JClass, JObject, JString},
    signature::TypeSignature,
    sys::jobjectArray,
    JNIEnv,
};
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

static mut JNI_SINGLETONS: Lazy<Mutex<HashMap<String, JniSingleton>>> = Lazy::new(Default::default);

pub fn get_jni_singletons<'a>() -> MutexGuard<'a, HashMap<String, JniSingleton>> {
    unsafe { JNI_SINGLETONS.lock().unwrap() }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterSingleton(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    obj: JObject,
) {
    println!("CrossbowPlugin_nativeRegisterSingleton: {:?}", obj);
    let singname = jstring_to_string(&env, name).unwrap();
    let singleton = JniSingleton::new(env.new_global_ref(obj).unwrap());
    let mut jni_singletons_guard = unsafe { JNI_SINGLETONS.lock().unwrap() };
    jni_singletons_guard.insert(singname, singleton);
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
    let singname = jstring_to_string(&env, sname).unwrap();
    let mut jni_singletons_guard = unsafe { JNI_SINGLETONS.lock().unwrap() };
    let singleton = if let Some(singleton) = jni_singletons_guard.get_mut(&singname) {
        singleton
    } else {
        println!("Plugin singleton {} is not registered", singname);
        return;
    };

    let mname = jstring_to_string(&env, name).unwrap();
    let sig = jstring_to_string(&env, sig).unwrap();
    let signature = TypeSignature::from_str(sig).unwrap();

    let cls = env.get_object_class(singleton.get_instance()).unwrap();
    let method_id = match env.get_method_id(cls, &mname, signature.to_string()) {
        Ok(mid) => mid.into_inner(),
        Err(e) => {
            println!(
                "Failed getting method_id '{}' with sig '{}': {:?}",
                mname, signature, e
            );
            return;
        }
    };
    singleton.add_method(&mname, method_id, signature);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterSignal(
    env: JNIEnv,
    class: JClass,
    j_plugin_name: JString,
    j_signal_name: JString,
    j_signal_param_types: jobjectArray,
) {
    // String singleton_name = jstring_to_string(j_plugin_name, env);

    // ERR_FAIL_COND(!jni_singletons.has(singleton_name));

    // JNISingleton *singleton = jni_singletons.get(singleton_name);

    // String signal_name = jstring_to_string(j_signal_name, env);
    // Vector<Variant::Type> types;

    // int stringCount = env->GetArrayLength(j_signal_param_types);

    // for (int i = 0; i < stringCount; i++) {
    // 	jstring j_signal_param_type = (jstring)env->GetObjectArrayElement(j_signal_param_types, i);
    // 	const String signal_param_type = jstring_to_string(j_signal_param_type, env);
    // 	types.push_back(get_jni_type(signal_param_type));
    // }

    // singleton->add_signal(signal_name, types);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeEmitSignal(
    env: JNIEnv,
    class: JClass,
    j_plugin_name: JString,
    j_signal_name: JString,
    j_signal_params: jobjectArray,
) {
    // String singleton_name = jstring_to_string(j_plugin_name, env);

    // ERR_FAIL_COND(!jni_singletons.has(singleton_name));

    // JNISingleton *singleton = jni_singletons.get(singleton_name);

    // String signal_name = jstring_to_string(j_signal_name, env);

    // int count = env->GetArrayLength(j_signal_params);

    // Variant *variant_params = (Variant *)alloca(sizeof(Variant) * count);
    // const Variant **args = (const Variant **)alloca(sizeof(Variant *) * count);

    // for (int i = 0; i < count; i++) {
    // 	jobject j_param = env->GetObjectArrayElement(j_signal_params, i);
    // 	variant_params[i] = _jobject_to_variant(env, j_param);
    // 	args[i] = &variant_params[i];
    // 	env->DeleteLocalRef(j_param);
    // }

    // singleton->emit_signalp(StringName(signal_name), args, count);
}
