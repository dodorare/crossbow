use crate::error::{AndroidError, Result};
use serde::{Deserialize, Serialize};

pub trait IntoRustTriple {
    /// Returns the triple used by the rust build tools.
    fn rust_triple(&self) -> &'static str;
}

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
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "x86_64-apple-ios" => Ok(Self::X86_64),
            "i386-apple-ios" => Ok(Self::I386),
            "aarch64-apple-ios" => Ok(Self::Aarch64),
            "aarch64-apple-ios-sim" => Ok(Self::Aarch64Sim),
            "armv7-apple-ios" => Ok(Self::Armv7),
            "armv7s-apple-ios" => Ok(Self::Armv7s),
            _ => Err(AndroidError::InvalidBuildTarget(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuildTarget {
    Android(AndroidTarget),
    Apple(IosTarget),
}

impl IntoRustTriple for BuildTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::Android(target) => target.rust_triple(),
            Self::Apple(target) => target.rust_triple(),
        }
    }
}

impl From<IosTarget> for BuildTarget {
    fn from(target: IosTarget) -> Self {
        Self::Apple(target)
    }
}

impl From<AndroidTarget> for BuildTarget {
    fn from(target: AndroidTarget) -> Self {
        Self::Android(target)
    }
}

#[derive(Debug, Clone)]
pub enum BuildTargets {
    Android(Vec<AndroidTarget>),
    Apple(Vec<IosTarget>),
}

impl From<Vec<AndroidTarget>> for BuildTargets {
    fn from(targets: Vec<AndroidTarget>) -> Self {
        Self::Android(targets)
    }
}

impl From<Vec<IosTarget>> for BuildTargets {
    fn from(targets: Vec<IosTarget>) -> Self {
        Self::Apple(targets)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrateType {
    /// A runnable executable.
    Bin,
    /// A Rust library.
    Lib,
    /// A dynamic Rust library.
    Dylib,
    /// A static system library.
    Staticlib,
    /// A dynamic system library.
    Cdylib,
    /// A "Rust library" file.
    Rlib,
}

impl AsRef<str> for CrateType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Bin => "bin",
            Self::Lib => "lib",
            Self::Dylib => "dylib",
            Self::Staticlib => "staticlib",
            Self::Cdylib => "cdylib",
            Self::Rlib => "rlib",
        }
    }
}
