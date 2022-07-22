use cocoa_foundation::{base::id, foundation::NSUInteger};

/// iOS Permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IosPermission {
    /// AVCaptureDevice.
    ///
    /// Hardware or virtual capture device like a camera or microphone.
    ///
    /// More details: https://developer.apple.com/documentation/avfoundation/avcapturedevice
    CaptureDevice(MediaType),
    /// PHPhotoLibrary.
    ///
    /// Access and changes to the user’s photo library.
    ///
    /// More details: https://developer.apple.com/documentation/photokit/phphotolibrary
    PhotoLibrary(AccessLevel),
}

/// AVMediaType.
///
/// An identifier for various media types.
///
/// More details: https://developer.apple.com/documentation/avfoundation/avmediatypeaudio
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaType {
    /// AVMediaTypeAudio.
    ///
    /// The media contains audio media.
    Audio,
    /// AVMediaTypeVideo.
    ///
    /// The media contains video.
    Video,
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    pub static AVMediaTypeVideo: id;
    pub static AVMediaTypeAudio: id;
}

impl From<MediaType> for id {
    fn from(val: MediaType) -> Self {
        match val {
            MediaType::Audio => unsafe { AVMediaTypeAudio },
            MediaType::Video => unsafe { AVMediaTypeVideo },
        }
    }
}

impl From<&MediaType> for id {
    fn from(val: &MediaType) -> Self {
        match val {
            MediaType::Audio => unsafe { AVMediaTypeAudio },
            MediaType::Video => unsafe { AVMediaTypeVideo },
        }
    }
}

/// PHAccessLevel.
///
/// The app’s level of access to the user’s photo library.
///
/// More details: https://developer.apple.com/documentation/photokit/phaccesslevel
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessLevel {
    AddOnly,
    ReadWrite,
}

impl From<AccessLevel> for NSUInteger {
    fn from(level: AccessLevel) -> Self {
        match level {
            AccessLevel::AddOnly => 1 << 0,
            AccessLevel::ReadWrite => 1 << 1,
        }
    }
}

impl From<&AccessLevel> for NSUInteger {
    fn from(level: &AccessLevel) -> Self {
        match level {
            AccessLevel::AddOnly => 1 << 0,
            AccessLevel::ReadWrite => 1 << 1,
        }
    }
}

/// PHAuthorizationStatus.
///
/// Information about your app’s authorization to access the user’s photo library.
///
/// More details: https://developer.apple.com/documentation/photokit/phauthorizationstatus
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AuthorizationStatus {
    NotDetermined,
    Restricted,
    Denied,
    Authorized,
    Limited,
}

impl From<NSUInteger> for AuthorizationStatus {
    fn from(integer: NSUInteger) -> Self {
        match integer {
            0 => Self::NotDetermined,
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::Authorized,
            _ => Self::Limited,
        }
    }
}
