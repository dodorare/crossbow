use crate::error::{Error, StdResult};

use std::path::PathBuf;

// struct AndroidSdk {
//     pub sdk_path: PathBuf,
// }

pub fn check_android_sdk() -> StdResult<()> {
    let sdk_path = {
        let mut sdk_path = std::env::var("ANDROID_SDK_ROOT").ok();
        if sdk_path.is_none() {
            sdk_path = std::env::var("ANDROID_HOME").ok();
            println!(
                "Warning: You use environment variable ANDROID_HOME that is deprecated.\
             Please, remove it and use ANDROID_SDK_ROOT instead. Now ANDROID_HOME is used"
            );
        }
        sdk_path.ok_or(Error::AndroidSdkNotFound)?;
    };
    Ok(())
}

// pub fn get_android_sdk() -> StdResult<String> {
//     let mut sdk_path = std::env::var("ANDROID_SDK_ROOT").ok();
//     if sdk_path.is_none() {
//         sdk_path = std::env::var("ANDROID_HOME").ok();
//         println!(
//             "Warning: You use environment variable ANDROID_HOME that is deprecated.\
//              Please, remove it and use ANDROID_SDK_ROOT instead. Now ANDROID_HOME is used"
//         );
//     }
//     Ok(sdk_path.ok_or(Error::AndroidSdkNotFound)?)
// }
