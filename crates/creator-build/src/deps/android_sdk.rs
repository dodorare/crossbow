use super::*;
use crate::error::*;
use std::path::PathBuf;

pub struct AndroidSdk {
    pub sdk_path: PathBuf,
}

impl Dependency for AndroidSdk {
    type Input = PathBuf;

    fn check(&self) -> StdResult<()> {
        println!("checking android sdk");
        if !self.sdk_path.exists() {
            Err(Error::AndroidSdkNotFound)?
        }
        Ok(())
    }

    fn init(sdk_path: Option<Self::Input>) -> StdResult<Arc<Self>> {
        if let Some(sdk_path) = sdk_path {
            return Ok(Self { sdk_path }.into());
        }
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
