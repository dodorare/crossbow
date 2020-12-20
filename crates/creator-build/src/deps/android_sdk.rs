use super::*;
use crate::error::*;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct AndroidSdk {
    sdk_path: PathBuf,
    build_tools_dir: PathBuf,
    build_tools_version: Vec<String>,
    platforms_dir: PathBuf,
    platforms: Vec<u32>,
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
    fn init() -> Result<Rc<Self>> {
        let sdk_path = {
            let sdk_path = std::env::var("ANDROID_SDK_ROOT")
                .ok()
                .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
                .or_else(|| std::env::var("ANDROID_HOME").ok());
            PathBuf::from(sdk_path.ok_or(AndroidError::AndroidSdkNotFound)?)
        };
        // let build_tools_dir = sdk_path.join("build-tools");
        // let build_tools_version = std::fs::read_dir(&build_tools_dir)
        //     .or(Err(NdkError::PathNotFound(build_tools_dir)))?
        //     .filter_map(|path| path.ok())
        //     .filter(|path| path.path().is_dir())
        //     .filter_map(|path| path.file_name().into_string().ok())
        //     .filter(|name| name.chars().next().unwrap().is_digit(10))
        //     .max()
        //     .ok_or(NdkError::BuildToolsNotFound)?;

        // let platforms_dir = sdk_path.join("platforms");
        // let platforms: Vec<u32> = std::fs::read_dir(&platforms_dir)
        //     .or(Err(NdkError::PathNotFound(platforms_dir)))?
        //     .filter_map(|path| path.ok())
        //     .filter(|path| path.path().is_dir())
        //     .filter_map(|path| path.file_name().into_string().ok())
        //     .filter_map(|name| {
        //         name.strip_prefix("android-")
        //             .and_then(|api| api.parse::<u32>().ok())
        //     })
        //     .collect();
        // Ok(Self { sdk_path }.into())
        todo!();
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
