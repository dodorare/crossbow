use crossbow_android::{error::*, jni::JavaVM, plugin::*};
use std::sync::Arc;

pub struct PlayBillingPlugin {
    singleton: Arc<JniSingleton>,
    vm: Arc<JavaVM>,
}

impl CrossbowPlugin for PlayBillingPlugin {
    fn from_java_vm(vm: Arc<JavaVM>) -> Result<Self>
    where
        Self: Sized,
    {
        let singleton = get_jni_singleton(Self::get_plugin_name()).ok_or_else(|| {
            AndroidError::SingletonNotRegistered(Self::get_plugin_name().to_owned())
        })?;
        Ok(Self { singleton, vm })
    }

    fn get_plugin_name() -> &'static str {
        "CrossbowPlayBilling"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl PlayBillingPlugin {
    pub fn init(&self, enable_popups: bool) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton
            .call_method(&jnienv, "init", &[enable_popups.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }
}
