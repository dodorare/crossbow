use crate::error::{AndroidError, Result};
use serde::{Deserialize, Serialize};

pub trait IntoRustTriple {
    /// Returns the triple used by the rust build tools
    fn rust_triple(&self) -> &'static str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AndroidTarget {
    Armv7LinuxAndroideabi,
    Aarch64LinuxAndroid,
    I686LinuxAndroid,
    X8664LinuxAndroid,
}

impl AndroidTarget {
    /// Identifier used in the NDK to refer to the ABI
    pub fn android_abi(self) -> &'static str {
        match self {
            Self::Armv7LinuxAndroideabi => "armeabi-v7a",
            Self::Aarch64LinuxAndroid => "arm64-v8a",
            Self::I686LinuxAndroid => "x86",
            Self::X8664LinuxAndroid => "x86_64",
        }
    }

    // Returns the triple NDK provided LLVM
    pub fn ndk_llvm_triple(self) -> &'static str {
        match self {
            Self::Armv7LinuxAndroideabi => "armv7a-linux-androideabi",
            Self::Aarch64LinuxAndroid => "aarch64-linux-android",
            Self::I686LinuxAndroid => "i686-linux-android",
            Self::X8664LinuxAndroid => "x86_64-linux-android",
        }
    }

    /// Returns the triple used by the non-LLVM parts of the NDK
    pub fn ndk_triple(self) -> &'static str {
        match self {
            Self::Armv7LinuxAndroideabi => "arm-linux-androideabi",
            Self::Aarch64LinuxAndroid => "aarch64-linux-android",
            Self::I686LinuxAndroid => "i686-linux-android",
            Self::X8664LinuxAndroid => "x86_64-linux-android",
        }
    }

    /// Returns `AndroidTarget` for abi.
    pub fn from_android_abi(abi: &str) -> Result<Self> {
        match abi {
            "armeabi-v7a" => Ok(Self::Armv7LinuxAndroideabi),
            "arm64-v8a" => Ok(Self::Aarch64LinuxAndroid),
            "x86" => Ok(Self::I686LinuxAndroid),
            "x86_64" => Ok(Self::X8664LinuxAndroid),
            _ => Err(AndroidError::UnsupportedTarget.into()),
        }
    }
}

impl std::str::FromStr for AndroidTarget {
    type Err = AndroidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "armv7-linux-androideabi" => Ok(Self::Armv7LinuxAndroideabi),
            "aarch64-linux-android" => Ok(Self::Aarch64LinuxAndroid),
            "i686-linux-android" => Ok(Self::I686LinuxAndroid),
            "x86_64-linux-android" => Ok(Self::X8664LinuxAndroid),
            _ => Err(AndroidError::InvalidBuildTarget(s.to_owned())),
        }
    }
}

impl IntoRustTriple for AndroidTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::Armv7LinuxAndroideabi => "armv7-linux-androideabi",
            Self::Aarch64LinuxAndroid => "aarch64-linux-android",
            Self::I686LinuxAndroid => "i686-linux-android",
            Self::X8664LinuxAndroid => "x86_64-linux-android",
        }
    }
}

/// Apple Target architectures.
/// List of Apple processors: https://en.wikipedia.org/wiki/Apple-designed_processors#List_of_Apple_processors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AppleTarget {
    #[serde(rename = "x86_64-apple-ios")]
    X86_64AppleIos,
    #[serde(rename = "i386-apple-ios")]
    I386AppleIos,
    #[serde(rename = "aarch64-apple-ios")]
    Aarch64AppleIos,
    #[serde(rename = "armv7-apple-ios")]
    Armv7AppleIos,
    #[serde(rename = "armv7s-apple-ios")]
    Armv7sAppleIos,
}

impl IntoRustTriple for AppleTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::X86_64AppleIos => "x86_64-apple-ios",
            Self::I386AppleIos => "i386-apple-ios",
            Self::Aarch64AppleIos => "aarch64-apple-ios",
            Self::Armv7AppleIos => "armv7-apple-ios",
            Self::Armv7sAppleIos => "armv7s-apple-ios",
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuildTarget {
    Android(AndroidTarget),
    Apple(AppleTarget),
}

impl IntoRustTriple for BuildTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::Android(target) => target.rust_triple(),
            Self::Apple(target) => target.rust_triple(),
        }
    }
}

impl From<AppleTarget> for BuildTarget {
    fn from(target: AppleTarget) -> Self {
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
    Apple(Vec<AppleTarget>),
}

impl From<Vec<AndroidTarget>> for BuildTargets {
    fn from(targets: Vec<AndroidTarget>) -> Self {
        Self::Android(targets)
    }
}

impl From<Vec<AppleTarget>> for BuildTargets {
    fn from(targets: Vec<AppleTarget>) -> Self {
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
