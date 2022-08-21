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
    pub fn start_connection(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "startConnection", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn end_connection(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton.call_method(&jnienv, "endConnection", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_ready(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let res = self.singleton.call_method(&jnienv, "isReady", &[])?;
        jnienv.exception_check()?;
        Ok(res.z()?)
    }

    pub fn get_connection_state(&self) -> Result<i32> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let res = self
            .singleton
            .call_method(&jnienv, "getConnectionState", &[])?;
        jnienv.exception_check()?;
        Ok(res.i()?)
    }

    pub fn query_purchases<S>(&self, purchase_type: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let purchase_type_str = jnienv.new_string(purchase_type)?;
        self.singleton
            .call_method(&jnienv, "queryPurchases", &[purchase_type_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn query_sku_details<S>(&self, sku_list: &[S], sku_type: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let empty_str = jnienv.new_string("")?;
        let string_array =
            jnienv.new_object_array(sku_list.len() as i32, "java/lang/String", empty_str)?;
        for (index, id) in sku_list.iter().enumerate() {
            let id_str = jnienv.new_string(id)?;
            jnienv.set_object_array_element(string_array, index as i32, id_str)?;
        }
        let sku_type_str = jnienv.new_string(sku_type)?;
        self.singleton.call_method(
            &jnienv,
            "querySkuDetails",
            &[string_array.into(), sku_type_str.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn acknowledge_purchase<S>(&self, purchase_token: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let purchase_token_str = jnienv.new_string(purchase_token)?;
        self.singleton
            .call_method(&jnienv, "acknowledgePurchase", &[purchase_token_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn consume_purchase<S>(&self, purchase_token: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let purchase_token_str = jnienv.new_string(purchase_token)?;
        self.singleton
            .call_method(&jnienv, "consumePurchase", &[purchase_token_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn confirm_price_change<S>(&self, sku: S) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let sku_str = jnienv.new_string(sku)?;
        let res = self
            .singleton
            .call_method(&jnienv, "confirmPriceChange", &[sku_str.into()])?;
        jnienv.exception_check()?;
        Ok(JniRustType::from_jobject(&jnienv, res.l()?)?)
    }

    pub fn purchase<S>(&self, sku: S) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let sku_str = jnienv.new_string(sku)?;
        let res = self
            .singleton
            .call_method(&jnienv, "purchase", &[sku_str.into()])?;
        jnienv.exception_check()?;
        Ok(JniRustType::from_jobject(&jnienv, res.l()?)?)
    }

    pub fn update_subscription<S>(
        &self,
        old_token: S,
        sku: S,
        proration_mode: i32,
    ) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let old_token_str = jnienv.new_string(old_token)?;
        let sku_str = jnienv.new_string(sku)?;
        let res = self.singleton.call_method(
            &jnienv,
            "updateSubscription",
            &[old_token_str.into(), sku_str.into(), proration_mode.into()],
        )?;
        jnienv.exception_check()?;
        Ok(JniRustType::from_jobject(&jnienv, res.l()?)?)
    }

    pub fn set_obfuscated_account_id<S>(&self, account_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let account_id_str = jnienv.new_string(account_id)?;
        self.singleton
            .call_method(&jnienv, "setObfuscatedAccountId", &[account_id_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn set_obfuscated_profile_id<S>(&self, profile_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let profile_id_str = jnienv.new_string(profile_id)?;
        self.singleton
            .call_method(&jnienv, "setObfuscatedProfileId", &[profile_id_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }
}
