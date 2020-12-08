use super::Dependency;
use crate::error::StdResult;
// use std::path::PathBuf;

pub struct AndroidSdk;

impl Dependency for AndroidSdk {
    fn check() -> StdResult<()> {
        println!("checked android sdk");
        Ok(())
    }
}

// pub struct AndroidSdk {
//     pub sdk_path: PathBuf,
// }

// impl AndroidSdk {
//     pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
//         let sdk_path = {
//             let mut sdk_path = std::env::var("ANDROID_SDK_ROOT").ok();
//             if sdk_path.is_none() {
//                 sdk_path = std::env::var("ANDROID_HOME").ok();
//                 println!(
//                     "Warning: You use environment variable ANDROID_HOME that is deprecated.\
//                  Please, remove it and use ANDROID_SDK_ROOT instead. Now ANDROID_HOME is used"
//                 );
//             }
//             sdk_path.ok_or(Error::AndroidSdkNotFound)?
//         };
//         Ok(AndroidSdk {
//             sdk_path: sdk_path.into(),
//         })
//     }
// }
