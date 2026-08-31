use serde::{Deserialize, Serialize};

/// Selects the Android activity and lifecycle integration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AndroidRuntime {
    /// Android's NativeActivity, used by Bevy and other native applications.
    #[default]
    #[serde(rename = "native-activity")]
    NativeActivity,
    /// Android Game Development Kit's GameActivity, used by Bevy and other native applications.
    #[serde(rename = "game-activity")]
    GameActivity,
    /// Miniquad's Java Activity and JNI bridge, used by Macroquad.
    #[serde(rename = "miniquad")]
    Miniquad,
}

impl AndroidRuntime {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeActivity => "native-activity",
            Self::GameActivity => "game-activity",
            Self::Miniquad => "miniquad",
        }
    }

    pub const fn requires_gradle(self) -> bool {
        matches!(self, Self::GameActivity | Self::Miniquad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_activity_is_the_default() {
        assert_eq!(AndroidRuntime::default(), AndroidRuntime::NativeActivity);
        assert_eq!(
            serde_json::from_str::<AndroidRuntime>("\"miniquad\"").unwrap(),
            AndroidRuntime::Miniquad
        );
        assert_eq!(
            serde_json::from_str::<AndroidRuntime>("\"game-activity\"").unwrap(),
            AndroidRuntime::GameActivity
        );
        assert!(AndroidRuntime::GameActivity.requires_gradle());
        assert!(!AndroidRuntime::NativeActivity.requires_gradle());
    }
}
