use jni::{
    errors::*,
    objects::{GlobalRef, JMethodID, JObject, JValue},
    signature::TypeSignature,
    sys::jmethodID,
    JNIEnv,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct JniSingleton {
    instance: GlobalRef,
    methods: HashMap<String, JniSingletonMethod>,
}

impl JniSingleton {
    pub fn new(instance: GlobalRef) -> Self {
        Self {
            instance,
            methods: HashMap::new(),
        }
    }

    pub fn get_instance(&self) -> JObject {
        self.instance.as_obj()
    }

    pub fn get_method(&self, name: &str) -> Option<&JniSingletonMethod> {
        self.methods.get(name)
    }

    pub fn get_methods(&self) -> &HashMap<String, JniSingletonMethod> {
        &self.methods
    }

    pub fn add_method(&mut self, name: &str, method_id: jmethodID, signature: TypeSignature) {
        self.methods.insert(
            name.to_owned(),
            JniSingletonMethod {
                method_id,
                signature,
            },
        );
    }

    pub fn call_method<'a>(
        &'a self,
        env: &'a JNIEnv,
        name: &str,
        args: &[JValue],
    ) -> Result<JValue<'a>> {
        let method = match self.get_method(name) {
            Some(method) => method,
            None => Err(Error::MethodNotFound {
                name: name.to_owned(),
                sig: "".to_owned(),
            })?,
        };
        let mid: JMethodID = method.method_id.into();
        let result = env.call_method_unchecked(
            self.get_instance(),
            mid,
            method.signature.ret.clone(),
            args,
        )?;
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct JniSingletonMethod {
    method_id: jmethodID,
    signature: TypeSignature,
}
