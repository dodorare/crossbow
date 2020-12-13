pub trait IntoRustTriple {
    /// Returns the triple used by the rust build tools
    fn rust_triple(&self) -> &'static str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AndroidTarget {
    Armv7LinuxAndroideabi,
    Aarch64LinuxAndroid,
    I686LinuxAndroid,
    X8664LinuxAndroid,
}

impl IntoRustTriple for AndroidTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::Armv7LinuxAndroideabi => "armv7-linux-androideabi",
            Self::Aarch64LinuxAndroid => "aarch64-linux-android",
            Self::I686LinuxAndroid => "i686-linux-android",
            Self::X8664LinuxAndroid => "x86_64-linux-android",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppleTarget {
    X8664AppleIosMacabi,
    I386AppleIos,
    Aarch64AppleIos,
    Armv7AppleIos,
    Armv7sAppleIos,
}

impl IntoRustTriple for AppleTarget {
    fn rust_triple(&self) -> &'static str {
        match self {
            Self::X8664AppleIosMacabi => "x86_64-apple-ios",
            Self::I386AppleIos => "i386-apple-ios",
            Self::Aarch64AppleIos => "aarch64-apple-ios",
            Self::Armv7AppleIos => "armv7-apple-ios",
            Self::Armv7sAppleIos => "armv7s-apple-ios",
        }
    }
}
