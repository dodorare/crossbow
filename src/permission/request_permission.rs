use super::*;
use crate::error::*;
#[cfg(all(target_os = "android", feature = "android"))]
use crossbow_android::permission::*;
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
            Permission::CalendarRead => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::ReadCalendar)
                    .await?
                    .into();
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::EventStore(EntityType::Event))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::CalendarWrite => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::WriteCalendar)
                    .await?
                    .into();
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::EventStore(EntityType::Event))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::Camera => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::Camera).await?.into();
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::CaptureDevice(MediaType::Video))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::ContactsRead => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::ReadContacts)
                    .await?
                    .into();
                // TODO: iOS
                Ok(_res)
            }
            Permission::ContactsWrite => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::WriteContacts)
                    .await?
                    .into();
                // TODO: iOS
                Ok(_res)
            }
            Permission::Flashlight => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::Flashlight)
                    .await?
                    .into();
                Ok(_res)
            }
            Permission::LocationWhenInUse => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = {
                    let res1 = request_permission(&AndroidPermission::AccessCoarseLocation).await?;
                    let res2 = request_permission(&AndroidPermission::AccessFineLocation).await?;
                    let res = res1 as u32 + res2 as u32;
                    match res {
                        2 => PermissionStatus::Granted,
                        1 => PermissionStatus::Restricted,
                        _ => PermissionStatus::Denied,
                    }
                };
                // TODO: iOS
                Ok(_res)
            }
            Permission::LocationAlways => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = {
                    let res1 = request_permission(&AndroidPermission::AccessCoarseLocation).await?;
                    let res2 = request_permission(&AndroidPermission::AccessFineLocation).await?;
                    let res3 =
                        request_permission(&AndroidPermission::AccessBackgroundLocation).await?;
                    let res = res1 as u32 + res2 as u32 + res3 as u32;
                    match res {
                        3 => PermissionStatus::Granted,
                        0 => PermissionStatus::Denied,
                        _ => PermissionStatus::Restricted,
                    }
                };
                // TODO: iOS
                Ok(_res)
            }
            Permission::Media => {
                let _res = PermissionStatus::Disabled;
                // TODO: Replace with https://developer.apple.com/documentation/mediaplayer/mpmedialibrary
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::PhotoLibrary(AccessLevel::AddOnly))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::Microphone => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::RecordAudio)
                    .await?
                    .into();
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::CaptureDevice(MediaType::Audio))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::Phone => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = {
                    let res1 = request_permission(&AndroidPermission::ReadPhoneState).await?;
                    let res2 = request_permission(&AndroidPermission::CallPhone).await?;
                    let res3 = request_permission(&AndroidPermission::ReadCallLog).await?;
                    let res4 = request_permission(&AndroidPermission::WriteCallLog).await?;
                    let res5 = request_permission(&AndroidPermission::AddVoicemail).await?;
                    let res6 = request_permission(&AndroidPermission::UseSip).await?;
                    // TODO: Add next line to be only if Android SDK 26
                    let res7 = request_permission(&AndroidPermission::AnswerPhoneCalls).await?;
                    // TODO: Add next line to be only if Android SDK 29
                    let res8 = request_permission(&AndroidPermission::ProcessOutgoingCalls).await?;
                    let res = res1 as u32
                        + res2 as u32
                        + res3 as u32
                        + res4 as u32
                        + res5 as u32
                        + res6 as u32
                        + res7 as u32
                        + res8 as u32;
                    match res {
                        8 => PermissionStatus::Granted,
                        0 => PermissionStatus::Denied,
                        _ => PermissionStatus::Restricted,
                    }
                };
                // TODO: iOS
                Ok(_res)
            }
            Permission::Photos => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::PhotoLibrary(AccessLevel::ReadWrite))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::Reminders => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::EventStore(EntityType::Reminder))
                    .await
                    .into();
                Ok(_res)
            }
            Permission::Sensors => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::BodySensors)
                    .await?
                    .into();
                // TODO: iOS
                Ok(_res)
            }
            Permission::Sms => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = {
                    let res1 = request_permission(&AndroidPermission::ReceiveSms).await?;
                    let res2 = request_permission(&AndroidPermission::SendSms).await?;
                    let res3 = request_permission(&AndroidPermission::ReadSms).await?;
                    let res4 = request_permission(&AndroidPermission::ReceiveWapPush).await?;
                    let res5 = request_permission(&AndroidPermission::ReceiveMms).await?;
                    let res = res1 as u32 + res2 as u32 + res3 as u32 + res4 as u32 + res5 as u32;
                    match res {
                        5 => PermissionStatus::Granted,
                        0 => PermissionStatus::Denied,
                        _ => PermissionStatus::Restricted,
                    }
                };
                // TODO: iOS
                Ok(_res)
            }
            Permission::Speech => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::RecordAudio)
                    .await?
                    .into();
                // TODO: iOS
                Ok(_res)
            }
            Permission::StorageRead => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::ReadExternalStorage)
                    .await?
                    .into();
                Ok(_res)
            }
            Permission::StorageWrite => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::WriteExternalStorage)
                    .await?
                    .into();
                Ok(_res)
            }
        }
    }
}
