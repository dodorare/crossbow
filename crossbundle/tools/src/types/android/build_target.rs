use crate::{
    error::{AndroidError, Result},
    types::IntoRustTriple,
};
use serde::{Deserialize, Serialize};

/// Android Target.
///
/// More details: https://doc.rust-lang.org/nightly/rustc/platform-support.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AndroidTarget {
    #[serde(rename = "armv7-linux-androideabi")]
    Armv7,
    #[serde(rename = "aarch64-linux-android")]
    Aarch64,
    #[serde(rename = "i686-linux-android")]
    I686,
    #[serde(rename = "x86_64-linux-android")]
    X8664,
}

impl AndroidTarget {
    /// Identifier used in the NDK to refer to the ABI.
    pub fn android_abi(self) -> &'static str {
        match self {
            Self::Armv7 => "armeabi-v7a",
            Self::Aarch64 => "arm64-v8a",
            Self::I686 => "x86",
            Self::X8664 => "x86_64",
        }
    }

    // Returns the triple NDK provided LLVM.
    pub fn ndk_llvm_triple(self) -> &'static str {
        match self {
            Self::Armv7 => "armv7a-linux-androideabi",
            Self::Aarch64 => "aarch64-linux-android",
            Self::I686 => "i686-linux-android",
            Self::X8664 => "x86_64-linux-android",
        }
    }

    /// Returns the triple used by the non-LLVM parts of the NDK.
    pub fn ndk_triple(self) -> &'static str {
        match self {
            Self::Armv7 => "arm-linux-androideabi",
            Self::Aarch64 => "aarch64-linux-android",
            Self::I686 => "i686-linux-android",
            Self::X8664 => "x86_64-linux-android",
        }
    }

    /// Returns `AndroidTarget` for abi.
    pub fn from_android_abi(abi: &str) -> Result<Self> {
        match abi {
            "armeabi-v7a" => Ok(Self::Armv7),
            "arm64-v8a" => Ok(Self::Aarch64),
            "x86" => Ok(Self::I686),
            "x86_64" => Ok(Self::X8664),
            _ => Err(AndroidError::UnsupportedTarget.into()),
        }
    }

    // Returns just the architecture component for clang
    pub fn clang_arch(self) -> &'static str {
        match self {
            Self::Armv7 => "arm",
            Self::Aarch64 => "aarch64",
            Self::I686 => "i386",
            Self::X8664 => "x86_64",
        }
    }
}

impl Default for AndroidTarget {
    fn default() -> Self {
        Self::Aarch64
    }
}

impl std::str::FromStr for AndroidTarget {
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "armv7-linux-androideabi" => Ok(Self::Armv7),
            "aarch64-linux-android" => Ok(Self::Aarch64),
            "i686-linux-android" => Ok(Self::I686),
            "x86_64-linux-android" => Ok(Self::X8664),
            _ => Err(AndroidError::InvalidBuildTarget(s.to_owned())),
        }
    }
}

impl IntoRustTriple for AndroidTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::Armv7 => "armv7-linux-androideabi",
            Self::Aarch64 => "aarch64-linux-android",
            Self::I686 => "i686-linux-android",
            Self::X8664 => "x86_64-linux-android",
        }
    }
}
