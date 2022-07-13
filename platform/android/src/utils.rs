use crate::error::*;
use jni::{
    objects::{JClass, JString},
    signature::JavaType,
    strings::JavaStr,
    JNIEnv,
};

pub fn jstring_to_string(env: &JNIEnv, jstring: JString) -> Result<String> {
    Ok(JavaStr::from_env(env, jstring)?.into())
}

/// Calls java.lang.Class.getName() and returns ClassName with is_array bool.
pub fn get_class_name(env: &JNIEnv, cls: JClass) -> Result<String> {
    let cclass = env.find_class("java/lang/Class")?;
    let get_name = env.get_method_id(cclass, "getName", "()Ljava/lang/String;")?;
    let cls_name =
        env.call_method_unchecked(cls, get_name, JavaType::Object("".to_owned()), &[])?;

    // let is_array_mid = env.get_method_id(cclass, "isArray", "()Z")?;
    // let is_arr = env.call_method_unchecked(
    //     cls,
    //     is_array_mid,
    //     JavaType::Primitive(Primitive::Boolean),
    //     &[],
    // )?;
    // let is_array = is_arr.z()?;

    let name = jstring_to_string(env, cls_name.l()?.into())?;
    Ok(name)
}
