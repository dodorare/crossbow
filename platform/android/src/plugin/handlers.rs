use super::*;
use crate::{error::*, utils::*};
use async_channel::unbounded;
use jni::{
    Env,
    objects::{JObject, JObjectArray, JString},
    signature::{JavaType, RuntimeMethodSignature},
};

pub(crate) fn on_native_register_singleton(
    env: &mut Env,
    name: &JString,
    obj: &JObject,
) -> Result<()> {
    let singleton_name = jstring_to_string(env, name)?;
    println!("Crossbow register plugin {:?}: {:?}", singleton_name, obj);
    let (sender, receiver) = unbounded();
    let singleton = JniSingleton::new(&singleton_name, env.new_global_ref(obj)?, receiver);
    insert_jni_singleton(&singleton_name, singleton);
    insert_sender(&singleton_name, sender);
    Ok(())
}

pub(crate) fn on_native_register_method(
    env: &mut Env,
    sname: &JString,
    name: &JString,
    sig: &JString,
) -> Result<()> {
    let singleton_name = jstring_to_string(env, sname)?;
    let singleton = get_jni_singleton_with_error(&singleton_name)?;
    let mut singleton = (*singleton).clone();

    let mname = jstring_to_string(env, name)?;
    let sig = jstring_to_string(env, sig)?;
    let signature = RuntimeMethodSignature::from_str(sig)?;

    singleton.add_method(&mname, signature);
    insert_jni_singleton(&singleton_name, singleton);
    Ok(())
}

pub(crate) fn on_native_register_signal(
    env: &mut Env,
    plugin_name: &JString,
    signal_name: &JString,
    signal_param_types: &JObjectArray<JString>,
) -> Result<()> {
    let singleton_name = jstring_to_string(env, plugin_name)?;
    let singleton = get_jni_singleton_with_error(&singleton_name)?;
    let mut singleton = (*singleton).clone();

    let mut types: Vec<JavaType> = vec![];
    let param_types_count = signal_param_types.len(env)?;
    for i in 0..param_types_count {
        let param_type_obj = signal_param_types.get_element(env, i)?;
        let param_type_str = jstring_to_string(env, &param_type_obj)?;
        let param_type = param_type_str.parse::<JavaType>()?;
        types.push(param_type);
    }

    let signal_name = jstring_to_string(env, signal_name)?;
    singleton.add_signal_info(&signal_name, types);
    insert_jni_singleton(&singleton_name, singleton);
    Ok(())
}

pub(crate) fn on_native_emit_signal(
    env: &mut Env,
    plugin_name: &JString,
    signal_name: &JString,
    signal_params: &JObjectArray,
) -> Result<()> {
    let signal_name = jstring_to_string(env, signal_name)?;
    let singleton_name = jstring_to_string(env, plugin_name)?;
    let sender = get_sender(&singleton_name)?;

    let mut args: Vec<JniRustType> = vec![];
    let params_count = signal_params.len(env)?;
    for i in 0..params_count {
        let param_obj = signal_params.get_element(env, i)?;
        let val = JniRustType::from_jobject(env, param_obj)?;
        args.push(val);
    }

    sender.try_send(Signal {
        name: signal_name,
        args,
    })?;
    Ok(())
}
