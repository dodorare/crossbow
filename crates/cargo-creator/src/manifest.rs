use creator_tools::types::AndroidMetadata;
use serde::{Deserialize, Serialize};

pub type Manifest = cargo_toml::Manifest<Metadata>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub android: AndroidMetadata,
    // pub apple: AppleMetadata,
}
