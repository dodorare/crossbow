pub mod android;
pub mod shared;

pub use android::builder::*;
use shared::build_config::BuildConfig;
pub use shared::cargo_manifest::*;

pub struct CreatorBuilder;

impl CreatorBuilder {
    pub fn android() -> AndroidBuilder {
        AndroidBuilder
    }
}
