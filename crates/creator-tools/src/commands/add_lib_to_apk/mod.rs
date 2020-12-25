use super::Command;
use crate::{deps::*, error::*, types::AndroidTarget};
use std::path::PathBuf;
use std::rc::Rc;

pub struct AddLibToApk {
    sdk: Rc<AndroidSdk>,
    build_dir: PathBuf,
    unaligned_apk_path: PathBuf,
    lib_path: PathBuf,
    build_target: AndroidTarget,
}

impl AddLibToApk {
    pub fn new(
        sdk: Rc<AndroidSdk>,
        build_dir: PathBuf,
        unaligned_apk_path: PathBuf,
        lib_path: PathBuf,
        build_target: AndroidTarget,
    ) -> Self {
        Self {
            sdk,
            build_dir,
            unaligned_apk_path,
            lib_path,
            build_target,
        }
    }
}

impl Command for AddLibToApk {
    type Deps = AndroidSdk;
    type Output = ();

    fn run(&self) -> Result<Self::Output> {
        if !self.lib_path.exists() {
            return Err(Error::PathNotFound(self.lib_path.clone()));
        }
        let abi = self.build_target.android_abi();
        let file_name = self.lib_path.file_name().unwrap();
        let out = self.build_dir.join("lib").join(abi);
        std::fs::create_dir_all(&out)?;
        std::fs::copy(self.lib_path.clone(), out.join(&file_name))?;

        let mut aapt = self.sdk.build_tool(bin!("aapt"))?;
        aapt.arg("add").arg(&self.unaligned_apk_path).arg(format!(
            "lib/{}/{}",
            abi,
            file_name.to_str().unwrap()
        ));
        if !aapt.status()?.success() {
            return Err(Error::CmdFailed(aapt));
        }
        Ok(())
    }
}
