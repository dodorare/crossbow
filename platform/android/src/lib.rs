pub(crate) mod externs;
pub(crate) mod utils;

mod crossbow;
pub mod error;
pub mod permission;
pub mod plugin;

pub use crossbow::*;
pub use jni;
pub use plugin::CrossbowPlugin;

/// Get java VM for executing Java calls
pub fn get_java_vm() -> error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}
