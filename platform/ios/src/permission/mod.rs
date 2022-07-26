mod request_permission;
mod types;

pub use request_permission::*;
pub use types::*;

pub async fn request_permission(permission: &IosPermission) -> AuthorizationStatus {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let handler = move |status| {
        sender.send(status).unwrap();
    };
    request_permission_with_handler(permission, handler);
    receiver.recv().unwrap()
}

pub fn request_permission_with_handler<F>(permission: &IosPermission, handler: F)
where
    F: Fn(AuthorizationStatus) + Send + Sync + 'static,
{
    match permission {
        IosPermission::EventStore(opt) => {
            request_calendar_permission(opt, move |granted, _error| {
                if granted {
                    handler(AuthorizationStatus::Authorized);
                } else {
                    handler(AuthorizationStatus::Denied);
                };
            });
        }
        IosPermission::CaptureDevice(opt) => {
            request_capture_device_permission(opt, move |granted| {
                if granted {
                    handler(AuthorizationStatus::Authorized);
                } else {
                    handler(AuthorizationStatus::Denied);
                };
            });
        }
        IosPermission::PhotoLibrary(opt) => {
            request_photo_library_permission(opt, handler);
        }
        IosPermission::AddressBook => {
            request_address_book_permission(move |granted, _error| {
                if granted {
                    handler(AuthorizationStatus::Authorized);
                } else {
                    handler(AuthorizationStatus::Denied);
                };
            });
        }
        IosPermission::MediaLibrary => {
            request_media_permission(move |status| handler(AuthorizationStatus::from(status)));
        }
        IosPermission::SpeechRecognizer => {
            request_speech_recognition_permission(move |status| {
                handler(AuthorizationStatus::from(status));
            });
        }
        IosPermission::MotionActivityManager => {
            request_motion_activity_permission(move |activities, _error| {
                if activities != cocoa_foundation::base::nil {
                    handler(AuthorizationStatus::Authorized);
                } else {
                    handler(AuthorizationStatus::Denied);
                };
            });
        }
        IosPermission::LocationManager(opt) => {
            request_location_permission(opt);
            // TODO: Check permission.
            handler(AuthorizationStatus::Authorized);
        }
    }
}
