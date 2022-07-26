use super::*;
use cocoa_foundation::{
    base::{id, nil},
    foundation::NSUInteger,
};
use objc::{class, msg_send, sel, sel_impl};

pub fn request_capture_device_permission<F>(media: &MediaType, handler: F)
where
    F: Fn(bool) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |success: bool| handler(success));
    let opt: id = media.into();
    let _: () = unsafe {
        msg_send![
            class!(AVCaptureDevice),
            requestAccessForMediaType: opt
            completionHandler: block.copy()
        ]
    };
}

pub fn request_photo_library_permission<F>(level: &AccessLevel, handler: F)
where
    F: Fn(AuthorizationStatus) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |res: NSUInteger| {
        handler(AuthorizationStatus::from(res));
    });
    let opt: NSUInteger = level.into();
    let _: () = unsafe {
        msg_send![
            class!(PHPhotoLibrary),
            requestAuthorizationForAccessLevel: opt
            handler: block.copy()
        ]
    };
}

pub fn request_calendar_permission<F>(entity_type: &EntityType, handler: F)
where
    F: Fn(bool, id) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |granted: bool, error: id| {
        handler(granted, error);
    });
    let opt: NSUInteger = entity_type.into();
    let _: () = unsafe {
        msg_send![
            class!(EKEventStore),
            requestAccessToEntityType: opt
            completion: block.copy()
        ]
    };
}

pub fn request_address_book_permission<F>(handler: F)
where
    F: Fn(bool, id) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |granted: bool, error: id| {
        handler(granted, error);
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/addressbook/1621991-abaddressbookcreatewithoptions
        let address_book_ref: id = msg_send![
            class!(ABAddressBook),
            ABAddressBookCreateWithOptions: nil
            error: nil
        ];
        // https://developer.apple.com/documentation/addressbook/1622001-abaddressbookrequestaccesswithco
        msg_send![
            class!(ABAddressBook),
            ABAddressBookRequestAccessWithCompletion: address_book_ref
            completion: block.copy()
        ]
    };
}

pub fn request_media_permission<F>(handler: F)
where
    F: Fn(MediaLibraryAuthorizationStatus) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |status: NSUInteger| {
        handler(MediaLibraryAuthorizationStatus::from(status));
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/mediaplayer/mpmedialibrary/1621276-requestauthorization
        msg_send![
            class!(MPMediaLibrary),
            requestAuthorization: block.copy()
        ]
    };
}

pub fn request_speech_recognition_permission<F>(handler: F)
where
    F: Fn(SpeechRecognizerAuthorizationStatus) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |status: NSUInteger| {
        handler(SpeechRecognizerAuthorizationStatus::from(status));
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/mediaplayer/mpmedialibrary/1621276-requestauthorization
        msg_send![
            class!(SFSpeechRecognizer),
            requestAuthorization: block.copy()
        ]
    };
}

pub fn request_motion_activity_permission<F>(handler: F)
where
    F: Fn(id, id) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |activities: id, error: id| {
        handler(activities, error);
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/coremotion/cmmotionactivitymanager/1615929-queryactivitystartingfromdate
        msg_send![
            class!(CMMotionActivityManager),
            queryActivityStartingFromDate: nil
            toDate: nil
            toQueue: nil
            handler: block.copy()
        ]
    };
}

pub fn request_location_permission(location: &LocationAuthorizationType) {
    match location {
        LocationAuthorizationType::Always => {
            let _: () = unsafe {
                // https://developer.apple.com/documentation/corelocation/cllocationmanager/1620551-requestalwaysauthorization
                msg_send![class!(CLLocationManager), requestAlwaysAuthorization]
            };
        }
        LocationAuthorizationType::WhenInUse => {
            let _: () = unsafe {
                // https://developer.apple.com/documentation/corelocation/cllocationmanager/1620562-requestwheninuseauthorization
                msg_send![class!(CLLocationManager), requestWhenInUseAuthorization]
            };
        }
    }
}
