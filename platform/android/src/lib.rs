#[cfg(feature = "embed")]
pub mod embed;

#[cfg(feature = "android")]
pub(crate) mod externs;
#[cfg(feature = "android")]
pub(crate) mod utils;

#[cfg(feature = "android")]
mod crossbow;
#[cfg(feature = "android")]
pub mod error;
#[cfg(feature = "android")]
pub mod permission;
#[cfg(feature = "android")]
pub mod plugin;

#[cfg(feature = "android")]
pub use crossbow::*;
#[cfg(feature = "android")]
pub use jni;
#[cfg(feature = "android")]
pub use plugin::CrossbowPlugin;

/// Get java VM for executing Java calls
#[cfg(feature = "android")]
pub fn get_java_vm() -> error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}
