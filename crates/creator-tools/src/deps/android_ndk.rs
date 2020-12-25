use super::*;
use crate::error::*;
use crate::types::AndroidTarget;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AndroidNdk {
    pub ndk_path: PathBuf,
}

impl Checks for AndroidNdk {
    fn check() -> Result<Vec<CheckInfo>> {
        Ok(Vec::new())
    }
}

impl AndroidNdk {
    pub fn init(sdk_path: Option<PathBuf>) -> Result<Rc<Self>> {
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
        Ok(Self { ndk_path }.into())
    }

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

    pub fn toolchain_bin(&self, bin: &str, target: AndroidTarget) -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        let ext = ".exe";
        #[cfg(not(target_os = "windows"))]
        let ext = "";
        let bin = self.toolchain_dir()?.join("bin").join(format!(
            "{}-{}{}",
            target.ndk_triple(),
            bin,
            ext
        ));
        if !bin.exists() {
            return Err(Error::PathNotFound(bin));
        }
        Ok(bin)
    }
}
