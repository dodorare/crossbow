///! Commands for compiling rust code for `Android`,
///! generation/aligning/signing/installing APKs,
///! generation `AndroidManifest.xml` and so on.
mod add_libs_into_apk;
mod align_apk;
mod attach_logger;
mod detect_abi;
mod gen_debug_key;
mod gen_manifest;
mod gen_unaligned_apk;
mod install_apk;
mod read_manifest;
mod rust_compile;
mod save_manifest;
mod sign_apk;
mod start_apk;

pub use add_libs_into_apk::*;
pub use align_apk::*;
pub use attach_logger::*;
pub use detect_abi::*;
pub use gen_debug_key::*;
pub use gen_manifest::*;
pub use gen_unaligned_apk::*;
pub use install_apk::*;
pub use read_manifest::*;
pub use rust_compile::*;
pub use save_manifest::*;
pub use sign_apk::*;
pub use start_apk::*;
