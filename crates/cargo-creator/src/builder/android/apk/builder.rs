use super::manifest::*;
use crate::builder::android::error::*;
use crate::builder::android::metadata::AndroidMetadata;
use crate::builder::android::ndk_build::Ndk;
use crate::builder::android::target::AndroidTarget;
use crate::builder::shared::*;

use std::path::PathBuf;

pub struct ApkBuilder {
    ndk: Ndk,
    artifacts: Vec<Artifact>,
    build_targets: Vec<AndroidTarget>,
    build_dir: PathBuf,
    version_name: String,
    version_code: String,
    profile: Profile,
    assets_path: PathBuf,
    res_path: String,
    metadata: AndroidMetadata,
}

impl ApkBuilder {
    // pub fn from_cargo_manifest(manifest: CargoManifest) -> Result<Self, NdkError> {
    //     let ndk = Ndk::from_env()?;
    //     let artifacts = Self::take_default_artifacts(&manifest);
    //     let mut build_targets = Self::take_build_targets(&manifest);
    //     if build_targets.is_empty() {
    //         build_targets.push(AndroidTarget::ArmV7a);
    //     };

    //     Ok(Self {
    //         ndk,
    //         artifacts,
    //         build_targets,
    //         // build_dir: None,
    //         // version_name: None,
    //         // version_code: None,
    //         profile: Profile::Dev,
    //         // assets: None,
    //         // res: None,
    //         // metadata: None,
    //     })
    // }

    pub fn build(&self) -> Result<Apk, NdkError> {
        // 1. Init AndroidManifest struct
        // 2. Create build_dir path
        // 3. Write AndroidManifest.xml into build_dir
        // 4. Create unaligned apk file using `aapt` tool

        let target_sdk_version = self
            .metadata
            .target_sdk_version
            .clone()
            .unwrap_or_else(|| self.ndk.default_platform());
        let features = self
            .metadata
            .feature
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let permissions = self
            .metadata
            .permission
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let intent_filters = self
            .metadata
            .clone()
            .intent_filter
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let application_metadatas = self
            .metadata
            .clone()
            .application_metadatas
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let activity_metadatas = self
            .metadata
            .clone()
            .activity_metadatas
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        let manifest = AndroidManifest {
            package_name: "".to_owned(),
            package_label: "".to_owned(),
            version_name: "".to_owned(),
            version_code: 1,
            split: Some("".to_owned()),
            target_name: "".to_owned(),
            debuggable: false,
            target_sdk_version,
            min_sdk_version: self.metadata.min_sdk_version.unwrap_or(23),
            opengles_version: self.metadata.opengles_version.unwrap_or((3, 1)),
            features,
            permissions,
            intent_filters,
            icon: self.metadata.icon.clone(),
            fullscreen: self.metadata.fullscreen.unwrap_or(false),
            orientation: self.metadata.orientation.clone(),
            application_metadatas,
            activity_metadatas,
        };

        // Ok(apk.align()?.sign(config.ndk.debug_key()?)?)
        Ok(Apk)
    }

    /// Defines default artifacts from given cargo manifest
    fn take_default_artifacts(manifest: &CargoManifest) -> Vec<Artifact> {
        let mut artifacts = Vec::new();
        let lib_name = manifest
            .lib
            .as_ref()
            .and_then(|lib| lib.name.clone())
            .unwrap_or_else(|| manifest.package.as_ref().unwrap().name.clone());
        artifacts.push(Artifact::Root(lib_name));
        artifacts
    }

    /// Defines build_targets from given cargo manifest
    fn take_build_targets(manifest: &CargoManifest) -> Vec<AndroidTarget> {
        let mut build_targets = Vec::new();
        if let Some(package) = manifest.package.as_ref() {
            if let Some(metadata) = package.metadata.as_ref() {
                build_targets = metadata.android.build_targets.clone();
            };
        };
        build_targets
    }

