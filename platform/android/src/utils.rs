use crate::error::*;
use jni::{
    Env, jni_sig, jni_str,
    objects::{JClass, JString},
};

pub fn jstring_to_string(env: &Env, jstring: &JString) -> Result<String> {
    Ok(jstring.try_to_string(env)?)
}

/// Calls java.lang.Class.getName() and returns ClassName with is_array bool.
pub fn get_class_name(env: &mut Env, cls: &JClass) -> Result<String> {
    let cls_name = env.call_method(
        cls,
        jni_str!("getName"),
        jni_sig!("()Ljava/lang/String;"),
        &[],
    )?;
    let cls_name = env.cast_local::<JString>(cls_name.l()?)?;
    let name = jstring_to_string(env, &cls_name)?;
    Ok(name)
}
