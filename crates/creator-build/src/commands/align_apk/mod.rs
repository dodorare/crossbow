use super::{Command, UnalignedApk};
use crate::deps::*;
use crate::error::*;
use std::path::PathBuf;
use std::rc::Rc;

pub struct AlignApk {
    sdk: Rc<AndroidSdk>,
    unaligned_apk: UnalignedApk,
}

impl AlignApk {
    pub fn new(sdk: Rc<AndroidSdk>, unaligned_apk: UnalignedApk) -> Self {
        Self { sdk, unaligned_apk }
    }
}

impl Command for AlignApk {
    type Deps = AndroidSdk;
    type Output = UnsignedApk;

    fn run(&self) -> Result<Self::Output> {
        let unsigned_apk_path = self
            .unaligned_apk
            .build_dir
            .join(format!("{}.apk", self.unaligned_apk.package_label));
        let mut zipalign = self.sdk.build_tool(bin!("zipalign"))?;
        zipalign
            .arg("-f")
            .arg("-v")
            .arg("4")
            .arg(&self.unaligned_apk.apk_path)
            .arg(&unsigned_apk_path);
        if !zipalign.status()?.success() {
            return Err(Error::CmdFailed(zipalign));
        }
        Ok(UnsignedApk {
            apk_path: unsigned_apk_path,
        })
    }
}

pub struct UnsignedApk {
    pub apk_path: PathBuf,
}
