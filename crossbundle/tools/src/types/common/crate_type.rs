#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrateType {
    /// A runnable executable.
    Bin,
    /// A Rust library.
    Lib,
    /// A dynamic Rust library.
    Dylib,
    /// A static system library.
    Staticlib,
    /// A dynamic system library.
    Cdylib,
    /// A "Rust library" file.
    Rlib,
}

impl AsRef<str> for CrateType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Bin => "bin",
            Self::Lib => "lib",
            Self::Dylib => "dylib",
            Self::Staticlib => "staticlib",
            Self::Cdylib => "cdylib",
            Self::Rlib => "rlib",
        }
    }
}
