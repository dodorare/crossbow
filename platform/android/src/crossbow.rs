use crate::{
    error::*, get_java_vm, permission::on_request_permission_result, plugin::CrossbowPlugin,
    utils::jstring_to_string,
};
use jni::{
    Env, jni_sig, jni_str,
    objects::{JObject, JString},
    sys::{JNI_TRUE, jboolean},
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

    pub(crate) fn crossbow_on_initialize(env: &mut Env, crossbow_instance: &JObject) -> Result<()> {
        env.call_method(
            crossbow_instance,
            jni_str!("onRenderInit"),
            jni_sig!("()V"),
            &[],
        )?;

        Ok(())
    }

    pub(crate) fn on_request_permission_result(
        env: &mut Env,
        permission: &JString,
        result: jboolean,
    ) -> Result<()> {
        let permission = jstring_to_string(env, permission)?;
        on_request_permission_result(permission, result == JNI_TRUE)?;
        Ok(())
    }
}

impl Default for CrossbowInstance {
    fn default() -> Self {
        Self::new()
    }
}
