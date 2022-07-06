use jni::{
    errors::*,
    objects::{GlobalRef, JClass, JObject, JValue},
    signature::{JavaType, TypeSignature},
    JNIEnv,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct JniSingleton {
    instance: GlobalRef,
    methods: HashMap<String, JniSingletonMethod>,
    signals: HashMap<String, Vec<JavaType>>,
}

#[derive(Clone)]
pub struct JniSingletonMethod {
    class: GlobalRef,
    signature: TypeSignature,
}

impl JniSingleton {
    pub fn new(instance: GlobalRef) -> Self {
        Self {
            instance,
            methods: HashMap::new(),
            signals: HashMap::new(),
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

    pub fn add_method(&mut self, name: &str, class: GlobalRef, signature: TypeSignature) {
        self.methods
            .insert(name.to_owned(), JniSingletonMethod { class, signature });
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

    pub fn add_signal(&mut self, name: &str, args: Vec<JavaType>) {
        self.signals.insert(name.to_owned(), args);
    }

    pub fn emit_signal(&mut self, name: &str, args: Vec<JValue>) {
        // self.signals.insert(name.to_owned(), args);
        println!("emit_signal: {}; args: {:?}", name, args);
    }
}
