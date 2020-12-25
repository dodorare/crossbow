use super::{AndroidManifest, Command};
use crate::deps::*;
use crate::error::*;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::rc::Rc;

pub struct GenUnalignedApk {
    sdk: Rc<AndroidSdk>,
    build_dir: PathBuf,
    assets: Option<PathBuf>,
    res: Option<String>,
    manifest: AndroidManifest,
}

impl GenUnalignedApk {
    pub fn new(
        sdk: Rc<AndroidSdk>,
        build_dir: PathBuf,
        assets: Option<PathBuf>,
        res: Option<String>,
        manifest: AndroidManifest,
    ) -> Self {
        Self {
            sdk,
            build_dir,
            assets,
            res,
            manifest,
        }
    }
}

impl Command for GenUnalignedApk {
    type Deps = AndroidSdk;
    type Output = UnalignedApk;

    fn run(&self) -> Result<Self::Output> {
        if !self.build_dir.exists() {
            create_dir_all(&self.build_dir)?;
        }
        let apk_path = self
            .build_dir
            .join(format!("{}-unaligned.apk", self.manifest.package_label));
        let mut aapt = self.sdk.build_tool(bin!("aapt"))?;
        aapt.arg("package")
            .arg("-f")
            .arg("-F")
            .arg(&apk_path)
            .arg("-M")
            .arg("AndroidManifest.xml")
            .arg("-I")
            .arg(self.sdk.android_jar(self.manifest.target_sdk_version)?);
        if let Some(res) = &self.res {
            aapt.arg("-S").arg(res);
        }
        if let Some(assets) = &self.assets {
            aapt.arg("-A").arg(dunce::simplified(assets));
        }

        if !aapt.status()?.success() {
            return Err(Error::CmdFailed(aapt));
        }
        Ok(UnalignedApk {
            build_dir: self.build_dir.clone(),
            apk_path,
            package_label: self.manifest.package_label.clone(),
        })
    }
}

pub struct UnalignedApk {
    pub build_dir: PathBuf,
    pub apk_path: PathBuf,
    pub package_label: String,
}
