use super::*;
use cocoa_foundation::{base::id, foundation::NSUInteger};
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
