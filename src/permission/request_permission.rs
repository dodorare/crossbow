use super::*;
use crate::error::*;
#[cfg(all(target_os = "android", feature = "android"))]
use crossbow_android::{permission::*, types::AndroidPermission};
#[cfg(all(target_os = "ios", feature = "ios"))]
use crossbow_ios::permission::*;

/// Generic Permission type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permission {
    /// Read Access to the Calendar.
    ///
    /// Platforms: __Android / iOS__.
    CalendarRead,
    /// Read and Write Access to the Calendar.
    ///
    /// Platforms: __Android / iOS__.
    CalendarWrite,
    /// Access to the Camera.
    ///
    /// Platforms: __Android / iOS__.
    Camera,
    /// Read Access to the Contacts.
    ///
    /// Platforms: __Android / iOS__.
    ContactsRead,
    /// Read and Write Access to the Contacts.
    ///
    /// Platforms: __Android / iOS__.
    ContactsWrite,
    /// Access to the Flashlight.
    ///
    /// Platforms: __Android__.
    Flashlight,
    /// Access to the Location when in use.
    ///
    /// Platforms: __Android / iOS__.
    LocationWhenInUse,
    /// Permanent Access to the Location.
    ///
    /// Platforms: __Android / iOS__.
    LocationAlways,
    /// Access to the Media.
    ///
    /// Platforms: __iOS__.
    Media,
    /// Access to the Microphone.
    ///
    /// Platforms: __Android / iOS__.
    Microphone,
    /// Access to the Phone.
    ///
    /// Platforms: __Android / iOS__.
    Phone,
    /// Access to the Photos.
    ///
    /// Platforms: __iOS__.
    Photos,
    /// Access to the Reminders.
    ///
    /// Platforms: __iOS__.
    Reminders,
    /// Access to the Sensors.
    ///
    /// Platforms: __Android / iOS__.
    Sensors,
    /// Access to the SMS.
    ///
    /// Platforms: __Android / iOS__.
    Sms,
    /// Access to the Speech Service.
    ///
    /// Platforms: __Android / iOS__.
    Speech,
    /// Read Access to the Storage.
    ///
    /// Platforms: __Android__.
    StorageRead,
    /// Read and Write Access to the Storage.
    ///
    /// Platforms: __Android__.
    StorageWrite,
}

impl Permission {
    pub async fn request_async(&self) -> Result<PermissionStatus> {
        match self {
            Permission::Camera => {
                let res = PermissionStatus::Denied;
                #[cfg(all(target_os = "android", feature = "android"))]
                let res = request_permission(&AndroidPermission::Camera)?;
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let res = request_permission(&IosPermission::CaptureDevice(MediaType::Video))
                    .await
                    .into();
                Ok(res)
            }
            _ => unimplemented!(),
        }
    }
}
