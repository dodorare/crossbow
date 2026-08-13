use super::JniRustType;
use async_channel::Receiver;
use jni::{
    Env,
    errors::*,
    objects::{Global, JObject, JValue, JValueOwned},
    signature::{JavaType, RuntimeMethodSignature},
    strings::JNIString,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct JniSingleton {
    name: String,
    instance: Arc<Global<JObject<'static>>>,
    methods: HashMap<String, JniSingletonMethod>,
    signals: HashMap<String, Vec<JavaType>>,
    receiver: Receiver<Signal>,
}

#[derive(Clone, Debug)]
pub struct Signal {
    pub name: String,
    pub args: Vec<JniRustType>,
}

#[derive(Clone)]
pub struct JniSingletonMethod {
    signature: RuntimeMethodSignature,
}

impl JniSingleton {
    pub fn new(name: &str, instance: Global<JObject<'static>>, receiver: Receiver<Signal>) -> Self {
        Self {
            name: name.to_string(),
            instance: Arc::new(instance),
            methods: HashMap::new(),
            signals: HashMap::new(),
            receiver,
        }
    }

    pub fn get_instance(&self) -> &JObject<'_> {
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

    pub(crate) fn add_method(&mut self, name: &str, signature: RuntimeMethodSignature) {
        self.methods
            .insert(name.to_owned(), JniSingletonMethod { signature });
    }

    pub(crate) fn add_signal_info(&mut self, name: &str, args: Vec<JavaType>) {
        self.signals.insert(name.to_owned(), args);
    }

    pub fn call_method<'local>(
        &self,
        env: &mut Env<'local>,
        name: &str,
        args: &[JValue<'_>],
    ) -> Result<JValueOwned<'local>> {
        let method = match self.get_method(name) {
            Some(method) => method,
            None => Err(Error::MethodNotFound {
                name: name.to_owned(),
                sig: "".to_owned(),
            })?,
        };
        let result = env.call_method(
            self.get_instance(),
            JNIString::new(name),
            method.signature.method_signature(),
            args,
        )?;
        Ok(result)
    }
}
