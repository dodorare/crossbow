#[cfg(all(target_os = "android", feature = "android"))]
pub use ndk_glue;

#[cfg(all(target_os = "android", feature = "android"))]
pub use crossbow_android as android;

#[cfg(all(target_os = "ios", feature = "ios"))]
pub use crossbow_ios as ios;

pub mod error;
mod permission;

pub use permission::*;
