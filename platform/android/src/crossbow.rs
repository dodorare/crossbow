use crate::error::*;
use jni::{objects::JObject, JNIEnv};
// use std::sync::Mutex;

// lazy_static::lazy_static! {
//     static ref PERMISSION_SENDER: Mutex<Option<CrossbowInstance>> = Default::default();
// }

pub struct CrossbowInstance;

impl CrossbowInstance {
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

    pub(crate) fn crossbow_on_focus_in(_env: JNIEnv) -> Result<()> {
        println!("CrossbowLib_focus_in");
        Ok(())
    }

    pub(crate) fn crossbow_on_focus_out(_env: JNIEnv) -> Result<()> {
        println!("CrossbowLib_focus_out");
        Ok(())
    }
}
