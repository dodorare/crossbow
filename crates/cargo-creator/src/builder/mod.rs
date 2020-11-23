pub mod android;

pub use android::builder::AndroidBuilder;

// Todo: where to check if `rustc` exists?

pub struct CreatorBuilder;

impl CreatorBuilder {
    pub fn android() -> AndroidBuilder {
        AndroidBuilder
    }

    // Todo: add `apple` function
}
