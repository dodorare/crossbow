use cocoa_foundation::{base::id, foundation::NSUInteger};

/// Type for iOS Permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IosPermission {
    /// EKEventStore.
    ///
    /// An object that accesses the user’s calendar events and reminders and supports the
    /// scheduling of new events.
    ///
    /// More details: https://developer.apple.com/documentation/eventkit/ekeventstore
    EventStore(EntityType),
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
    /// ABAddressBook.
    ///
    /// The main object you use to access the Address Book database.
    ///
    /// More details: https://developer.apple.com/documentation/addressbook/abaddressbook
    AddressBook,
    /// MPMediaLibrary.
    ///
    /// An object that represents the state of synced media items on a device.
    ///
    /// More details: https://developer.apple.com/documentation/mediaplayer/mpmedialibrary
    MediaLibrary,
    /// SFSpeechRecognizer.
    ///
    /// An object you use to check for the availability of the speech recognition service,
    /// and to initiate the speech recognition process.
    ///
    /// More details: https://developer.apple.com/documentation/speech/sfspeechrecognizer
    SpeechRecognizer,
    /// CMMotionActivityManager.
    ///
    /// An object that manages access to the motion data stored by the device.
    ///
    /// More details: https://developer.apple.com/documentation/coremotion/cmmotionactivitymanager
    MotionActivityManager,
    /// CLLocationManager.
    ///
    /// The object that you use to start and stop the delivery of location-related events
    /// to your app.
    ///
    /// More details: https://developer.apple.com/documentation/corelocation/cllocationmanager
    LocationManager(LocationAuthorizationType),
}

/// LocationAuthorizationType.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LocationAuthorizationType {
    /// Requests the user’s permission to use location services while the app is in use.
    WhenInUse,
    /// Requests the user’s permission to use location services regardless of whether the
    /// app is in use.
    Always,
}

/// AVMediaType.
///
/// An identifier for various media types.
///
/// More details: https://developer.apple.com/documentation/avfoundation/avmediatypeaudio
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaType {
    /// The media contains audio media.
    Audio,
    /// The media contains video.
    Video,
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    pub static AVMediaTypeVideo: id;
    pub static AVMediaTypeAudio: id;
}

impl Into<id> for &MediaType {
    fn into(self) -> id {
        match self {
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
    /// A value that indicates the app may only add to the user’s photo library.
    AddOnly,
    /// A value that indicates the app can read from and write to the user’s photo
    /// library.
    ReadWrite,
}

impl Into<NSUInteger> for &AccessLevel {
    fn into(self) -> NSUInteger {
        match self {
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
    /// The user hasn’t set the app’s authorization status.
    NotDetermined,
    /// The app isn’t authorized to access the photo library, and the user can’t grant
    /// such permission.
    Restricted,
    /// The user explicitly denied this app access to the photo library.
    Denied,
    /// The user explicitly granted this app access to the photo library.
    Authorized,
    /// The user authorized this app for limited photo library access.
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

/// EKEntityType.
///
/// The type of entities allowed for a source.
///
/// More details: https://developer.apple.com/documentation/eventkit/ekentitytype
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntityType {
    /// Represents an event.
    Event,
    /// Represents a reminder.
    Reminder,
}

impl Into<NSUInteger> for &EntityType {
    fn into(self) -> NSUInteger {
        match self {
            EntityType::Event => 0,
            EntityType::Reminder => 1,
        }
    }
}

/// MPMediaLibraryAuthorizationStatus.
///
/// The list of possible states for authorization to access to the user's media library.
///
/// More details: https://developer.apple.com/documentation/mediaplayer/mpmedialibraryauthorizationstatus
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaLibraryAuthorizationStatus {
    /// The user hasn't determined whether to authorize the use of their media library.
    NotDetermined,
    /// The app may not access the items in the user's media library.
    Denied,
    /// The app may access some of the content in the user's media library.
    Restricted,
    /// Your app may access items in the user's media library.
    Authorized,
}

impl From<NSUInteger> for MediaLibraryAuthorizationStatus {
    fn from(integer: NSUInteger) -> Self {
        match integer {
            0 => Self::NotDetermined,
            1 => Self::Denied,
            2 => Self::Restricted,
            _ => Self::Authorized,
        }
    }
}

impl From<MediaLibraryAuthorizationStatus> for AuthorizationStatus {
    fn from(val: MediaLibraryAuthorizationStatus) -> Self {
        match val {
            MediaLibraryAuthorizationStatus::NotDetermined => Self::NotDetermined,
            MediaLibraryAuthorizationStatus::Denied => Self::Denied,
            MediaLibraryAuthorizationStatus::Restricted => Self::Restricted,
            MediaLibraryAuthorizationStatus::Authorized => Self::Authorized,
        }
    }
}

/// SFSpeechRecognizerAuthorizationStatus.
///
/// The app's authorization to perform speech recognition.
///
/// More details: https://developer.apple.com/documentation/mediaplayer/mpmedialibraryauthorizationstatus
pub type SpeechRecognizerAuthorizationStatus = MediaLibraryAuthorizationStatus;
