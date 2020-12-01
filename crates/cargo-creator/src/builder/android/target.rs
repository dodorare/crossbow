use super::error::NdkError;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[repr(u8)]
pub enum AndroidTarget {
    #[serde(rename = "armv7-linux-androideabi")]
    ArmV7a = 1,
    #[serde(rename = "aarch64-linux-android")]
    Arm64V8a = 2,
    #[serde(rename = "i686-linux-android")]
    X86 = 3,
    #[serde(rename = "x86_64-linux-android")]
    X86_64 = 4,
}

impl AndroidTarget {
    /// Identifier used in the NDK to refer to the ABI
    pub fn android_abi(self) -> &'static str {
        match self {
            Self::Arm64V8a => "arm64-v8a",
            Self::ArmV7a => "armeabi-v7a",
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
        }
    }

    /// Returns `AndroidTarget` for abi.
    pub fn from_android_abi(abi: &str) -> Result<AndroidTarget, NdkError> {
        match abi {
            "arm64-v8a" => Ok(AndroidTarget::Arm64V8a),
            "armeabi-v7a" => Ok(AndroidTarget::ArmV7a),
            "x86" => Ok(AndroidTarget::X86),
            "x86_64" => Ok(AndroidTarget::X86_64),
            _ => Err(NdkError::UnsupportedTarget),
        }
    }

    /// Returns the triple used by the rust build tools
    pub fn rust_triple(self) -> &'static str {
        match self {
            AndroidTarget::Arm64V8a => "aarch64-linux-android",
            AndroidTarget::ArmV7a => "armv7-linux-androideabi",
            AndroidTarget::X86 => "i686-linux-android",
            AndroidTarget::X86_64 => "x86_64-linux-android",
        }
    }

    /// Returns `AndroidTarget` for rust triple.
    pub fn from_rust_triple(triple: &str) -> Result<AndroidTarget, NdkError> {
        match triple {
            "aarch64-linux-android" => Ok(AndroidTarget::Arm64V8a),
            "armv7-linux-androideabi" => Ok(AndroidTarget::ArmV7a),
            "i686-linux-android" => Ok(AndroidTarget::X86),
            "x86_64-linux-android" => Ok(AndroidTarget::X86_64),
            _ => Err(NdkError::UnsupportedTarget),
        }
    }

    // Returns the triple NDK provided LLVM
    pub fn ndk_llvm_triple(self) -> &'static str {
        match self {
            AndroidTarget::Arm64V8a => "aarch64-linux-android",
            AndroidTarget::ArmV7a => "armv7a-linux-androideabi",
            AndroidTarget::X86 => "i686-linux-android",
            AndroidTarget::X86_64 => "x86_64-linux-android",
        }
    }

    /// Returns the triple used by the non-LLVM parts of the NDK
    pub fn ndk_triple(self) -> &'static str {
        match self {
            AndroidTarget::Arm64V8a => "aarch64-linux-android",
            AndroidTarget::ArmV7a => "arm-linux-androideabi",
            AndroidTarget::X86 => "i686-linux-android",
            AndroidTarget::X86_64 => "x86_64-linux-android",
        }
    }
}
