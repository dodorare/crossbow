use crate::{error::*, types::*, utils::*};
use async_channel::{unbounded, Sender};
use jni::{
    objects::{JObject, JString},
    signature::{JavaType, TypeSignature},
    sys::jobjectArray,
    JNIEnv,
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    static ref JNI_SINGLETONS: Mutex<HashMap<String, Arc<JniSingleton>>> = Default::default();
    static ref JNI_SIGNAL_SENDERS: Mutex<HashMap<String, Sender<Signal>>> = Default::default();
}

fn insert_jni_singleton(
    singleton_name: &str,
    singleton: JniSingleton,
) -> Option<Arc<JniSingleton>> {
    let mut jni_signletons_guard = JNI_SINGLETONS.lock().unwrap();
    jni_signletons_guard.insert(singleton_name.to_owned(), Arc::new(singleton))
}

fn insert_sender(singleton_name: &str, sender: Sender<Signal>) -> Option<Sender<Signal>> {
    JNI_SIGNAL_SENDERS
        .lock()
        .unwrap()
        .insert(singleton_name.to_owned(), sender)
}

pub fn get_jni_singleton(singleton_name: &str) -> Option<Arc<JniSingleton>> {
    let jni_signletons_guard = JNI_SINGLETONS.lock().unwrap();
    jni_signletons_guard.get(singleton_name).cloned()
}

pub fn get_jni_singleton_with_error(singleton_name: &str) -> Result<Arc<JniSingleton>> {
    if let Some(jni_signleton) = get_jni_singleton(singleton_name) {
        Ok(jni_signleton.clone())
    } else {
        Err(AndroidError::SingletonNotRegistered(
            singleton_name.to_owned().into(),
        ))
    }
}

pub fn get_sender(singleton_name: &str) -> Result<Sender<Signal>> {
    let jni_signals = JNI_SIGNAL_SENDERS.lock().unwrap();
    let sender = jni_signals
        .get(singleton_name)
        .ok_or_else(|| AndroidError::SignalSenderNotAvailable(singleton_name.to_owned()))?;
    if sender.is_closed() {
        return Err(AndroidError::SignalSenderNotAvailable(
            singleton_name.to_owned(),
        ));
    }
    Ok(sender.clone())
}

pub(crate) fn native_register_singleton(env: JNIEnv, name: JString, obj: JObject) -> Result<()> {
    let singleton_name = jstring_to_string(&env, name)?;
    println!("Crossbow register plugin {:?}: {:?}", singleton_name, obj);
    let (sender, receiver) = unbounded();
    let singleton = JniSingleton::new(&singleton_name, env.new_global_ref(obj)?, receiver);
    insert_jni_singleton(&singleton_name, singleton);
    insert_sender(&singleton_name, sender);
    Ok(())
}

pub(crate) fn native_register_method(
    env: JNIEnv,
    sname: JString,
    name: JString,
    sig: JString,
) -> Result<()> {
    let singleton_name = jstring_to_string(&env, sname)?;
    let singleton = get_jni_singleton_with_error(&singleton_name)?;
    let mut singleton = (*singleton).clone();

    let mname = jstring_to_string(&env, name)?;
    let sig = jstring_to_string(&env, sig)?;
    let signature = TypeSignature::from_str(sig)?;

    let cls = env.get_object_class(singleton.get_instance())?;
    let class = env.new_global_ref(cls)?;
    singleton.add_method(&mname, class, signature);
    insert_jni_singleton(&singleton_name, singleton);
    Ok(())
}

pub(crate) fn native_register_signal(
    env: JNIEnv,
    plugin_name: JString,
    signal_name: JString,
    signal_param_types: jobjectArray,
) -> Result<()> {
    let singleton_name = jstring_to_string(&env, plugin_name)?;
    let singleton = get_jni_singleton_with_error(&singleton_name)?;
    let mut singleton = (*singleton).clone();

    let mut types: Vec<JavaType> = vec![];
    let param_types_count = env.get_array_length(signal_param_types)?;
    for i in 0..param_types_count {
        let param_type_obj = env.get_object_array_element(signal_param_types, i)?;
        let param_type_str = jstring_to_string(&env, param_type_obj.into())?;
        let param_type = JavaType::from_str(&param_type_str)?;
        types.push(param_type);
    }

    let signal_name = jstring_to_string(&env, signal_name)?;
    singleton.add_signal_info(&signal_name, types);
    insert_jni_singleton(&singleton_name, singleton);
    Ok(())
}

pub(crate) fn native_emit_signal(
    env: JNIEnv,
    plugin_name: JString,
    signal_name: JString,
    signal_params: jobjectArray,
) -> Result<()> {
    let signal_name = jstring_to_string(&env, signal_name)?;
    let singleton_name = jstring_to_string(&env, plugin_name)?;
    let sender = get_sender(&singleton_name)?;

    let mut args: Vec<JniRustType> = vec![];
    let params_count = env.get_array_length(signal_params)?;
    for i in 0..params_count {
        let param_obj = env.get_object_array_element(signal_params, i)?;
        let val = JniRustType::from_jobject(&env, param_obj)?;
        args.push(val);
        env.delete_local_ref(param_obj)?;
    }

    sender.try_send(Signal { signal_name, args }).unwrap();
    Ok(())
}
