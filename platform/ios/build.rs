//! Emits linker flags depending on platforms and features.

fn main() {
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=Photos");
}
