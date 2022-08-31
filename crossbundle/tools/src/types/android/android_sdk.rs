use crate::{error::*, types::Aapt2};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

/// Helper structure that contains information about the Android SDK path
/// and returns paths to the tools.
#[derive(Debug, Default)]
pub struct AndroidSdk {
    sdk_path: PathBuf,
    build_deps_path: PathBuf,
    build_deps_version: String,
    platforms_path: PathBuf,
    platforms: Vec<u32>,
}

impl AndroidSdk {
    /// Using environment variables tools
    pub fn from_env() -> Result<Self> {
        let sdk_path = android_sdk_path()?;
        let build_deps_path = sdk_path.join("build-tools");
        let build_deps_version = std::fs::read_dir(&build_deps_path)
            .map_err(|_| Error::PathNotFound(build_deps_path.clone()))?
            .filter_map(|path| path.ok())
            .filter(|path| path.path().is_dir())
            .filter_map(|path| path.file_name().into_string().ok())
            .filter(|name| name.chars().next().unwrap().is_ascii_digit())
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
            build_deps_path,
            build_deps_version,
            platforms_path,
            platforms,
        })
    }

    /// Path to SDK
    pub fn sdk_path(&self) -> &Path {
        &self.sdk_path
    }

    /// Build path deps
    pub fn build_deps_path(&self) -> &Path {
        &self.build_deps_path
    }

    /// Build version deps
    pub fn build_deps_version(&self) -> &str {
        &self.build_deps_version
    }

    /// Platforms path
    pub fn platforms_path(&self) -> &Path {
        &self.platforms_path
    }

    /// Platforms
    pub fn platforms(&self) -> &[u32] {
        &self.platforms
    }

    /// Provides path to SDK tool
    pub fn build_tool(&self, tool: &str, current_dir: Option<&Path>) -> Result<ProcessCommand> {
        let path = self
            .build_deps_path
            .join(&self.build_deps_version)
            .join(tool);
        if !path.exists() {
            return Err(Error::CmdNotFound(tool.to_string()));
        }
        let mut command = ProcessCommand::new(dunce::canonicalize(path)?);
        if let Some(current_dir) = current_dir {
            command.current_dir(current_dir);
        };
        Ok(command)
    }

    /// AAPT2 tools
    pub fn aapt2(&self) -> Result<Aapt2> {
        self.build_tool(bin!("aapt2"), None)?;
        Ok(Aapt2)
    }

    /// Platforms tools
    pub fn platform_tool(&self, tool: &str) -> Result<ProcessCommand> {
        let path = self.sdk_path.join("platform-tools").join(tool);
        if !path.exists() {
            return Err(Error::CmdNotFound(tool.to_string()));
        }
        Ok(ProcessCommand::new(dunce::canonicalize(path)?))
    }

    /// Default platforms
    pub fn default_platform(&self) -> u32 {
        self.platforms().iter().max().cloned().unwrap()
    }

    /// Platforms directory path
    pub fn platform_dir(&self, platform: u32) -> Result<PathBuf> {
        let dir = self.platforms_path.join(format!("android-{}", platform));
        if !dir.exists() {
            return Err(AndroidError::PlatformNotFound(platform).into());
        }
        Ok(dir)
    }

    /// Returns android_jar path
    pub fn android_jar(&self, platform: u32) -> Result<PathBuf> {
        let android_jar = self.platform_dir(platform)?.join("android.jar");
        if !android_jar.exists() {
            return Err(Error::PathNotFound(android_jar));
        }
        Ok(android_jar)
    }
}

/// Get path to android sdk
pub fn android_sdk_path() -> Result<PathBuf> {
    let sdk_path = {
        let sdk_path = std::env::var("ANDROID_SDK_ROOT")
            .ok()
            .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
            .or_else(|| std::env::var("ANDROID_HOME").ok());
        if let Some(sdk_path) = sdk_path {
            PathBuf::from(sdk_path)
        } else {
            android_tools::sdk_install_path()?
        }
    };
    Ok(sdk_path)
}
