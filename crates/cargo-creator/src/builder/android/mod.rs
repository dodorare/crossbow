macro_rules! bin {
    ($bin:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bin = $bin;
        #[cfg(target_os = "windows")]
        let bin = concat!($bin, ".exe");
        bin
    }};
}

macro_rules! bat {
    ($bat:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bat = $bat;
        #[cfg(target_os = "windows")]
        let bat = concat!($bat, ".bat");
        bat
    }};
}

pub mod apk;
pub mod builder;
pub mod dylibs;
pub mod error;
pub mod metadata;
pub mod ndk_build;
pub mod target;

// pub use apk::*;
// pub use builder::*;
// pub use dylibs::*;
// pub use metadata::*;
// pub use ndk_build::*;
// pub use target::*;
