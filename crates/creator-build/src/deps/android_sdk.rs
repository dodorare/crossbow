use super::*;
use crate::error::*;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct AndroidSdk {
    pub sdk_path: PathBuf,
}

impl Checks for AndroidSdk {
    fn check() -> Result<HashSet<CheckInfo>> {
        let mut checks = HashSet::new();
        if let Some(sdk_path) = std::env::var("ANDROID_SDK_ROOT")
            .ok()
            .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
            .or_else(|| std::env::var("ANDROID_HOME").ok())
        {
            let sdk_path = PathBuf::from(sdk_path);
            checks.insert(AndroidSdkChecks::EnvVarsAreSet.check_passed());
            if sdk_path.exists() {
                checks.insert(AndroidSdkChecks::Exists.check_passed());
            } else {
                // todo: need check default android sdk path
                checks.insert(AndroidSdkChecks::Exists.check_failed());
            };
        } else {
            checks.insert(AndroidSdkChecks::EnvVarsAreSet.check_failed());
            // todo: need check default android sdk path
        };
        Ok(checks)
    }
}

impl AndroidSdk {
    #[allow(dead_code)]
    fn init() -> Result<Rc<Self>> {
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

// #[derive(IntoCheckInfo)]
// #[name = "Android SDK"]
enum AndroidSdkChecks {
    // #[name = "Android SDK exists"]
    Exists,
    // #[name = "ANDROID_SDK_ROOT or ANDROID_SDK_PATH or ANDROID_HOME are set"]
    EnvVarsAreSet,
}

/// in future `IntoCheckInfo` will be implemented by #[derive] macro
impl IntoCheckInfo for AndroidSdkChecks {
    fn check_passed(self) -> CheckInfo {
        match self {
            AndroidSdkChecks::Exists => CheckInfo {
                dependency_name: "Android SDK".to_owned(),
                check_name: "Android SDK exists".to_owned(),
                passed: true,
            },
            AndroidSdkChecks::EnvVarsAreSet => CheckInfo {
                dependency_name: "Android SDK".to_owned(),
                check_name: "ANDROID_SDK_ROOT or ANDROID_SDK_PATH or ANDROID_HOME are set"
                    .to_owned(),
                passed: true,
            },
        }
    }
}
