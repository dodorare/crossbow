use crossbow_android::{error::*, jni::JavaVM, plugin::*};
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
        "AdMob"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl AdMobPlugin {
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
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let g_str = jnienv.new_string(max_ad_content_rating)?;
        self.singleton.call_method(
            &jnienv,
            "initialize",
            &[
                is_for_child_directed_treatment.into(),
                g_str.into(),
                is_real.into(),
                is_test_europe_user_consent.into(),
            ],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_initialized(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_is_initialized", &[])?;
        Ok(val.z()?)
    }

    pub fn load_interstitial(&self, ad_id: &str) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let ad_id = jnienv.new_string(ad_id.to_string())?;
        self.singleton
            .call_method(&jnienv, "load_interstitial", &[ad_id.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_interstitial_loaded(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_is_interstitial_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn show_interstitial(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton
            .call_method(&jnienv, "show_interstitial", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn request_user_consent(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton
            .call_method(&jnienv, "request_user_consent", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn reset_consent_state(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton
            .call_method(&jnienv, "reset_consent_state", &[])?;
        jnienv.exception_check()?;
        Ok(())
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
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let ad_unit_id = jnienv.new_string(ad_unit_id)?;
        let size = jnienv.new_string(size.to_string())?;
        self.singleton.call_method(
            &jnienv,
            "load_banner",
            &[
                ad_unit_id.into(),
                position.into(),
                size.into(),
                show_instantly.into(),
                respect_safe_area.into(),
            ],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_banner_loaded(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_is_banner_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn destroy_banner(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton.call_method(&jnienv, "destroy_banner", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn show_banner(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton.call_method(&jnienv, "show_banner", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn hide_banner(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton.call_method(&jnienv, "hide_banner", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn banner_width(&self) -> Result<i32> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_banner_width", &[])?;
        Ok(val.i()?)
    }

    pub fn banner_height(&self) -> Result<i32> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_banner_height", &[])?;
        Ok(val.i()?)
    }

    pub fn banner_width_in_pixels(&self) -> Result<i32> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_banner_width_in_pixels", &[])?;
        Ok(val.i()?)
    }

    pub fn banner_height_in_pixels(&self) -> Result<i32> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_banner_height_in_pixels", &[])?;
        Ok(val.i()?)
    }

    pub fn load_rewarded<S>(&self, ad_unit_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let ad_unit_id = jnienv.new_string(ad_unit_id)?;
        self.singleton
            .call_method(&jnienv, "load_rewarded", &[ad_unit_id.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_rewarded_loaded(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self
            .singleton
            .call_method(&jnienv, "get_is_rewarded_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn show_rewarded(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton.call_method(&jnienv, "show_rewarded", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_rewarded_interstitial<S>(&self, ad_unit_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let ad_unit_id = jnienv.new_string(ad_unit_id)?;
        self.singleton
            .call_method(&jnienv, "load_rewarded_interstitial", &[ad_unit_id.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_rewarded_interstitial_loaded(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val =
            self.singleton
                .call_method(&jnienv, "get_is_rewarded_interstitial_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn show_rewarded_interstitial(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton
            .call_method(&jnienv, "show_rewarded_interstitial", &[])?;
        jnienv.exception_check()?;
        Ok(())
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

impl ToString for BannerSize {
    fn to_string(&self) -> String {
        match self {
            Self::Banner => "BANNER".to_string(),
            Self::LargeBanner => "LARGE_BANNER".to_string(),
            Self::MediumRectangle => "MEDIUM_RECTANGLE".to_string(),
            Self::FullBanner => "FULL_BANNER".to_string(),
            Self::Leaderboard => "LEADERBOARD".to_string(),
            Self::Adaptive => "ADAPTIVE".to_string(),
            Self::SmartBanner => "SMART_BANNER".to_string(),
        }
    }
}
