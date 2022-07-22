use crate::{error::*, types::*};
use cocoa_foundation::{base::id, foundation::NSUInteger};
use objc::{class, msg_send, sel, sel_impl};

pub fn request_permission(permission: &IosPermission) -> Result<bool> {
    match permission {
        // IosPermission::PhotoLibrary(level) => {
        //     request_photo_library_permission(level);
        //     Ok(res == AuthorizationStatus::Limited || res == AuthorizationStatus::Authorized)
        // }
        _ => Ok(false),
    }
}

pub fn request_capture_device_permission<F>(media: &MediaType, handler: F)
where
    F: Fn(bool) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |success: bool| handler(success));
    let opt: id = media.into();
    unsafe {
        let _: () = msg_send![class!(AVCaptureDevice), requestAccessForMediaType:opt completionHandler:block.copy()];
    }
}

pub fn request_photo_library_permission<F>(level: &AccessLevel, handler: F)
where
    F: Fn(AuthorizationStatus) + Send + Sync + 'static,
{
    let block = block::ConcreteBlock::new(move |res: NSUInteger| {
        handler(AuthorizationStatus::from(res));
    });
    let opt: NSUInteger = level.into();
    unsafe {
        let _: () = msg_send![class!(PHPhotoLibrary), requestAuthorizationForAccessLevel:opt handler:block.copy()];
    }
}

// , error: id
// let mut opts: NSUInteger = 0;
// for opt in options {
//     let o: NSUInteger = opt.into();
//     opts = opts << o;
// }
// let localized_description: id = msg_send![error, localizedDescription];
// let bytes = localized_description.UTF8String() as *const u8;
// let e = std::str::from_utf8(std::slice::from_raw_parts(
//     bytes,
//     localized_description.len(),
// ))
// .unwrap();
// if e != "" {
//     println!("Error: {:?}", e);
// }
