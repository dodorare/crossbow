use crossbow_android::{error::*, jni::JNIEnv, types::*};
use std::sync::Arc;

pub struct AdMobPlugin<'a> {
    singleton: Arc<JniSingleton>,
    jnienv: JNIEnv<'a>,
}

impl<'a> AdMobPlugin<'a> {
    pub fn from_jnienv(singleton: Arc<JniSingleton>, jnienv: JNIEnv<'a>) -> Result<Self> {
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

    pub fn get_is_initialized(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_initialized", &[])?;
        Ok(val.z()?)
    }

    pub fn load_interstitial(&self, ad_id: &str) -> Result<()> {
        let ad_id = self.jnienv.new_string(ad_id.to_string())?;
        self.singleton
            .call_method(&self.jnienv, "load_interstitial", &[ad_id.into()])?;
        Ok(())
    }

    pub fn get_is_interstitial_loaded(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_interstitial_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn show_interstitial(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "show_interstitial", &[])?;
        Ok(())
    }

    pub fn request_user_consent(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "request_user_consent", &[])?;
        Ok(())
    }

    pub fn reset_consent_state(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "reset_consent_state", &[])?;
        Ok(())
    }

    pub fn load_banner(
        &self,
        ad_unit_id: &str,
        position: i32,
        size: BannerSize,
        show_instantly: bool,
        respect_safe_area: bool,
    ) -> Result<()> {
        let ad_unit_id = self.jnienv.new_string(ad_unit_id.to_string())?;
        let size = self.jnienv.new_string(size.to_string())?;
        self.singleton.call_method(
            &self.jnienv,
            "load_banner",
            &[
                ad_unit_id.into(),
                position.into(),
                size.into(),
                show_instantly.into(),
                respect_safe_area.into(),
            ],
        )?;
        Ok(())
    }

    pub fn get_is_banner_loaded(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_banner_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn destroy_banner(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "destroy_banner", &[])?;
        Ok(())
    }

    pub fn show_banner(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "show_banner", &[])?;
        Ok(())
    }

    pub fn hide_banner(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "hide_banner", &[])?;
        Ok(())
    }

    pub fn get_banner_width(&self) -> Result<i32> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_banner_width", &[])?;
        Ok(val.i()?)
    }

    pub fn get_banner_height(&self) -> Result<i32> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_banner_height", &[])?;
        Ok(val.i()?)
    }

    pub fn get_banner_width_in_pixels(&self) -> Result<i32> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_banner_width_in_pixels", &[])?;
        Ok(val.i()?)
    }

    pub fn get_banner_height_in_pixels(&self) -> Result<i32> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_banner_height_in_pixels", &[])?;
        Ok(val.i()?)
    }

    pub fn load_rewarded(&self, ad_unit_id: &str) -> Result<()> {
        let ad_unit_id = self.jnienv.new_string(ad_unit_id.to_string())?;
        self.singleton
            .call_method(&self.jnienv, "load_rewarded", &[ad_unit_id.into()])?;
        Ok(())
    }

    pub fn get_is_rewarded_loaded(&self) -> Result<bool> {
        let val = self
            .singleton
            .call_method(&self.jnienv, "get_is_rewarded_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn show_rewarded(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "show_rewarded", &[])?;
        Ok(())
    }

    pub fn load_rewarded_interstitial(&self, ad_unit_id: &str) -> Result<()> {
        let ad_unit_id = self.jnienv.new_string(ad_unit_id.to_string())?;
        self.singleton.call_method(
            &self.jnienv,
            "load_rewarded_interstitial",
            &[ad_unit_id.into()],
        )?;
        Ok(())
    }

    pub fn get_is_rewarded_interstitial_loaded(&self) -> Result<bool> {
        let val =
            self.singleton
                .call_method(&self.jnienv, "get_is_rewarded_interstitial_loaded", &[])?;
        Ok(val.z()?)
    }

    pub fn show_rewarded_interstitial(&self) -> Result<()> {
        self.singleton
            .call_method(&self.jnienv, "show_rewarded_interstitial", &[])?;
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
