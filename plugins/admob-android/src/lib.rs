use crossbow_android::{
    error::*,
    jni::{JavaVM, objects::JString},
    plugin::*,
};
use std::sync::Arc;

pub struct AdMobPlugin {
    singleton: Arc<JniSingleton>,
    vm: Arc<JavaVM>,
}

impl CrossbowPlugin for AdMobPlugin {
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
        "CrossbowAdMob"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl AdMobPlugin {
    fn call_void(&self, method: &str) -> Result<()> {
        self.vm.attach_current_thread(|env| {
            self.singleton.call_method(env, method, &[])?;
            Ok(())
        })
    }

    fn call_bool(&self, method: &str) -> Result<bool> {
        self.vm.attach_current_thread(|env| {
            let value = self.singleton.call_method(env, method, &[])?;
            Ok(value.z()?)
        })
    }

    fn call_int(&self, method: &str) -> Result<i32> {
        self.vm.attach_current_thread(|env| {
            let value = self.singleton.call_method(env, method, &[])?;
            Ok(value.i()?)
        })
    }

    // TODO: Make async API
    // pub async fn initialize_async<S>(
    //     &self,
    //     is_for_child_directed_treatment: bool,
    //     max_ad_content_rating: S,
    //     is_real: bool,
    //     is_test_europe_user_consent: bool,
    // ) -> Result<()>
    // where
    //     S: AsRef<str>,
    // {
    //     self.initialize(
    //         is_for_child_directed_treatment,
    //         max_ad_content_rating,
    //         is_real,
    //         is_test_europe_user_consent,
    //     )?;
    //     // loop {
    //     //     self.get_receiver().recv().await?;
    //     // }
    //     Ok(())
    // }

    // TODO: Fix initialization_complete Signal not being sent
    pub fn initialize<S>(
        &self,
        is_for_child_directed_treatment: bool,
        max_ad_content_rating: S,
        is_real: bool,
        is_test_europe_user_consent: bool,
    ) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let rating = JString::from_str(env, max_ad_content_rating)?;
            self.singleton.call_method(
                env,
                "initialize",
                &[
                    is_for_child_directed_treatment.into(),
                    (&rating).into(),
                    is_real.into(),
                    is_test_europe_user_consent.into(),
                ],
            )?;
            Ok(())
        })
    }

    pub fn is_initialized(&self) -> Result<bool> {
        self.call_bool("getIsInitialized")
    }

    pub fn load_interstitial(&self, ad_id: &str) -> Result<()> {
        self.vm.attach_current_thread(|env| {
            let ad_id = JString::from_str(env, ad_id)?;
            self.singleton
                .call_method(env, "loadInterstitial", &[(&ad_id).into()])?;
            Ok(())
        })
    }

    pub fn is_interstitial_loaded(&self) -> Result<bool> {
        self.call_bool("getIsInterstitialLoaded")
    }

    pub fn show_interstitial(&self) -> Result<()> {
        self.call_void("showInterstitial")
    }

    pub fn request_user_consent(&self) -> Result<()> {
        self.call_void("requestUserConsent")
    }

    pub fn reset_consent_state(&self) -> Result<()> {
        self.call_void("resetConsentState")
    }

    pub fn load_banner<S>(
        &self,
        ad_unit_id: S,
        position: i32,
        size: BannerSize,
        show_instantly: bool,
        respect_safe_area: bool,
    ) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let ad_unit_id = JString::from_str(env, ad_unit_id)?;
            let size = JString::from_str(env, size.to_string())?;
            self.singleton.call_method(
                env,
                "loadBanner",
                &[
                    (&ad_unit_id).into(),
                    position.into(),
                    (&size).into(),
                    show_instantly.into(),
                    respect_safe_area.into(),
                ],
            )?;
            Ok(())
        })
    }

    pub fn is_banner_loaded(&self) -> Result<bool> {
        self.call_bool("getIsBannerLoaded")
    }

    pub fn destroy_banner(&self) -> Result<()> {
        self.call_void("destroyBanner")
    }

    pub fn show_banner(&self) -> Result<()> {
        self.call_void("showBanner")
    }

    pub fn hide_banner(&self) -> Result<()> {
        self.call_void("hideBanner")
    }

    pub fn banner_width(&self) -> Result<i32> {
        self.call_int("getBannerWidth")
    }

    pub fn banner_height(&self) -> Result<i32> {
        self.call_int("getBannerHeight")
    }

    pub fn banner_width_in_pixels(&self) -> Result<i32> {
        self.call_int("getBannerWidthInPixels")
    }

    pub fn banner_height_in_pixels(&self) -> Result<i32> {
        self.call_int("getBannerHeightInPixels")
    }

    pub fn load_rewarded<S>(&self, ad_unit_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let ad_unit_id = JString::from_str(env, ad_unit_id)?;
            self.singleton
                .call_method(env, "loadRewarded", &[(&ad_unit_id).into()])?;
            Ok(())
        })
    }

    pub fn is_rewarded_loaded(&self) -> Result<bool> {
        self.call_bool("getIsRewardedLoaded")
    }

    pub fn show_rewarded(&self) -> Result<()> {
        self.call_void("showRewarded")
    }

    pub fn load_rewarded_interstitial<S>(&self, ad_unit_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let ad_unit_id = JString::from_str(env, ad_unit_id)?;
            self.singleton
                .call_method(env, "loadRewardedInterstitial", &[(&ad_unit_id).into()])?;
            Ok(())
        })
    }

    pub fn is_rewarded_interstitial_loaded(&self) -> Result<bool> {
        self.call_bool("getIsRewardedInterstitialLoaded")
    }

    pub fn show_rewarded_interstitial(&self) -> Result<()> {
        self.call_void("showRewardedInterstitial")
    }
}

#[derive(Clone, Copy, Default)]
pub enum BannerSize {
    Banner,
    LargeBanner,
    MediumRectangle,
    FullBanner,
    Leaderboard,
    Adaptive,
    #[default]
    SmartBanner,
}

impl std::fmt::Display for BannerSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Banner => "BANNER",
            Self::LargeBanner => "LARGE_BANNER",
            Self::MediumRectangle => "MEDIUM_RECTANGLE",
            Self::FullBanner => "FULL_BANNER",
            Self::Leaderboard => "LEADERBOARD",
            Self::Adaptive => "ADAPTIVE",
            Self::SmartBanner => "SMART_BANNER",
        })
    }
}
