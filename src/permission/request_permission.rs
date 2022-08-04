use super::*;
use crate::error::*;
#[cfg(all(target_os = "android", feature = "android"))]
use crossbow_android::permission::*;
#[cfg(all(target_os = "ios", feature = "ios"))]
use crossbow_ios::permission::*;
use serde::{Deserialize, Serialize};

/// Generic Permission type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Permission {
    /// Read Access to the Calendar.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.READ_CALENDAR**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSCalendarsUsageDescription**
    #[serde(rename = "calendar-read")]
    CalendarRead,
    /// Read and Write Access to the Calendar.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.WRITE_CALENDAR**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSCalendarsUsageDescription**
    #[serde(rename = "calendar-write")]
    CalendarWrite,
    /// Access to the Camera.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.CAMERA**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSCameraUsageDescription**
    #[serde(rename = "camera")]
    Camera,
    /// Read Access to the Contacts.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.READ_CONTACTS**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSContactsUsageDescription**
    #[serde(rename = "contacts-read")]
    ContactsRead,
    /// Read and Write Access to the Contacts.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.WRITE_CONTACTS**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSContactsUsageDescription**
    #[serde(rename = "contacts-write")]
    ContactsWrite,
    /// Access to the Flashlight.
    ///
    /// Platforms: **Android**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.CAMERA**
    /// * **android.permission.FLASHLIGHT**
    #[serde(rename = "flashlight")]
    Flashlight,
    /// Access to the Location when in use.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.ACCESS_COARSE_LOCATION**
    /// * **android.permission.ACCESS_FINE_LOCATION**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSLocationWhenInUseUsageDescription**
    #[serde(rename = "location-when-in-use")]
    LocationWhenInUse,
    /// Permanent Access to the Location.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.ACCESS_COARSE_LOCATION**
    /// * **android.permission.ACCESS_FINE_LOCATION**
    /// * **android.permission.ACCESS_BACKGROUND_LOCATION**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSLocationAlwaysAndWhenInUseUsageDescription**
    /// * **NSLocationAlwaysUsageDescription**
    #[serde(rename = "location-always")]
    LocationAlways,
    /// Access to the Media.
    ///
    /// Platforms: **iOS**.
    ///
    /// Required Permissions for **iOS**:
    /// * **NSAppleMusicUsageDescription**
    #[serde(rename = "media")]
    Media,
    /// Access to the Microphone.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.RECORD_AUDIO**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSMicrophoneUsageDescription**
    #[serde(rename = "microphone")]
    Microphone,
    /// Access to the Phone.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.READ_PHONE_STATE**
    /// * **android.permission.CALL_PHONE**
    /// * **android.permission.READ_CALL_LOG**
    /// * **android.permission.WRITE_CALL_LOG**
    /// * **android.permission.ADD_VOICEMAIL**
    /// * **android.permission.USE_SIP**
    /// * **android.permission.ANSWER_PHONE_CALLS**
    /// * **android.permission.PROCESS_OUTGOING_CALLS**
    #[serde(rename = "phone")]
    Phone,
    /// Access to the Photos.
    ///
    /// Platforms: **iOS**.
    ///
    /// Required Permissions for **iOS**:
    /// * **NSPhotoLibraryAddUsageDescription**
    /// * **NSPhotoLibraryUsageDescription**
    #[serde(rename = "photos")]
    Photos,
    /// Access to the Reminders.
    ///
    /// Platforms: **iOS**.
    ///
    /// Required Permissions for **iOS**:
    /// * **NSRemindersUsageDescription**
    #[serde(rename = "reminders")]
    Reminders,
    /// Access to the Sensors.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.BODY_SENSORS**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSMotionUsageDescription**
    #[serde(rename = "sensors")]
    Sensors,
    /// Access to the SMS.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.RECEIVE_SMS**
    /// * **android.permission.SEND_SMS**
    /// * **android.permission.READ_SMS**
    /// * **android.permission.RECEIVE_WAP_PUSH**
    /// * **android.permission.RECEIVE_MMS**
    #[serde(rename = "sms")]
    Sms,
    /// Access to the Speech Service.
    ///
    /// Platforms: **Android / iOS**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.RECORD_AUDIO**
    ///
    /// Required Permissions for **iOS**:
    /// * **NSSpeechRecognitionUsageDescription**
    #[serde(rename = "speech")]
    Speech,
    /// Read Access to the Storage.
    ///
    /// Platforms: **Android**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.READ_EXTERNAL_STORAGE**
    #[serde(rename = "storage-read")]
    StorageRead,
    /// Read and Write Access to the Storage.
    ///
    /// Platforms: **Android**.
    ///
    /// Required Permissions for **Android**:
    /// * **android.permission.WRITE_EXTERNAL_STORAGE**
    #[serde(rename = "storage-write")]
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
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::AddressBook).await.into();
                Ok(_res)
            }
            Permission::ContactsWrite => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::WriteContacts)
                    .await?
                    .into();
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::AddressBook).await.into();
                Ok(_res)
            }
            Permission::Flashlight => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::Camera).await?.into();
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
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::LocationManager(
                    LocationAuthorizationType::WhenInUse,
                ))
                .await
                .into();
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
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::LocationManager(
                    LocationAuthorizationType::Always,
                ))
                .await
                .into();
                Ok(_res)
            }
            Permission::Media => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::MediaLibrary)
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
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::MotionActivityManager)
                    .await
                    .into();
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
                Ok(_res)
            }
            Permission::Speech => {
                let _res = PermissionStatus::Disabled;
                #[cfg(all(target_os = "android", feature = "android"))]
                let _res = request_permission(&AndroidPermission::RecordAudio)
                    .await?
                    .into();
                #[cfg(all(target_os = "ios", feature = "ios"))]
                let _res = request_permission(&IosPermission::SpeechRecognizer)
                    .await
                    .into();
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

    #[cfg(feature = "update-manifest")]
    pub fn update_manifest(&self, manifest: &mut android_manifest::AndroidManifest) {
        let mut permissions = vec![];
        let mut add_p = |p: &str| {
            permissions.push(android_manifest::UsesPermission {
                name: Some(p.to_owned()),
                ..Default::default()
            });
        };
        match self {
            Permission::CalendarRead => {
                add_p("android.permission.READ_CALENDAR");
            }
            Permission::CalendarWrite => {
                add_p("android.permission.WRITE_CALENDAR");
            }
            Permission::Camera => {
                add_p("android.permission.CAMERA");
            }
            Permission::ContactsRead => {
                add_p("android.permission.READ_CONTACTS");
            }
            Permission::ContactsWrite => {
                add_p("android.permission.WRITE_CONTACTS");
            }
            Permission::Flashlight => {
                add_p("android.permission.CAMERA");
                add_p("android.permission.FLASHLIGHT");
            }
            Permission::LocationWhenInUse => {
                add_p("android.permission.ACCESS_COARSE_LOCATION");
                add_p("android.permission.ACCESS_FINE_LOCATION");
            }
            Permission::LocationAlways => {
                add_p("android.permission.ACCESS_COARSE_LOCATION");
                add_p("android.permission.ACCESS_FINE_LOCATION");
                add_p("android.permission.ACCESS_BACKGROUND_LOCATION");
            }
            Permission::Media => {}
            Permission::Microphone => {
                add_p("android.permission.RECORD_AUDIO");
            }
            Permission::Phone => {
                add_p("android.permission.READ_PHONE_STATE");
                add_p("android.permission.CALL_PHONE");
                add_p("android.permission.READ_CALL_LOG");
                add_p("android.permission.WRITE_CALL_LOG");
                add_p("android.permission.ADD_VOICEMAIL");
                add_p("android.permission.USE_SIP");
                add_p("android.permission.ANSWER_PHONE_CALLS");
                add_p("android.permission.PROCESS_OUTGOING_CALLS");
            }
            Permission::Photos => {}
            Permission::Reminders => {}
            Permission::Sensors => {
                add_p("android.permission.BODY_SENSORS");
            }
            Permission::Sms => {
                add_p("android.permission.RECEIVE_SMS");
                add_p("android.permission.SEND_SMS");
                add_p("android.permission.READ_SMS");
                add_p("android.permission.RECEIVE_WAP_PUSH");
                add_p("android.permission.RECEIVE_MMS");
            }
            Permission::Speech => {
                add_p("android.permission.RECORD_AUDIO");
            }
            Permission::StorageRead => {
                add_p("android.permission.READ_EXTERNAL_STORAGE");
            }
            Permission::StorageWrite => {
                add_p("android.permission.WRITE_EXTERNAL_STORAGE");
            }
        }
        // If AndroidManifest already has permission we want to add - don't add it.
        let mut filtered = permissions
            .iter()
            .filter(|p| !manifest.uses_permission.iter().any(|x| x.name == p.name))
            .cloned()
            .collect();
        manifest.uses_permission.append(&mut filtered);
    }

    #[cfg(feature = "update-manifest")]
    pub fn update_info_plist(&self, props: &mut apple_bundle::info_plist::InfoPlist) {
        match self {
            Permission::CalendarRead => {
                props.calendar_and_reminders.calendars_usage_description =
                    Some("This app needs access to your phone's calendar.".to_string());
            }
            Permission::CalendarWrite => {
                props.calendar_and_reminders.calendars_usage_description =
                    Some("This app needs access to your phone's calendar.".to_string());
            }
            Permission::Camera => {
                props.camera_and_microphone.camera_usage_description =
                    Some("This app needs access to your phone's camera.".to_string());
            }
            Permission::ContactsRead => {
                props.contacts.contacts_usage_description =
                    Some("This app needs access to your phone's contacts.".to_string());
            }
            Permission::ContactsWrite => {
                props.contacts.contacts_usage_description =
                    Some("This app needs access to your phone's contacts.".to_string());
            }
            Permission::Flashlight => {}
            Permission::LocationWhenInUse => {
                props.location.location_when_in_use_usage_description =
                    Some("This app needs access to your phone's location when in use.".to_string());
            }
            Permission::LocationAlways => {
                props
                    .location
                    .location_always_and_when_in_use_usage_description =
                    Some("This app needs permanent access to your phone's location.".to_string());
                // #[warn(deprecated)]
                // props.location.location_always_usage_description =
                //     Some("This app needs permanent access to your phone's
                // location.".to_string());
            }
            Permission::Media => {
                props.media_player.apple_music_usage_description =
                    Some("This app needs access to your phone's Apple Music.".to_string());
            }
            Permission::Microphone => {
                props.camera_and_microphone.microphone_usage_description =
                    Some("This app needs access to your phone's microphone.".to_string());
            }
            Permission::Phone => {}
            Permission::Photos => {
                props.photos.photo_library_add_usage_description =
                    Some("This app needs add access to your phone's photo library.".to_string());
                props.photos.photo_library_usage_description =
                    Some("This app needs access to your phone's photo library.".to_string());
            }
            Permission::Reminders => {
                props.calendar_and_reminders.reminders_usage_description =
                    Some("This app needs access to your phone's reminders.".to_string());
            }
            Permission::Sensors => {
                props.motion.motion_usage_description =
                    Some("This app needs access to your phone's motion sensors.".to_string());
            }
            Permission::Sms => {}
            Permission::Speech => {
                props.speech.speech_recognition_usage_description =
                    Some("This app needs access to your phone's speech recognition.".to_string());
            }
            Permission::StorageRead => {}
            Permission::StorageWrite => {}
        }
    }
}
