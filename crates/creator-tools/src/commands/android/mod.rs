mod add_libs_into_aapt2;
///! Commands for compiling rust code for `Android`,
///! generation/aligning/signing/installing APKs,
///! generation `AndroidManifest.xml` and so on.
mod add_libs_into_apk;
mod align_apk;
mod detect_abi;
mod extract_apk;
mod gen_aab_from_modules;
mod gen_base_aab_module;
mod gen_debug_key;
mod gen_debug_key_aab;
mod gen_manifest;
mod gen_unaligned_apk;
mod gen_unaligned_apk_aapt2;
mod install_apk;
mod jarsigner;
mod read_manifest;
mod rust_compile;
mod save_manifest;
mod sign_apk;
mod start_apk;
mod write_zip;

pub use add_libs_into_aapt2::*;
pub use add_libs_into_apk::*;
pub use align_apk::*;
pub use detect_abi::*;
pub use extract_apk::*;
pub use gen_aab_from_modules::*;
pub use gen_base_aab_module::*;
pub use gen_debug_key::*;
pub use gen_debug_key_aab::*;
pub use gen_manifest::*;
pub use gen_unaligned_apk::*;
pub use gen_unaligned_apk_aapt2::*;
pub use install_apk::*;
pub use jarsigner::*;
pub use read_manifest::*;
pub use rust_compile::*;
pub use save_manifest::*;
pub use sign_apk::*;
pub use start_apk::*;
pub use write_zip::*;
