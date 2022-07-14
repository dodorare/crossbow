#[cfg(target_os = "android")]
pub use ndk_glue;

#[cfg(feature = "android")]
pub use crossbow_android as android;

mod permission;
pub use permission::*;
