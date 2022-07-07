pub mod android_config;
pub mod apple_config;

pub use android_config::*;
pub use apple_config::*;

pub const MIN_SDK_VERSION: u32 = 21;
