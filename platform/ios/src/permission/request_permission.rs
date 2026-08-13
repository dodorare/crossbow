use super::*;
use block2::RcBlock;
use objc2::{
    class, msg_send,
    runtime::{AnyObject, Bool},
};

pub fn request_capture_device_permission<F>(media: &MediaType, handler: F)
where
    F: Fn(bool) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |success: Bool| handler(success.as_bool()));
    let opt: ObjcObjectPtr = media.into();
    let _: () = unsafe {
        msg_send![
            class!(AVCaptureDevice),
            requestAccessForMediaType: opt,
            completionHandler: &*block
        ]
    };
}

pub fn request_photo_library_permission<F>(level: &AccessLevel, handler: F)
where
    F: Fn(AuthorizationStatus) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |res: usize| {
        handler(AuthorizationStatus::from(res));
    });
    let opt: usize = level.into();
    let _: () = unsafe {
        msg_send![
            class!(PHPhotoLibrary),
            requestAuthorizationForAccessLevel: opt,
            handler: &*block
        ]
    };
}

pub fn request_calendar_permission<F>(entity_type: &EntityType, handler: F)
where
    F: Fn(bool, ObjcObjectPtr) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |granted: Bool, error: ObjcObjectPtr| {
        handler(granted.as_bool(), error);
    });
    let opt: usize = entity_type.into();
    let _: () = unsafe {
        msg_send![
            class!(EKEventStore),
            requestAccessToEntityType: opt,
            completion: &*block
        ]
    };
}

pub fn request_address_book_permission<F>(handler: F)
where
    F: Fn(bool, ObjcObjectPtr) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |granted: Bool, error: ObjcObjectPtr| {
        handler(granted.as_bool(), error);
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/addressbook/1621991-abaddressbookcreatewithoptions
        let address_book_ref: ObjcObjectPtr = msg_send![
            class!(ABAddressBook),
            ABAddressBookCreateWithOptions: std::ptr::null_mut::<AnyObject>(),
            error: std::ptr::null_mut::<AnyObject>()
        ];
        // https://developer.apple.com/documentation/addressbook/1622001-abaddressbookrequestaccesswithco
        msg_send![
            class!(ABAddressBook),
            ABAddressBookRequestAccessWithCompletion: address_book_ref,
            completion: &*block
        ]
    };
}

pub fn request_media_permission<F>(handler: F)
where
    F: Fn(MediaLibraryAuthorizationStatus) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |status: usize| {
        handler(MediaLibraryAuthorizationStatus::from(status));
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/mediaplayer/mpmedialibrary/1621276-requestauthorization
        msg_send![
            class!(MPMediaLibrary),
            requestAuthorization: &*block
        ]
    };
}

pub fn request_speech_recognition_permission<F>(handler: F)
where
    F: Fn(SpeechRecognizerAuthorizationStatus) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |status: usize| {
        handler(SpeechRecognizerAuthorizationStatus::from(status));
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/mediaplayer/mpmedialibrary/1621276-requestauthorization
        msg_send![
            class!(SFSpeechRecognizer),
            requestAuthorization: &*block
        ]
    };
}

pub fn request_motion_activity_permission<F>(handler: F)
where
    F: Fn(ObjcObjectPtr, ObjcObjectPtr) + Send + Sync + 'static,
{
    let block = RcBlock::new(move |activities: ObjcObjectPtr, error: ObjcObjectPtr| {
        handler(activities, error);
    });
    let _: () = unsafe {
        // https://developer.apple.com/documentation/coremotion/cmmotionactivitymanager/1615929-queryactivitystartingfromdate
        msg_send![
            class!(CMMotionActivityManager),
            queryActivityStartingFromDate: std::ptr::null_mut::<AnyObject>(),
            toDate: std::ptr::null_mut::<AnyObject>(),
            toQueue: std::ptr::null_mut::<AnyObject>(),
            handler: &*block
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
