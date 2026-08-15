use crate::error::AndroidError;
use serde::{Deserialize, Serialize};

/// Selects how Rust code is compiled for Android.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AndroidRustCompiler {
    #[default]
    #[serde(rename = "cargo")]
    Cargo,
    #[serde(rename = "ndk-glue")]
    NdkGlue,
    #[serde(rename = "quad")]
    Quad,
}

impl std::str::FromStr for AndroidRustCompiler {
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(Self::Cargo),
            "ndk-glue" => Ok(Self::NdkGlue),
            "quad" => Ok(Self::Quad),
            _ => Err(AndroidError::InvalidRustCompiler(s.to_owned())),
        }
    }
}

#[deprecated(since = "0.2.4", note = "use AndroidRustCompiler instead")]
pub type AppWrapper = AndroidRustCompiler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_is_the_default_compilation_path() {
        assert_eq!(AndroidRustCompiler::default(), AndroidRustCompiler::Cargo);
        assert_eq!(
            serde_json::to_string(&AndroidRustCompiler::Cargo).unwrap(),
            "\"cargo\""
        );
    }

    #[test]
    fn legacy_wrappers_remain_explicitly_configurable() {
        assert_eq!(
            "ndk-glue".parse::<AndroidRustCompiler>().unwrap(),
            AndroidRustCompiler::NdkGlue
        );
        assert_eq!(
            "quad".parse::<AndroidRustCompiler>().unwrap(),
            AndroidRustCompiler::Quad
        );
        assert_eq!(
            serde_json::from_str::<AndroidRustCompiler>("\"quad\"").unwrap(),
            AndroidRustCompiler::Quad
        );
    }
}