    // pub fn cli(mut self, cli: CliBuildAndroid) -> Result<Self, NdkError> {
    //     let mut artifacts = Vec::new();
    //     artifacts.append(
    //         &mut cli
    //             .cargo
    //             .bin
    //             .into_iter()
    //             .map(|v| Artifact::Root(v))
    //             .collect(),
    //     );
    //     artifacts.append(
    //         &mut cli
    //             .cargo
    //             .example
    //             .into_iter()
    //             .map(|v| Artifact::Example(v))
    //             .collect(),
    //     );
    //     if !artifacts.is_empty() {
    //         self.artifacts = Some(artifacts);
    //     };
    //     let mut build_targets = Vec::new();
    //     for target in cli.cargo.target {
    //         let build_target = AndroidTarget::from_rust_triple(&target)?;
    //         build_targets.push(build_target);
    //     }
    //     self.build_targets = build_targets;
    //     if cli.cargo.release {
    //         self.profile = Profile::Release;
    //     };
    //     // self.build_dir = Some(
    //     //     dunce::simplified(cli.cargo.target_dir)
    //     //         .join(self.profile)
    //     //         .join("apk"),
    //     // );
    //     Ok(self)
    // }

    // pub fn manifest(mut self, manifest: CargoManifest) -> Result<Self, NdkError> {
    //     if self.build_targets.is_empty() {
    //         if let Some(package) = manifest.package {
    //             if let Some(metadata) = package.metadata {
    //                 for target in metadata.android.build_targets {
    //                     self.build_targets.push(target);
    //                 }
    //             };
    //         };
    //     };
    //     // Todo: take only bin|examples from all `cargo_toml::Product`
    //     Ok(self)
    // }
}

pub struct Apk;

// use super::error::NdkError;
// use super::manifest::ApkManifest;
// use super::metadata::Metadata;
// use super::ndk_build::{Key, Ndk};
// use super::target::AndroidTarget;
// use std::path::{Path, PathBuf};
// use std::process::Command;

// pub struct ApkConfig {
//     pub ndk: Ndk,
//     pub build_dir: PathBuf,
//     pub assets: Option<PathBuf>,
//     pub res: Option<String>,
//     pub manifest: ApkManifest,
// }

// impl ApkConfig {
//     pub fn from_config(config: Config, metadata: Metadata) -> Self {
//         let target_sdk_version = metadata
//             .target_sdk_version
//             .unwrap_or_else(|| config.ndk.default_platform());
//         let features = metadata
//             .feature
//             .unwrap_or_default()
//             .into_iter()
//             .map(Into::into)
//             .collect();
//         let permissions = metadata
//             .permission
//             .unwrap_or_default()
//             .into_iter()
//             .map(Into::into)
//             .collect();
//         let intent_filters = metadata
//             .intent_filter
//             .unwrap_or_default()
//             .into_iter()
//             .map(Into::into)
//             .collect();
//         let application_metadatas = metadata
//             .application_metadatas
//             .unwrap_or_default()
//             .into_iter()
//             .map(Into::into)
//             .collect();
//         let activity_metadatas = metadata
//             .activity_metadatas
//             .unwrap_or_default()
//             .into_iter()
//             .map(Into::into)
//             .collect();

//         let manifest = ApkManifest {
//             package_name: config.package_name,
//             package_label: config.package_label,
//             version_name: config.version_name,
//             version_code: config.version_code,
//             split: config.split,
//             target_name: config.target_name,
//             debuggable: config.debuggable,
//             target_sdk_version,
//             min_sdk_version: metadata.min_sdk_version.unwrap_or(23),
//             opengles_version: metadata.opengles_version.unwrap_or((3, 1)),
//             features,
//             permissions,
//             intent_filters,
//             icon: metadata.icon,
//             fullscreen: metadata.fullscreen.unwrap_or(false),
//             orientation: metadata.orientation,
//             application_metadatas,
//             activity_metadatas,
//         };
//         Self {
//             ndk: config.ndk,
//             build_dir: config.build_dir,
//             assets: config.assets,
//             res: config.res,
//             manifest,
//         }
//     }

//     fn build_tool(&self, tool: &'static str) -> Result<Command, NdkError> {
//         let mut cmd = self.ndk.build_tool(tool)?;
//         cmd.current_dir(&self.build_dir);
//         Ok(cmd)
//     }

//     fn unaligned_apk(&self) -> PathBuf {
//         self.build_dir
//             .join(format!("{}-unaligned.apk", self.manifest.package_label))
//     }

//     fn apk(&self) -> PathBuf {
//         self.build_dir
//             .join(format!("{}.apk", self.manifest.package_label))
//     }

//     pub fn create_apk(&self) -> Result<UnalignedApk, NdkError> {
//         std::fs::create_dir_all(&self.build_dir)?;
//         self.manifest.write_to(&self.build_dir)?;

