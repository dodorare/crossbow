#[cfg(feature = "embed")]
pub mod embed;

#[cfg(target_os = "android")]
pub(crate) mod externs;
#[cfg(target_os = "android")]
pub(crate) mod utils;

#[cfg(target_os = "android")]
mod crossbow;
#[cfg(target_os = "android")]
pub mod error;
#[cfg(target_os = "android")]
pub mod permission;
#[cfg(target_os = "android")]
pub mod plugin;

#[cfg(target_os = "android")]
pub use crossbow::*;
#[cfg(target_os = "android")]
pub use jni;
#[cfg(target_os = "android")]
pub use plugin::CrossbowPlugin;

/// Get java VM for executing Java calls
#[cfg(target_os = "android")]
pub fn get_java_vm() -> error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}
