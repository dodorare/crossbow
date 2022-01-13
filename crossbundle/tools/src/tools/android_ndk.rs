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
    pub fn from_env(sdk_path: Option<&Path>) -> Result<Self> {
        let ndk_path = {
            let ndk_path = std::env::var("ANDROID_NDK_ROOT")
                .ok()
                .or_else(|| std::env::var("ANDROID_NDK_PATH").ok())
                .or_else(|| std::env::var("ANDROID_NDK_HOME").ok())
                .or_else(|| std::env::var("NDK_HOME").ok());
            // Default ndk installation path
            if ndk_path.is_none()
                && sdk_path.is_some()
                && sdk_path.as_ref().unwrap().join("ndk-bundle").exists()
            {
                sdk_path.unwrap().join("ndk-bundle")
            } else {
                PathBuf::from(ndk_path.ok_or(AndroidError::AndroidNdkNotFound)?)
            }
        };
        let build_tag = std::fs::read_to_string(ndk_path.join("source.properties"))
            .expect("Failed to read source.properties");
        let build_tag = build_tag
            .split('\n')
            .find_map(|line| {
                let (key, value) = line
                    .split_once('=')
                    .expect("Failed to parse `key = value` from source.properties");
                if key.trim() == "Pkg.Revision" {
                    // AOSP writes a constantly-incrementing build version to the patch field.
                    // This number is incrementing across NDK releases.
                    let mut parts = value.trim().split('.');
                    let _major = parts.next().unwrap();
                    let _minor = parts.next().unwrap();
                    let patch = parts.next().unwrap();
                    // Can have an optional `XXX-beta1`
                    let patch = patch.split_once('-').map_or(patch, |(patch, _beta)| patch);
                    Some(patch.parse().expect("Failed to parse patch field"))
                } else {
                    None
                }
            })
            .expect("No `Pkg.Revision` in source.properties");
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
                .then(|| llvm_path)
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

    /// Sysroot and lib path
    pub fn sysroot_lib_dir(&self, build_target: AndroidTarget) -> Result<PathBuf> {
        let sysroot_lib_dir = self
            .toolchain_dir()?
            .join("sysroot")
            .join("usr")
            .join("lib")
            .join(build_target.ndk_triple());
        if !sysroot_lib_dir.exists() {
            return Err(Error::PathNotFound(sysroot_lib_dir));
        }
        Ok(sysroot_lib_dir)
    }

    /// Sysroot and lib platform
    pub fn sysroot_platform_lib_dir(
        &self,
        build_target: AndroidTarget,
        min_sdk_version: u32,
    ) -> Result<PathBuf> {
        let sysroot_lib_dir = self.sysroot_lib_dir(build_target)?;
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
}
