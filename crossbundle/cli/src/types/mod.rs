pub mod android_config;
pub mod apple_config;

pub use android_config::*;
pub use apple_config::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Metadata {
    #[serde(default)]
    pub android: AndroidConfig,
    #[serde(default)]
    pub apple: AppleConfig,
}
