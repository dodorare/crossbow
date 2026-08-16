use crate::{error::AppleError, types::IntoRustTriple};
use serde::{Deserialize, Serialize};

/// iOS Target.
///
/// More details: https://doc.rust-lang.org/nightly/rustc/platform-support.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum IosTarget {
    #[serde(rename = "x86_64-apple-ios")]
    X86_64Sim,
    #[serde(rename = "aarch64-apple-ios")]
    Aarch64Device,
    #[serde(rename = "aarch64-apple-ios-sim")]
    Aarch64Sim,
}

impl IosTarget {
    /// Simulator target matching the host architecture.
    pub const fn host_simulator() -> Self {
        if cfg!(target_arch = "aarch64") {
            Self::Aarch64Sim
        } else {
            Self::X86_64Sim
        }
    }

    pub const fn is_simulator(self) -> bool {
        matches!(self, Self::X86_64Sim | Self::Aarch64Sim)
    }
}

impl IntoRustTriple for IosTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::X86_64Sim => "x86_64-apple-ios",
            Self::Aarch64Device => "aarch64-apple-ios",
            Self::Aarch64Sim => "aarch64-apple-ios-sim",
        }
    }
}

impl std::str::FromStr for IosTarget {
    type Err = AppleError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "x86_64-apple-ios" => Ok(Self::X86_64Sim),
            "aarch64-apple-ios" => Ok(Self::Aarch64Device),
            "aarch64-apple-ios-sim" => Ok(Self::Aarch64Sim),
            _ => Err(AppleError::InvalidBuildTarget(s.to_owned())),
        }
    }
}
