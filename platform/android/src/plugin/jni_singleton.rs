use super::JniRustType;
use async_channel::Receiver;
use jni::{
    errors::*,
    objects::{GlobalRef, JClass, JObject, JValue},
    signature::{JavaType, TypeSignature},
    JNIEnv,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct JniSingleton {
    name: String,
    instance: GlobalRef,
    methods: HashMap<String, JniSingletonMethod>,
    signals: HashMap<String, Vec<JavaType>>,
    receiver: Receiver<Signal>,
}

#[derive(Clone, Debug)]
pub struct Signal {
    pub signal_name: String,
    pub args: Vec<JniRustType>,
}

#[derive(Clone)]
pub struct JniSingletonMethod {
    class: GlobalRef,
    signature: TypeSignature,
}

impl JniSingleton {
    pub fn new(name: &str, instance: GlobalRef, receiver: Receiver<Signal>) -> Self {
        Self {
            name: name.to_string(),
            instance,
            methods: HashMap::new(),
            signals: HashMap::new(),
            receiver,
        }
    }

    pub fn get_instance(&self) -> JObject {
        self.instance.as_obj()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_receiver(&self) -> &Receiver<Signal> {
        &self.receiver
    }

    pub fn get_method(&self, name: &str) -> Option<&JniSingletonMethod> {
        self.methods.get(name)
    }

    pub fn get_methods(&self) -> &HashMap<String, JniSingletonMethod> {
        &self.methods
    }

    pub(crate) fn add_method(&mut self, name: &str, class: GlobalRef, signature: TypeSignature) {
        self.methods
            .insert(name.to_owned(), JniSingletonMethod { class, signature });
    }

    pub(crate) fn add_signal_info(&mut self, name: &str, args: Vec<JavaType>) {
        self.signals.insert(name.to_owned(), args);
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
        let class: JClass = method.class.as_obj().into();
        let method_id = env.get_method_id(class, name, method.signature.to_string())?;

        let result = env.call_method_unchecked(
            self.get_instance(),
            method_id,
            method.signature.ret.clone(),
            args,
        )?;
        Ok(result)
    }
}
