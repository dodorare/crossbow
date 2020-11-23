pub mod android;

pub use android::builder::AndroidBuilder;

pub struct CreatorBuilder;

impl CreatorBuilder {
    pub fn android() -> AndroidBuilder {
        AndroidBuilder
    }
}
