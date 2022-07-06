pub(crate) mod crossbow_lib;
pub(crate) mod utils;

pub mod crossbow_plugin;
pub mod error;
pub mod permission;
pub mod types;

pub use jni;

pub fn init() {
    println!("init");
}

/// Create a java VM for executing Java calls
pub fn create_java_vm() -> error::Result<(ndk_context::AndroidContext, jni::JavaVM)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    Ok((ctx, vm))
}
