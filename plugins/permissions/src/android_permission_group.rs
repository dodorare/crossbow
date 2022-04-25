/// Android Permission Group
///
/// See for more details: https://developer.android.com/reference/android/Manifest.permission_group
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidPermissionGroup {
    /// Used for permissions that are associated with activity recognition.
    ActivityRecognition,
    /// Used for runtime permissions related to user's calendar.
    Calendar,
    /// Used for permissions that are associated telephony features.
    CallLog,
    /// Used for permissions that are associated with accessing camera or
    /// capturing images/video from the device.
    Camera,
    /// Used for runtime permissions related to contacts and profiles on this
    /// device.
    Contacts,
    /// Used for permissions that allow accessing the device location.
    Location,
    /// Used for permissions that are associated with accessing microphone audio
    /// from the device.
    Microphone,
    /// Required to be able to discover and connect to nearby Bluetooth devices.
    ///
    /// Protection level: dangerous
    NearbyDevices,
    /// Used for permissions that are associated telephony features.
    Phone,
    /// Used for permissions that are associated with accessing body or
    /// environmental sensors.
    Sensors,
    /// Used for runtime permissions related to user's SMS messages.
    SMS,
    /// Used for runtime permissions related to the shared external storage.
    Storage,
}

impl AndroidPermissionGroup {
    pub fn full_string(&self) -> String {
        "android.permission-group.".to_string() + self.to_string().as_str()
    }
}

impl std::fmt::Display for AndroidPermissionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ActivityRecognition => {
                write!(f, "ACTIVITY_RECOGNITION")
            }
            Self::Calendar => write!(f, "CALENDAR"),
            Self::CallLog => write!(f, "CALL_LOG"),
            Self::Camera => write!(f, "CAMERA"),
            Self::Contacts => write!(f, "CONTACTS"),
            Self::Location => write!(f, "LOCATION"),
            Self::Microphone => write!(f, "MICROPHONE"),
            Self::NearbyDevices => write!(f, "NEARBY_DEVICES"),
            Self::Phone => write!(f, "PHONE"),
            Self::Sensors => write!(f, "SENSORS"),
            Self::SMS => write!(f, "SMS"),
            Self::Storage => write!(f, "STORAGE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_string() {
        assert_eq!(
            AndroidPermissionGroup::ActivityRecognition.full_string(),
            "android.permission-group.ACTIVITY_RECOGNITION"
        );
    }
}