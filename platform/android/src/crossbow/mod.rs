use crate::error::*;
use jni::{objects::JObject, JNIEnv};

pub(crate) fn crossbow_initialize(
    env: JNIEnv,
    activity: JObject,
    crossbow_instance: JObject,
    _asset_manager: JObject,
) -> Result<()> {
    println!("CrossbowLib_initialize: {:?}", activity);

    env.call_method(crossbow_instance, "onRenderInit", "()V", &[])?;
    env.exception_check()?;

    // TODO: Create wrapper around CrossbowInstance

    Ok(())
}

pub(crate) fn crossbow_on_back_pressed(_env: JNIEnv) -> Result<()> {
    println!("CrossbowLib_onBackPressed");
    Ok(())
}

pub(crate) fn crossbow_on_destroy(_env: JNIEnv) -> Result<()> {
    println!("CrossbowLib_onDestroy");
    Ok(())
}
