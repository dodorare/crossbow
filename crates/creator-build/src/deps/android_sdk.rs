use super::*;
use crate::error::*;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

pub struct AndroidSdk {
    sdk_path: PathBuf,
    build_tools_path: PathBuf,
    build_tools_version: String,
    platforms_path: PathBuf,
    platforms: Vec<u32>,
}

impl Checks for AndroidSdk {
    fn check() -> Result<Vec<CheckInfo>> {
        let mut checks = Vec::new();
        if let Some(sdk_path) = std::env::var("ANDROID_SDK_ROOT")
            .ok()
            .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
            .or_else(|| std::env::var("ANDROID_HOME").ok())
        {
            let sdk_path = PathBuf::from(sdk_path);
            checks.push(AndroidSdkChecks::EnvVarsAreSet.check_passed());
            if sdk_path.exists() {
                checks.push(AndroidSdkChecks::Found.check_passed());
            } else {
                checks.push(AndroidSdkChecks::Found.check_failed());
            };
        } else {
            checks.push(AndroidSdkChecks::EnvVarsAreSet.check_failed());
            checks.push(AndroidSdkChecks::Found.check_failed());
        };
        Ok(checks)
    }
}

impl AndroidSdk {
    pub fn init() -> Result<Rc<Self>> {
        let sdk_path = {
            let sdk_path = std::env::var("ANDROID_SDK_ROOT")
                .ok()
                .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
                .or_else(|| std::env::var("ANDROID_HOME").ok());
            PathBuf::from(sdk_path.ok_or(AndroidError::AndroidSdkNotFound)?)
        };
        let build_tools_path = sdk_path.join("build-tools");
        let build_tools_version = std::fs::read_dir(&build_tools_path)
            .map_err(|_| Error::PathNotFound(build_tools_path.clone()))?
            .filter_map(|path| path.ok())
            .filter(|path| path.path().is_dir())
            .filter_map(|path| path.file_name().into_string().ok())
            .filter(|name| name.chars().next().unwrap().is_digit(10))
            .max()
            .ok_or(AndroidError::BuildToolsNotFound)?;
        let platforms_path = sdk_path.join("platforms");
        let platforms: Vec<u32> = std::fs::read_dir(&platforms_path)
            .map_err(|_| Error::PathNotFound(platforms_path.clone()))?
            .filter_map(|path| path.ok())
            .filter(|path| path.path().is_dir())
            .filter_map(|path| path.file_name().into_string().ok())
            .filter_map(|name| {
                name.strip_prefix("android-")
                    .and_then(|api| api.parse::<u32>().ok())
            })
            .collect();
        if platforms.is_empty() {
            return Err(AndroidError::NoPlatformsFound.into());
        };
        Ok(Self {
            sdk_path,
            build_tools_path,
            build_tools_version,
            platforms_path,
            platforms,
        }
        .into())
    }

    pub fn sdk_path(&self) -> PathBuf {
        self.sdk_path.clone()
    }

    pub fn build_tools_path(&self) -> PathBuf {
        self.build_tools_path.clone()
    }

    pub fn build_tools_version(&self) -> &str {
        &self.build_tools_version
    }

    pub fn platforms_path(&self) -> PathBuf {
        self.platforms_path.clone()
    }

    pub fn platforms(&self) -> &[u32] {
        &self.platforms
    }

    pub fn build_tool(&self, tool: &str) -> Result<ProcessCommand> {
        let path = self
            .build_tools_path
            .join(&self.build_tools_version)
            .join(tool);
        if !path.exists() {
            return Err(Error::CmdNotFound(tool.to_string()));
        }
        Ok(ProcessCommand::new(dunce::canonicalize(path)?))
    }

    pub fn platform_tool(&self, tool: &str) -> Result<ProcessCommand> {
        let path = self.sdk_path.join("platform-tools").join(tool);
        if !path.exists() {
            return Err(Error::CmdNotFound(tool.to_string()));
        }
        Ok(ProcessCommand::new(dunce::canonicalize(path)?))
    }

    pub fn default_platform(&self) -> u32 {
        self.platforms().iter().max().cloned().unwrap()
    }

    pub fn platform_dir(&self, platform: u32) -> Result<PathBuf> {
        let dir = self.platforms_path.join(format!("android-{}", platform));
        if !dir.exists() {
            return Err(AndroidError::PlatformNotFound(platform).into());
        }
        Ok(dir)
    }

    pub fn android_jar(&self, platform: u32) -> Result<PathBuf> {
        let android_jar = self.platform_dir(platform)?.join("android.jar");
        if !android_jar.exists() {
            return Err(Error::PathNotFound(android_jar));
        }
        Ok(android_jar)
    }
}

enum AndroidSdkChecks {
    Found,
    EnvVarsAreSet,
}

impl IntoCheckInfo for AndroidSdkChecks {
    fn check_passed(self) -> CheckInfo {
        match self {
            AndroidSdkChecks::Found => CheckInfo {
                dependency_name: "Android SDK".to_owned(),
                check_name: "Android SDK found".to_owned(),
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
