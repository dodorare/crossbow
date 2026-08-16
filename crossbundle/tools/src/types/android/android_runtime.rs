use serde::{Deserialize, Serialize};

/// Selects the Android activity and lifecycle integration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AndroidRuntime {
    /// Android's NativeActivity, used by Bevy and other native applications.
    #[default]
    #[serde(rename = "native-activity")]
    NativeActivity,
    /// Miniquad's Java Activity and JNI bridge, used by Macroquad.
    #[serde(rename = "miniquad")]
    Miniquad,
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
    }
}
