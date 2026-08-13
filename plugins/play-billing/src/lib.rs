use crossbow_android::{
    error::*,
    jni::{JavaVM, objects::JObjectArray, objects::JString},
    plugin::*,
};
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
        self.vm.attach_current_thread(|env| {
            self.singleton.call_method(env, "startConnection", &[])?;
            Ok(())
        })
    }

    pub fn end_connection(&self) -> Result<()> {
        self.vm.attach_current_thread(|env| {
            self.singleton.call_method(env, "endConnection", &[])?;
            Ok(())
        })
    }

    pub fn is_ready(&self) -> Result<bool> {
        self.vm.attach_current_thread(|env| {
            let res = self.singleton.call_method(env, "isReady", &[])?;
            Ok(res.z()?)
        })
    }

    pub fn get_connection_state(&self) -> Result<i32> {
        self.vm.attach_current_thread(|env| {
            let res = self.singleton.call_method(env, "getConnectionState", &[])?;
            Ok(res.i()?)
        })
    }

    pub fn query_purchases<S>(&self, purchase_type: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let purchase_type_str = JString::from_str(env, purchase_type)?;
            self.singleton
                .call_method(env, "queryPurchases", &[(&purchase_type_str).into()])?;
            Ok(())
        })
    }

    pub fn query_product_details<S>(&self, product_ids: &[S], product_type: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let empty_str = JString::from_str(env, "")?;
            let string_array = JObjectArray::<JString>::new(env, product_ids.len(), &empty_str)?;
            for (index, id) in product_ids.iter().enumerate() {
                let id_str = JString::from_str(env, id)?;
                string_array.set_element(env, index, &id_str)?;
            }
            let product_type_str = JString::from_str(env, product_type)?;
            self.singleton.call_method(
                env,
                "queryProductDetails",
                &[(&string_array).into(), (&product_type_str).into()],
            )?;
            Ok(())
        })
    }

    #[deprecated(note = "use query_product_details; Play Billing now uses ProductDetails")]
    pub fn query_sku_details<S>(&self, sku_list: &[S], sku_type: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.query_product_details(sku_list, sku_type)
    }

    pub fn acknowledge_purchase<S>(&self, purchase_token: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let purchase_token_str = JString::from_str(env, purchase_token)?;
            self.singleton.call_method(
                env,
                "acknowledgePurchase",
                &[(&purchase_token_str).into()],
            )?;
            Ok(())
        })
    }

    pub fn consume_purchase<S>(&self, purchase_token: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let purchase_token_str = JString::from_str(env, purchase_token)?;
            self.singleton
                .call_method(env, "consumePurchase", &[(&purchase_token_str).into()])?;
            Ok(())
        })
    }

    #[deprecated(note = "price change confirmation was removed in Play Billing 9")]
    pub fn confirm_price_change<S>(&self, sku: S) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let sku_str = JString::from_str(env, sku)?;
            let res =
                self.singleton
                    .call_method(env, "confirmPriceChange", &[(&sku_str).into()])?;
            JniRustType::from_jobject(env, res.l()?)
        })
    }

    pub fn purchase_with_offer<S>(&self, product_id: S, offer_token: S) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let product_id = JString::from_str(env, product_id)?;
            let offer_token = JString::from_str(env, offer_token)?;
            let res = self.singleton.call_method(
                env,
                "purchaseWithOffer",
                &[(&product_id).into(), (&offer_token).into()],
            )?;
            JniRustType::from_jobject(env, res.l()?)
        })
    }

    pub fn purchase<S>(&self, sku: S) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let sku_str = JString::from_str(env, sku)?;
            let res = self
                .singleton
                .call_method(env, "purchase", &[(&sku_str).into()])?;
            JniRustType::from_jobject(env, res.l()?)
        })
    }

    #[deprecated(note = "use replace_subscription with the old product ID")]
    pub fn update_subscription<S>(
        &self,
        old_token: S,
        sku: S,
        proration_mode: i32,
    ) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let old_token_str = JString::from_str(env, old_token)?;
            let sku_str = JString::from_str(env, sku)?;
            let res = self.singleton.call_method(
                env,
                "updateSubscription",
                &[
                    (&old_token_str).into(),
                    (&sku_str).into(),
                    proration_mode.into(),
                ],
            )?;
            JniRustType::from_jobject(env, res.l()?)
        })
    }

    #[deprecated(note = "use replace_subscription with the old product ID")]
    pub fn update_subscription_with_offer<S>(
        &self,
        old_token: S,
        product_id: S,
        offer_token: S,
        replacement_mode: i32,
    ) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let old_token = JString::from_str(env, old_token)?;
            let product_id = JString::from_str(env, product_id)?;
            let offer_token = JString::from_str(env, offer_token)?;
            let res = self.singleton.call_method(
                env,
                "updateSubscriptionWithOffer",
                &[
                    (&old_token).into(),
                    (&product_id).into(),
                    (&offer_token).into(),
                    replacement_mode.into(),
                ],
            )?;
            JniRustType::from_jobject(env, res.l()?)
        })
    }

    pub fn replace_subscription<S>(
        &self,
        old_product_id: S,
        new_product_id: S,
        offer_token: S,
        replacement_mode: i32,
    ) -> Result<JniRustType>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let old_product_id = JString::from_str(env, old_product_id)?;
            let new_product_id = JString::from_str(env, new_product_id)?;
            let offer_token = JString::from_str(env, offer_token)?;
            let res = self.singleton.call_method(
                env,
                "replaceSubscription",
                &[
                    (&old_product_id).into(),
                    (&new_product_id).into(),
                    (&offer_token).into(),
                    replacement_mode.into(),
                ],
            )?;
            JniRustType::from_jobject(env, res.l()?)
        })
    }

    pub fn set_obfuscated_account_id<S>(&self, account_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let account_id_str = JString::from_str(env, account_id)?;
            self.singleton.call_method(
                env,
                "setObfuscatedAccountId",
                &[(&account_id_str).into()],
            )?;
            Ok(())
        })
    }

    pub fn set_obfuscated_profile_id<S>(&self, profile_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let profile_id_str = JString::from_str(env, profile_id)?;
            self.singleton.call_method(
                env,
                "setObfuscatedProfileId",
                &[(&profile_id_str).into()],
            )?;
            Ok(())
        })
    }
}
