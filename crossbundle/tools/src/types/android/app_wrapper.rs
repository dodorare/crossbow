use crate::error::AndroidError;
use serde::{Deserialize, Serialize};

/// Selects standard Cargo compilation or an explicit legacy source wrapper.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AppWrapper {
    #[default]
    #[serde(rename = "cargo")]
    Cargo,
    #[serde(rename = "ndk-glue")]
    NdkGlue,
    #[serde(rename = "quad")]
    Quad,
}

impl std::str::FromStr for AppWrapper {
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(Self::Cargo),
            "ndk-glue" => Ok(Self::NdkGlue),
            "quad" => Ok(Self::Quad),
            _ => Err(AndroidError::InvalidAppWrapper(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_is_the_default_compilation_path() {
        assert_eq!(AppWrapper::default(), AppWrapper::Cargo);
        assert_eq!(
            serde_json::to_string(&AppWrapper::Cargo).unwrap(),
            "\"cargo\""
        );
    }

    #[test]
    fn legacy_wrappers_remain_explicitly_configurable() {
        assert_eq!(
            "ndk-glue".parse::<AppWrapper>().unwrap(),
            AppWrapper::NdkGlue
        );
        assert_eq!("quad".parse::<AppWrapper>().unwrap(), AppWrapper::Quad);
        assert_eq!(
            serde_json::from_str::<AppWrapper>("\"quad\"").unwrap(),
            AppWrapper::Quad
        );
    }
}
