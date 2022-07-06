use crate::{
    types::{JniRustType, JniSingleton},
    utils::*,
};
use jni::{
    objects::{JClass, JObject, JString},
    signature::{JavaType, TypeSignature},
    sys::jobjectArray,
    JNIEnv,
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{mpsc::Sender, Mutex, MutexGuard},
};

lazy_static::lazy_static! {
    static ref JNI_SINGLETONS: Mutex<HashMap<String, JniSingleton>> = Default::default();
    static ref JNI_SIGNALS: Mutex<Option<Sender<Signal>>> = Mutex::new(None);
}

#[derive(Clone, Debug)]
pub struct Signal {
    pub plugin_name: String,
    pub signal_name: String,
    pub args: Vec<JniRustType>,
}

pub fn get_jni_singletons<'a>() -> MutexGuard<'a, HashMap<String, JniSingleton>> {
    JNI_SINGLETONS.lock().unwrap()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dodorare_crossbow_plugin_CrossbowPlugin_nativeRegisterSingleton(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    obj: JObject,
) {
    let singname = jstring_to_string(&env, name).unwrap();
    println!("Crossbow register plugin {:?}: {:?}", singname, obj);
    let singleton = JniSingleton::new(env.new_global_ref(obj).unwrap());
    let mut jni_singletons_guard = get_jni_singletons();
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
    let mut jni_singletons_guard = get_jni_singletons();
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
    let class = env.new_global_ref(cls).unwrap();
    singleton.add_method(&mname, class, signature);
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
    let singname = jstring_to_string(&env, plugin_name).unwrap();
    let mut jni_singletons_guard = get_jni_singletons();
    let singleton = if let Some(singleton) = jni_singletons_guard.get_mut(&singname) {
        singleton
    } else {
        println!("Plugin singleton {} is not registered", singname);
        return;
    };

    let mut types: Vec<JavaType> = vec![];
    let param_types_count = env.get_array_length(signal_param_types).unwrap();
    for i in 0..param_types_count {
        let param_type_obj = env.get_object_array_element(signal_param_types, i).unwrap();
        let param_type_str = jstring_to_string(&env, param_type_obj.into()).unwrap();
        let param_type = JavaType::from_str(&param_type_str).unwrap();
        types.push(param_type);
    }

    let signal_name = jstring_to_string(&env, signal_name).unwrap();
    singleton.add_signal(&signal_name, types);
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
    let singname = jstring_to_string(&env, plugin_name).unwrap();
    let mut jni_singletons_guard = get_jni_singletons();
    let _singleton = if let Some(singleton) = jni_singletons_guard.get_mut(&singname) {
        singleton
    } else {
        println!("Plugin singleton {} is not registered", singname);
        return;
    };

    let mut args: Vec<JniRustType> = vec![];
    let params_count = env.get_array_length(signal_params).unwrap();
    for i in 0..params_count {
        let param_obj = env.get_object_array_element(signal_params, i).unwrap();
        let val = JniRustType::from_jobject(&env, param_obj).unwrap();
        args.push(val);
        env.delete_local_ref(param_obj).unwrap();
    }

    let signal_name = jstring_to_string(&env, signal_name).unwrap();
    // singleton.emit_signal(&signal_name, args);

    let signal = Signal {
        plugin_name: singname,
        signal_name,
        args,
    };
    println!("Emit signal {:?}", signal);
}
