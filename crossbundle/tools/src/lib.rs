/// On Windows adds `.exe` to given string.
#[cfg(feature = "android")]
macro_rules! bin {
    ($bin:expr_2021) => {{
        #[cfg(not(target_os = "windows"))]
        let bin = $bin;
        #[cfg(target_os = "windows")]
        let bin = concat!($bin, ".exe");
        bin
    }};
}

/// On Windows adds `.bat` to given string.
#[cfg(feature = "android")]
macro_rules! bat {
    ($bat:expr_2021) => {{
        #[cfg(not(target_os = "windows"))]
        let bat = $bat;
        #[cfg(target_os = "windows")]
        let bat = concat!($bat, ".bat");
        bat
    }};
}

#[cfg(target_os = "windows")]
pub const EXECUTABLE_SUFFIX_BAT: &str = ".bat";

#[cfg(not(target_os = "windows"))]
pub const EXECUTABLE_SUFFIX_BAT: &str = "";

pub mod commands;
pub mod error;
#[cfg(any(feature = "android", feature = "apple"))]
pub mod toolchain;
pub mod types;
