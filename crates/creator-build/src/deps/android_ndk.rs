use super::*;
use crate::error::*;
use std::path::PathBuf;

pub struct AndroidNdk {
    pub android_sdk: Rc<AndroidSdk>,
    pub ndk_path: PathBuf,
    pub build_tools_version: String,
    pub platforms: Vec<u32>,
}

impl Dependency for AndroidNdk {
    fn check(&self) -> StdResult<()> {
        println!("checked android sdk");
        Ok(())
    }

    fn init() -> StdResult<Rc<Self>> {
        Err(Error::AndroidNdkNotFound)?
        // let ndk_path = {
        //     let ndk_path = std::env::var("ANDROID_NDK_ROOT")
        //         .ok()
        //         .or_else(|| std::env::var("ANDROID_NDK_PATH").ok())
        //         .or_else(|| std::env::var("ANDROID_NDK_HOME").ok())
        //         .or_else(|| std::env::var("NDK_HOME").ok());
        //     // default ndk installation path
        //     if ndk_path.is_none() && sdk_path.join("ndk-bundle").exists() {
        //         sdk_path.join("ndk-bundle")
        //     } else {
        //         PathBuf::from(ndk_path.ok_or(Error::AndroidNdkNotFound)?)
        //     }
        // };
        // Ok(Self {}.into())
    }
}

impl AndroidNdk {
    pub fn new(
        android_sdk: Rc<AndroidSdk>,
        ndk_path: PathBuf,
        build_tools_version: String,
        platforms: Vec<u32>,
    ) -> Rc<Self> {
        Self {
            android_sdk,
            ndk_path,
            build_tools_version,
            platforms,
        }
        .into()
    }
}
