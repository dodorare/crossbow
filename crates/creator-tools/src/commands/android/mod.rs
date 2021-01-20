///! Commands for compiling rust code for `Android`,
///! generation/aligning/signing/installing APKs,
///! generation `AndroidManifest.xml` and so on.
mod add_libs_into_apk;
mod align_apk;
mod create_manifest;
mod detect_abi;
mod gen_debug_key;
mod gen_unaligned_apk;
mod install_apk;
mod rust_compile;
mod sign_apk;
mod start_apk;

pub use add_libs_into_apk::*;
pub use align_apk::*;
pub use create_manifest::*;
pub use detect_abi::*;
pub use gen_debug_key::*;
pub use gen_unaligned_apk::*;
pub use install_apk::*;
pub use rust_compile::*;
pub use sign_apk::*;
pub use start_apk::*;
