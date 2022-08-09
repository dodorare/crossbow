mod handlers;
mod jni_rust_type;
mod jni_singleton;

pub use async_channel::{Receiver, Sender};
pub(crate) use handlers::*;
pub use jni_rust_type::*;
pub use jni_singleton::*;

use crate::error::*;
use jni::JavaVM;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub trait CrossbowPlugin {
    fn from_java_vm(vm: Arc<JavaVM>) -> Result<Self>
    where
        Self: Sized;
    fn get_plugin_name() -> &'static str;
    fn get_receiver(&self) -> &Receiver<Signal>;
}

lazy_static::lazy_static! {
    static ref JNI_SINGLETONS: Mutex<HashMap<String, Arc<JniSingleton>>> = Default::default();
    static ref JNI_SIGNAL_SENDERS: Mutex<HashMap<String, Sender<Signal>>> = Default::default();
}

fn insert_jni_singleton(
    singleton_name: &str,
    singleton: JniSingleton,
) -> Option<Arc<JniSingleton>> {
    let mut jni_signletons_guard = JNI_SINGLETONS.lock().unwrap();
    jni_signletons_guard.insert(singleton_name.to_owned(), Arc::new(singleton))
}

pub fn get_jni_singleton(singleton_name: &str) -> Option<Arc<JniSingleton>> {
    let jni_signletons_guard = JNI_SINGLETONS.lock().unwrap();
    jni_signletons_guard.get(singleton_name).cloned()
}

fn insert_sender(singleton_name: &str, sender: Sender<Signal>) -> Option<Sender<Signal>> {
    JNI_SIGNAL_SENDERS
        .lock()
        .unwrap()
        .insert(singleton_name.to_owned(), sender)
}

pub fn get_jni_singleton_with_error(singleton_name: &str) -> Result<Arc<JniSingleton>> {
    if let Some(jni_signleton) = get_jni_singleton(singleton_name) {
        Ok(jni_signleton)
    } else {
        Err(AndroidError::SingletonNotRegistered(
            singleton_name.to_owned(),
        ))
    }
}

pub fn get_sender(singleton_name: &str) -> Result<Sender<Signal>> {
    let jni_signals = JNI_SIGNAL_SENDERS.lock().unwrap();
    let sender = jni_signals
        .get(singleton_name)
        .ok_or_else(|| AndroidError::SignalSenderNotAvailable(singleton_name.to_owned()))?;
    if sender.is_closed() {
        return Err(AndroidError::SignalSenderNotAvailable(
            singleton_name.to_owned(),
        ));
    }
    Ok(sender.clone())
}
