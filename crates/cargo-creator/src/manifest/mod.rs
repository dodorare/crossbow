mod android;
mod apple;

pub use android::*;
pub use apple::*;

use serde::{Deserialize, Serialize};

pub type Manifest = cargo_toml::Manifest<Metadata>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub android: Option<AndroidMetadata>,
    pub apple: Option<AppleMetadata>,
}
