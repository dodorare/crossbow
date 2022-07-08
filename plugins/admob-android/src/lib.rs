use crossbow_android::{error::*, jni::JNIEnv, types::*};
use std::sync::Arc;

pub struct AdMobPlugin<'a> {
    singleton: Arc<JniSingleton>,
    jnienv: JNIEnv<'a>,
}

impl<'a> AdMobPlugin<'a> {
    pub fn from_singleton_and_jnienv(
        singleton: Arc<JniSingleton>,
        jnienv: JNIEnv<'a>,
    ) -> Result<Self> {
        Ok(Self { singleton, jnienv })
    }

    pub fn initialize(
        &self,
        is_for_child_directed_treatment: bool,
        max_ad_content_rating: &str,
        is_real: bool,
        is_test_europe_user_consent: bool,
    ) -> Result<()> {
        let g_str = self.jnienv.new_string(max_ad_content_rating.to_string())?;
        self.singleton.call_method(
            &self.jnienv,
            "initialize",
            &[
                is_for_child_directed_treatment.into(),
                g_str.into(),
                is_real.into(),
                is_test_europe_user_consent.into(),
            ],
        )?;
        Ok(())
    }

    pub fn load_interstitial(&self, ad_id: &str) -> Result<()> {
        let ad_id = self.jnienv.new_string(ad_id.to_string())?;
        self.singleton
            .call_method(&self.jnienv, "load_interstitial", &[ad_id.into()])?;
        Ok(())
    }

    pub fn show_interstitial(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "show_interstitial", &[])?;
        Ok(())
    }

    pub fn get_is_initialized(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_initialized", &[])?;
        Ok(val.z()?)
    }

    pub fn get_is_banner_loaded(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_banner_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn get_is_interstitial_loaded(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_interstitial_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn get_is_rewarded_loaded(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_rewarded_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn get_is_rewarded_interstitial_loaded(&self) -> Result<bool> {
        let val =
            self.singleton
                .call_method(&self.jnienv, "get_is_rewarded_interstitial_loaded", &[])?;
        Ok(val.z()?)
    }
}
