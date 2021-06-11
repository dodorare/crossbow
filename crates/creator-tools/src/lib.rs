/// On Windows adds `.exe` to given string.
macro_rules! bin {
    ($bin:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bin = $bin;
        #[cfg(target_os = "windows")]
        let bin = concat!($bin, ".exe");
        bin
    }};
}

/// On Windows adds `.bat` to given string.
macro_rules! bat {
    ($bat:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bat = $bat;
        #[cfg(target_os = "windows")]
        let bat = concat!($bat, ".bat");
        bat
    }};
}

pub mod commands;
pub mod error;
pub mod tools;
pub mod types;
pub mod utils;

pub use simctl;
