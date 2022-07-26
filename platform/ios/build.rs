//! Emits linker flags depending on platforms and features.

fn main() {
    #[cfg(feature = "eventkit")]
    println!("cargo:rustc-link-lib=framework=EventKit");
    #[cfg(feature = "avfoundation")]
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    #[cfg(feature = "photos")]
    println!("cargo:rustc-link-lib=framework=Photos");
    #[cfg(feature = "addressbook")]
    println!("cargo:rustc-link-lib=framework=AddressBook");
    #[cfg(feature = "mediaplayer")]
    println!("cargo:rustc-link-lib=framework=MediaPlayer");
    #[cfg(feature = "coremotion")]
    println!("cargo:rustc-link-lib=framework=CoreMotion");
    #[cfg(feature = "speech")]
    println!("cargo:rustc-link-lib=framework=Speech");
    #[cfg(feature = "corelocation")]
    println!("cargo:rustc-link-lib=framework=CoreLocation");
}
