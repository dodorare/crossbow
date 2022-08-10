use crate::error::*;
use crate::types::AndroidTarget;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Helper structure that contains information about the Android NDK Path
/// and returns paths to the tools.
#[derive(Debug, Clone)]
pub struct AndroidNdk {
    ndk_path: PathBuf,
    build_tag: u32,
}

impl AndroidNdk {
    /// Using environment variables
    pub fn from_env(sdk_path: &Path) -> Result<Self> {
        let ndk_path = {
            let ndk_path = std::env::var("ANDROID_NDK_ROOT")
                .ok()
                .or_else(|| std::env::var("ANDROID_NDK_PATH").ok())
                .or_else(|| std::env::var("ANDROID_NDK_HOME").ok())
                .or_else(|| std::env::var("NDK_HOME").ok());
            // Default ndk installation path
            if let Some(ndk_path) = ndk_path {
                PathBuf::from(ndk_path)
            } else if ndk_path.is_none() && sdk_path.join("ndk-bundle").exists() {
                sdk_path.join("ndk-bundle")
            } else {
                let ndk_path = sdk_path.join("ndk");
                let ndk_ver = std::fs::read_dir(&ndk_path)
                    .map_err(|_| Error::PathNotFound(ndk_path.clone()))?
                    .filter_map(|path| path.ok())
                    .filter(|path| path.path().is_dir())
                    .filter_map(|path| path.file_name().into_string().ok())
                    .filter(|name| name.chars().next().unwrap().is_ascii_digit())
                    .max()
                    .ok_or(AndroidError::AndroidNdkNotFound)?;
                ndk_path.join(ndk_ver)
            }
        };
        let build_tag = std::fs::read_to_string(ndk_path.join("source.properties"))
            .map_err(|_| AndroidError::FailedToReadSourceProperties)?;
        let build_tag = build_tag
            .split('\n')
            .find_map(|line| {
                if let Some((key, value)) = line.split_once('=') {
                    if key.trim() == "Pkg.Revision" {
                        // AOSP writes a constantly-incrementing build version to the patch field.
                        // This number is incrementing across NDK releases.
                        let mut parts = value.trim().split('.');
                        let _major = parts.next().unwrap();
                        let _minor = parts.next().unwrap();
                        let patch = parts.next().unwrap();
                        // Can have an optional `XXX-beta1`
                        let patch = patch.split_once('-').map_or(patch, |(patch, _beta)| patch);
                        return Some(patch.parse().expect("Failed to parse patch field"));
                    }
                }
                None
            })
            .ok_or_else(|| {
                AndroidError::InvalidSourceProperties(
                    "No `Pkg.Revision` in source.properties".to_owned(),
                )
            })?;
        Ok(Self {
            ndk_path,
            build_tag,
        })
    }

    /// Build tag
    pub fn build_tag(&self) -> u32 {
        self.build_tag
    }

    /// NDK path
    pub fn ndk_path(&self) -> &Path {
        &self.ndk_path
    }

    /// Operating system type
    pub fn toolchain_dir(&self) -> Result<PathBuf> {
        let host_os = std::env::var("HOST").ok();
        let host_contains = |s| host_os.as_ref().map(|h| h.contains(s)).unwrap_or(false);
        let arch = if host_contains("linux") {
            "linux"
        } else if host_contains("macos") {
            "darwin"
        } else if host_contains("windows") {
            "windows"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "darwin"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            return match host_os {
                Some(host_os) => Err(AndroidError::UnsupportedHost(host_os)),
                _ => Err(AndroidError::UnsupportedTarget),
            }?;
        };
        let mut toolchain_dir = self
            .ndk_path
            .join("toolchains")
            .join("llvm")
            .join("prebuilt")
            .join(format!("{}-x86_64", arch));
        if !toolchain_dir.exists() {
            toolchain_dir.set_file_name(arch);
        }
        if !toolchain_dir.exists() {
            return Err(Error::PathNotFound(toolchain_dir));
        }
        Ok(toolchain_dir)
    }

