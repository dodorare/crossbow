///! Commands for compiling rust code for `Android`,
///! generation/aligning/signing/installing/starting on device APKs and AAB,
///! generation `AndroidManifest.xml` and so on.
mod add_libs_into_aapt2;
mod add_libs_into_apk;
mod align_apk;
mod attach_logger;
mod detect_abi;
mod extract_apk;
mod generate;
mod helper_functions;
mod install_apk;
mod read_manifest;
mod rust_compile;
mod save_manifest;
mod sign_apk;
mod start_apk;
mod write_zip;
mod gradle_command;

pub use add_libs_into_aapt2::*;
pub use add_libs_into_apk::*;
pub use align_apk::*;
pub use attach_logger::*;
pub use detect_abi::*;
pub use extract_apk::*;
pub use generate::*;
pub use helper_functions::*;
pub use install_apk::*;
pub use read_manifest::*;
pub use rust_compile::*;
pub use save_manifest::*;
pub use sign_apk::*;
pub use start_apk::*;
pub use write_zip::*;
pub use gradle_command::*;