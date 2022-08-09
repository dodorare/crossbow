use crossbow_android::{error::*, jni::JavaVM, plugin::*};
use std::sync::Arc;

pub struct PlayGamesServicesPlugin {
    singleton: Arc<JniSingleton>,
    vm: Arc<JavaVM>,
}

impl CrossbowPlugin for PlayGamesServicesPlugin {
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
        "PlayGamesServices"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl PlayGamesServicesPlugin {
    pub fn get_receiver(&self) -> Result<()> {
        self.vm.attach_current_thread_as_daemon()?;
        Ok(())
    }
}
