use crate::{
    error::*, get_java_vm, permission::on_request_permission_result, plugin::CrossbowPlugin,
    utils::jstring_to_string,
};
use jni::{
    objects::{JObject, JString},
    sys::{jboolean, JNI_TRUE},
    JNIEnv,
};
use std::sync::Arc;

pub struct CrossbowInstance {
    pub vm: Arc<jni::JavaVM>,
}

impl CrossbowInstance {
    pub fn new() -> Self {
        let (_, vm) = get_java_vm().unwrap();
        Self { vm: Arc::from(vm) }
    }

    pub fn get_plugin<T>(&self) -> Result<T>
    where
        T: CrossbowPlugin,
    {
        T::from_java_vm(self.vm.clone())
    }

    pub(crate) fn crossbow_on_initialize(
        env: JNIEnv,
        activity: JObject,
        crossbow_instance: JObject,
        _asset_manager: JObject,
    ) -> Result<()> {
        println!("CrossbowLib_initialize: {:?}", activity);

        env.call_method(crossbow_instance, "onRenderInit", "()V", &[])?;
        env.exception_check()?;

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

    pub(crate) fn on_request_permission_result(
        env: JNIEnv,
        permission: JString,
        result: jboolean,
    ) -> Result<()> {
        let permission = jstring_to_string(&env, permission)?;
        on_request_permission_result(permission, result == JNI_TRUE)?;
        Ok(())
    }
}
