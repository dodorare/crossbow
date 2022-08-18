use crossbow_android::{error::*, jni::JavaVM, plugin::*};
use std::sync::Arc;

pub struct PlayCorePlugin {
    singleton: Arc<JniSingleton>,
    vm: Arc<JavaVM>,
}

impl CrossbowPlugin for PlayCorePlugin {
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
        "CrossbowPlayCore"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl PlayCorePlugin {
    pub fn check_update(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton.call_method(&jnienv, "checkUpdate", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn in_progress_update(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "inProgressUpdate", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }
}
