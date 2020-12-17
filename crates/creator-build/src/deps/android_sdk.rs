use super::*;
use crate::error::*;
use std::path::PathBuf;

pub struct AndroidSdk {
    pub sdk_path: PathBuf,
}

impl Dependency for AndroidSdk {
    fn check(&self) -> StdResult<()> {
        println!("checking android sdk");
        if !self.sdk_path.exists() {
            Err(Error::AndroidSdkNotFound)?
        }
        Ok(())
    }

    fn init() -> StdResult<Rc<Self>> {
        let sdk_path = {
            let sdk_path = std::env::var("ANDROID_SDK_ROOT")
                .ok()
                .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
                .or_else(|| std::env::var("ANDROID_HOME").ok());
            PathBuf::from(sdk_path.ok_or(Error::AndroidSdkNotFound)?)
        };
        Ok(Self { sdk_path }.into())
    }
}

impl AndroidSdk {
    pub fn new(sdk_path: PathBuf) -> Rc<Self> {
        Self { sdk_path }.into()
    }
}
