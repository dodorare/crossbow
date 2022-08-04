use crate::{error::AppleError, types::IntoRustTriple};
use serde::{Deserialize, Serialize};

/// iOS Target.
///
/// More details: https://doc.rust-lang.org/nightly/rustc/platform-support.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum IosTarget {
    #[serde(rename = "x86_64-apple-ios")]
    X86_64,
    #[serde(rename = "i386-apple-ios")]
    I386,
    #[serde(rename = "aarch64-apple-ios")]
    Aarch64,
    #[serde(rename = "aarch64-apple-ios-sim")]
    Aarch64Sim,
    #[serde(rename = "armv7-apple-ios")]
    Armv7,
    #[serde(rename = "armv7s-apple-ios")]
    Armv7s,
}

impl IntoRustTriple for IosTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64-apple-ios",
            Self::I386 => "i386-apple-ios",
            Self::Aarch64 => "aarch64-apple-ios",
            Self::Aarch64Sim => "aarch64-apple-ios-sim",
            Self::Armv7 => "armv7-apple-ios",
            Self::Armv7s => "armv7s-apple-ios",
        }
    }
}

impl std::str::FromStr for IosTarget {
    type Err = AppleError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "x86_64-apple-ios" => Ok(Self::X86_64),
            "i386-apple-ios" => Ok(Self::I386),
            "aarch64-apple-ios" => Ok(Self::Aarch64),
            "aarch64-apple-ios-sim" => Ok(Self::Aarch64Sim),
            "armv7-apple-ios" => Ok(Self::Armv7),
            "armv7s-apple-ios" => Ok(Self::Armv7s),
            _ => Err(AppleError::InvalidBuildTarget(s.to_owned())),
        }
    }
}
