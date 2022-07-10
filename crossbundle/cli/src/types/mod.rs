pub mod android_config;
pub mod apple_config;

pub use android_config::*;
pub use apple_config::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub android: AndroidConfig,
    pub apple: AppleConfig,
}