//         let mut aapt = self.build_tool(bin!("aapt"))?;
//         aapt.arg("package")
//             .arg("-f")
//             .arg("-F")
//             .arg(self.unaligned_apk())
//             .arg("-M")
//             .arg("AndroidManifest.xml")
//             .arg("-I")
//             .arg(self.ndk.android_jar(self.manifest.target_sdk_version)?);

//         if let Some(res) = &self.res {
//             aapt.arg("-S").arg(res);
//         }

//         if let Some(assets) = &self.assets {
//             aapt.arg("-A").arg(dunce::simplified(assets));
//         }

//         if !aapt.status()?.success() {
//             return Err(NdkError::CmdFailed(aapt));
//         }

//         Ok(UnalignedApk(self))
//     }
// }

// pub struct UnalignedApk<'a>(&'a ApkConfig);

// impl<'a> UnalignedApk<'a> {
//     pub fn config(&self) -> &ApkConfig {
//         self.0
//     }

//     pub fn add_lib(&self, path: &Path, target: AndroidTarget) -> Result<(), NdkError> {
//         if !path.exists() {
//             return Err(NdkError::PathNotFound(path.into()));
//         }
//         let abi = target.android_abi();
//         let file_name = path.file_name().unwrap();
//         let out = self.0.build_dir.join("lib").join(abi);
//         std::fs::create_dir_all(&out)?;
//         std::fs::copy(path, out.join(&file_name))?;

//         let mut aapt = self.0.build_tool(bin!("aapt"))?;
//         aapt.arg("add").arg(self.0.unaligned_apk()).arg(format!(
//             "lib/{}/{}",
//             abi,
//             file_name.to_str().unwrap()
//         ));
//         if !aapt.status()?.success() {
//             return Err(NdkError::CmdFailed(aapt));
//         }
//         Ok(())
//     }

//     pub fn align(self) -> Result<UnsignedApk<'a>, NdkError> {
//         let mut zipalign = self.0.build_tool(bin!("zipalign"))?;
//         zipalign
//             .arg("-f")
//             .arg("-v")
//             .arg("4")
//             .arg(self.0.unaligned_apk())
//             .arg(self.0.apk());
//         if !zipalign.status()?.success() {
//             return Err(NdkError::CmdFailed(zipalign));
//         }
//         Ok(UnsignedApk(self.0))
//     }
// }

// pub struct UnsignedApk<'a>(&'a ApkConfig);

// impl<'a> UnsignedApk<'a> {
//     pub fn sign(self, key: Key) -> Result<Apk, NdkError> {
//         let mut apksigner = self.0.build_tool(bat!("apksigner"))?;
//         apksigner
//             .arg("sign")
//             .arg("--ks")
//             .arg(&key.path)
//             .arg("--ks-pass")
//             .arg(format!("pass:{}", &key.password))
//             .arg(self.0.apk());
//         if !apksigner.status()?.success() {
//             return Err(NdkError::CmdFailed(apksigner));
//         }
//         Ok(Apk::from_config(self.0))
//     }
// }

// pub struct Apk {
//     path: PathBuf,
//     package_name: String,
//     ndk: Ndk,
// }

// impl Apk {
//     pub fn from_config(config: &ApkConfig) -> Self {
//         let ndk = config.ndk.clone();
//         Self {
//             path: config.apk(),
//             package_name: config.manifest.package_name.clone(),
//             ndk,
//         }
//     }

//     pub fn install(&self) -> Result<(), NdkError> {
//         let mut adb = self.ndk.platform_tool(bin!("adb"))?;
//         adb.arg("install").arg("-r").arg(&self.path);
//         if !adb.status()?.success() {
//             return Err(NdkError::CmdFailed(adb));
//         }
//         Ok(())
//     }

//     pub fn start(&self) -> Result<(), NdkError> {
//         let mut adb = self.ndk.platform_tool(bin!("adb"))?;
//         adb.arg("shell")
//             .arg("am")
//             .arg("start")
//             .arg("-a")
//             .arg("android.intent.action.MAIN")
//             .arg("-n")
//             .arg(format!("{}/android.app.NativeActivity", &self.package_name));
//         if !adb.status()?.success() {
//             return Err(NdkError::CmdFailed(adb));
//         }
//         Ok(())
//     }
// }