    /// Path to Clang
    pub fn clang(&self, target: AndroidTarget, platform: u32) -> Result<(PathBuf, PathBuf)> {
        #[cfg(target_os = "windows")]
        let ext = ".cmd";
        #[cfg(not(target_os = "windows"))]
        let ext = "";
        let bin_name = format!("{}{}-clang", target.ndk_llvm_triple(), platform);
        let bin_path = self.toolchain_dir()?.join("bin");
        let clang = bin_path.join(format!("{}{}", &bin_name, ext));
        if !clang.exists() {
            return Err(Error::PathNotFound(clang));
        }
        let clang_pp = bin_path.join(format!("{}++{}", &bin_name, ext));
        if !clang_pp.exists() {
            return Err(Error::PathNotFound(clang_pp));
        }
        Ok((clang, clang_pp))
    }

    /// Path to bin
    pub fn toolchain_bin(&self, name: &str, build_target: AndroidTarget) -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        let ext = ".exe";
        #[cfg(not(target_os = "windows"))]
        let ext = "";
        let toolchain_path = self.toolchain_dir()?.join("bin");
        // Since r21 (https://github.com/android/ndk/wiki/Changelog-r21) LLVM binutils are included _for testing_;
        // Since r22 (https://github.com/android/ndk/wiki/Changelog-r22) GNU binutils are deprecated in favour of LLVM's;
        // Since r23 (https://github.com/android/ndk/wiki/Changelog-r23) GNU binutils have been removed.
        // To maintain stability with the current ndk-build crate release, prefer GNU binutils for
        // as long as it is provided by the NDK instead of trying to use llvm-* from r21 onwards.
        let gnu_bin = format!("{}-{}{}", build_target.ndk_triple(), name, ext);
        let gnu_path = toolchain_path.join(&gnu_bin);
        if gnu_path.exists() {
            Ok(gnu_path)
        } else {
            let llvm_bin = format!("llvm-{}{}", name, ext);
            let llvm_path = toolchain_path.join(&llvm_bin);
            llvm_path
                .exists()
                .then_some(llvm_path)
                .ok_or(Error::ToolchainBinaryNotFound {
                    toolchain_path,
                    gnu_bin,
                    llvm_bin,
                })
        }
    }

    /// Displaying various information
    pub fn readelf(&self, build_target: AndroidTarget) -> Result<Command> {
        let readelf_path = self.toolchain_bin("readelf", build_target)?;
        Ok(Command::new(readelf_path))
    }

    /// Sysroot and lib platform
    pub fn sysroot_platform_lib_dir(
        &self,
        build_target: AndroidTarget,
        min_sdk_version: u32,
    ) -> Result<PathBuf> {
        let sysroot_lib_dir = self.sysroot_lib_dir(&build_target)?;
        // Look for a platform <= min_sdk_version
        let mut tmp_platform = min_sdk_version;
        while tmp_platform > 1 {
            let path = sysroot_lib_dir.join(tmp_platform.to_string());
            if path.exists() {
                return Ok(path);
            }
            tmp_platform += 1;
        }
        // Look for the minimum API level supported by the NDK
        let mut tmp_platform = min_sdk_version;
        while tmp_platform < 100 {
            let path = sysroot_lib_dir.join(tmp_platform.to_string());
            if path.exists() {
                return Ok(path);
            }
            tmp_platform += 1;
        }
        Err(AndroidError::PlatformNotFound(min_sdk_version).into())
    }

    /// Helper function for looking for a path based on the platform version
    /// Calls a closure for each attempt and then return the PathBuf for the first file
    /// that exists. Uses approach that NDK build tools use which is described at:
    /// https://developer.android.com/ndk/guides/application_mk
    /// " - The platform version matching APP_PLATFORM.
    ///   - The next available API level below APP_PLATFORM. For example, android-19 will
    ///     be used when APP_PLATFORM is android-20, since there were no new native APIs
    ///     in android-20.
    ///   - The minimum API level supported by the NDK."
    pub fn find_ndk_path<F>(platform: u32, path_builder: F) -> Result<PathBuf>
    where
        F: Fn(u32) -> PathBuf,
    {
        let mut tmp_platform = platform;
        // Look for the file which matches the specified platform
        // If that doesn't exist, look for a lower version
        while tmp_platform > 1 {
            let path = path_builder(tmp_platform);
            if path.exists() {
                return Ok(path);
            }
            tmp_platform -= 1;
        }
        // If that doesn't exist... Look for a higher one. This would be the minimum API level
        // supported by the NDK
        tmp_platform = platform;
        while tmp_platform < 100 {
            let path = path_builder(tmp_platform);
            if path.exists() {
                return Ok(path);
            }

            tmp_platform += 1;
        }
        Err(AndroidError::UnableToFindNDKFile.into())
    }

    /// Return tool root from the toolchain directory
    pub fn tool_root(&self) -> cargo::CargoResult<PathBuf> {
        let tool_root = self
            .toolchain_dir()
            .map_err(|_| anyhow::Error::msg("The path to tool root not found"))?;
        Ok(tool_root)
    }

    /// Return path to linker
    pub fn linker_path(&self, build_target: &AndroidTarget) -> cargo::CargoResult<PathBuf> {
        let linker = bin!("ld.gold");
        let linker_path = self
            .tool_root()?
            .join(build_target.ndk_triple())
            .join("bin")
            .join(linker);
        if !linker_path.exists() {
            return Err(anyhow::Error::msg(format!(
                "The path to the {} not found",
                linker_path.to_string_lossy()
            )));
        }
        Ok(linker_path)
    }

    /// Return path to gcc library
    pub fn gcc_lib_path(&self, build_target: &AndroidTarget) -> cargo::CargoResult<PathBuf> {
        let triple = build_target.ndk_triple();
        let gcc_lib_path = self
            .tool_root()?
            .join("lib")
            .join("gcc")
            .join(triple)
            .join("4.9.x");
        if !gcc_lib_path.exists() {
            return Err(anyhow::Error::msg(format!(
                "The path to {} not found",
                gcc_lib_path.to_string_lossy()
            )));
        }
        Ok(gcc_lib_path)
    }

    /// Return path to sysroot
    pub fn sysroot(&self) -> cargo::CargoResult<PathBuf> {
        let sysroot = self.tool_root()?.join("sysroot");
        if !sysroot.exists() {
            return Err(anyhow::Error::msg(format!(
                "The path to {} not found",
                sysroot.to_string_lossy()
            )));
        }
        Ok(sysroot)
    }

    /// Return path to sysroot library
    pub fn sysroot_lib_dir(&self, build_target: &AndroidTarget) -> Result<PathBuf> {
        let sysroot_lib_dir = self
            .toolchain_dir()?
            .join(self.sysroot()?)
            .join("usr")
            .join("lib")
            .join(build_target.ndk_triple());
        if !sysroot_lib_dir.exists() {
            return Err(Error::PathNotFound(sysroot_lib_dir));
        }
        Ok(sysroot_lib_dir)
    }

    /// Return path to version specific libraries
    pub fn version_specific_libraries_path(
        &self,
        target_sdk_version: u32,
        build_target: &AndroidTarget,
    ) -> cargo::CargoResult<PathBuf> {
        let version_specific_libraries_path = Self::find_ndk_path(target_sdk_version, |plarform| {
            self.sysroot_lib_dir(build_target)
                .map_err(|_| {
                    self.sysroot_lib_dir(build_target).unwrap();
                })
                .unwrap()
                .join(plarform.to_string())
        })
        .map_err(|_| anyhow::Error::msg("Failed to get access to the ndk path"))?;

        if !version_specific_libraries_path.exists() {
            return Err(anyhow::Error::msg(format!(
                "The path to {} not found",
                version_specific_libraries_path.to_string_lossy()
            )));
        }
        Ok(version_specific_libraries_path)
    }
}
