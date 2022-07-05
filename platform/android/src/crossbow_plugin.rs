#![allow(unused_variables)]

use crate::utils::*;
use jni::{
    objects::{JClass, JObject, JString},
    sys::{jobject, jobjectArray},
    JNIEnv,
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

// val pluginRegistry = crossbowFragment!!.pluginRegistry
// if (pluginRegistry === null) {
//     Log.e("CrossbowApp", "CrossbowFragment.pluginRegistry is null")
// }
// val admob: AdMob = pluginRegistry!!.getPlugin("AdMob") as AdMob
// admob.initialize(true, "G", false, true)
// admob.load_interstitial("ca-app-pub-3940256099942544/1033173712")
// admob.show_interstitial()

pub struct JniSingleton {
    instance: jobject,
}

static mut JNI_SINGLETONS: Lazy<Mutex<HashMap<String, JniSingleton>>> = Lazy::new(Default::default);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterSingleton(
    env: JNIEnv,
    class: JClass,
    name: JString,
    obj: JObject,
) {
    let singname = jstring_to_string(&env, name).unwrap();
    let singleton = JniSingleton {
        instance: obj.into_inner(),
    };
    let mut jni_singletons_guard = unsafe { JNI_SINGLETONS.lock().unwrap() };
    jni_singletons_guard.insert(singname, singleton);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterMethod(
    env: JNIEnv,
    class: JClass,
    sname: JString,
    name: JString,
    ret: JString,
    args: jobjectArray,
) {
    let singname = jstring_to_string(&env, sname).unwrap();
    let mut jni_singletons_guard = unsafe { JNI_SINGLETONS.lock().unwrap() };
    let mut singleton = jni_singletons_guard.get_mut(&singname);

    if singleton.is_none() {
        println!("Plugin singleton {} is not registered", singname);
        return;
    }

    let mname = jstring_to_string(&env, name).unwrap();
    let retval = jstring_to_string(&env, ret).unwrap();

    let mut types: Vec<&'static str> = vec![];
    let mut cs = "(";

    let string_count = env.get_array_length(args).unwrap();

    // TODO: Fix this. Probably will not work well.
    for i in 0..string_count {
        let jstring = env.get_object_array_element(args, i).unwrap();
        let raw_string = jstring_to_string(&env, jstring.into()).unwrap();
        // let arg_type = get_jni_type(raw_string);
        // types.push(arg_type);
        // cs += &arg_type;
    }

    cs = &[")", get_jni_sig(&retval)].concat();

    // jclass cls = env->GetObjectClass(s->get_instance());
    // jmethodID mid = env->GetMethodID(cls, mname.ascii().get_data(), cs.ascii().get_data());
    // if (!mid) {
    // 	print_line("Failed getting method ID " + mname);
    // }

    // s->add_method(mname, mid, types, get_jni_type(retval));
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
