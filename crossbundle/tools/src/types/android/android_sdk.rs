use crate::{error::*, types::Aapt2};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

/// Helper structure that contains information about the Android SDK path
/// and returns paths to the tools.
#[derive(Clone, Debug, Default)]
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
        Self::from_path(sdk_path)
    }

    /// Loads an SDK from an already-resolved root without consulting the environment.
    pub fn from_path(sdk_path: PathBuf) -> Result<Self> {
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

    /// Loads the exact SDK components selected during planning.
    pub fn from_resolved(sdk_path: PathBuf, build_tools: &Path, platform: &Path) -> Result<Self> {
        let build_deps_path = build_tools
            .parent()
            .ok_or_else(|| Error::PathNotFound(build_tools.to_owned()))?
            .to_owned();
        let build_deps_version = build_tools
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::PathNotFound(build_tools.to_owned()))?
            .to_owned();
        let platforms_path = platform
            .parent()
            .ok_or_else(|| Error::PathNotFound(platform.to_owned()))?
            .to_owned();
        let platform = platform
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_prefix("android-"))
            .and_then(|api| api.parse().ok())
            .ok_or(AndroidError::NoPlatformsFound)?;
        Ok(Self {
            sdk_path,
            build_deps_path,
            build_deps_version,
            platforms_path,
            platforms: vec![platform],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_sdk_keeps_the_planned_component_versions() {
        let temp = tempfile::tempdir().unwrap();
        let build_tools = temp.path().join("build-tools/36.0.0");
        let platform = temp.path().join("platforms/android-36");
        std::fs::create_dir_all(&build_tools).unwrap();
        std::fs::create_dir_all(&platform).unwrap();

        let sdk = AndroidSdk::from_resolved(temp.path().into(), &build_tools, &platform).unwrap();
        assert_eq!(sdk.build_deps_version(), "36.0.0");
        assert_eq!(sdk.default_platform(), 36);
    }
}
