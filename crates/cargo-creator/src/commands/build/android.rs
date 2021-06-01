use super::{BuildContext, SharedBuildCommand};
use crate::{error::*, manifest::into_android_manifest};
use clap::Clap;
use creator_tools::{
    commands::android,
    deps::{AndroidNdk, AndroidSdk},
    types::*,
    utils::Config,
};
use std::{convert::TryInto, path::PathBuf};

#[derive(Clap, Clone, Debug)]
pub struct AndroidBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build for the given android architecture.
    /// Supported targets are: `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`.
    #[clap(long, default_value = "aarch64-linux-android")]
    pub target: Vec<AndroidTarget>,
}

impl AndroidBuildCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let build_context = BuildContext::init(config, self.shared.target_dir.clone())?;
        self.execute(config, &build_context)?;
        Ok(())
    }

    pub fn execute(
        &self,
        config: &Config,
        build_context: &BuildContext,
    ) -> Result<(String, AndroidSdk, PathBuf)> {
        let package = build_context
            .manifest
            .package
            .as_ref()
            .ok_or(Error::InvalidManifest)?;
        let metadata = package
            .metadata
            .clone()
            .ok_or(Error::InvalidManifestMetadata)?
            .android
            .unwrap();
        let project_path = build_context.project_path.clone();
        let target_dir = build_context.target_dir.clone();
        let profile = match self.shared.release {
            true => Profile::Release,
            false => Profile::Debug,
        };
        let package_name;
        let target = if let Some(example) = self.shared.example.clone() {
            package_name = example.clone();
            Target::Example(example)
        } else {
            package_name = package.name.clone();
            Target::Lib
        };
        config
            .shell()
            .status_message("Starting build process", &package_name)?;
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let target_sdk_version = metadata.manifest.uses_sdk.unwrap().target_sdk_version.unwrap_or_else(|| sdk.default_platform().try_into().unwrap());
        let build_targets = if !self.target.is_empty() {
            self.target.clone()
        } else if metadata.build_targets.is_some()
            && !metadata.build_targets.as_ref().unwrap().is_empty()
        {
            metadata.build_targets.unwrap()
        } else {
            vec![AndroidTarget::Aarch64LinuxAndroid]
        };
        let mut compiled_libs = Vec::new();
        for build_target in build_targets.iter() {
            let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
            let rust_triple = build_target.rust_triple();
            config
                .shell()
                .status_message("Compiling for architecture", rust_triple)?;
            android::compile_rust_for_android(
                &ndk,
                target.clone(),
                *build_target,
                &project_path,
                profile,
                self.shared.features.clone(),
                self.shared.all_features,
                self.shared.no_default_features,
                target_sdk_version.try_into().unwrap(),
            )
            .unwrap();
            let out_dir = target_dir.join(build_target.rust_triple()).join(&profile);
            let compiled_lib = out_dir.join(lib_name);
            compiled_libs.push((compiled_lib, build_target));
        }
        config
            .shell()
            .status_message("Generating", "AndroidManifest.xml")?;
        let android_manifest = into_android_manifest(package_name, target_sdk_version);
        let apk_build_dir = target_dir.join("android").join(&profile);
        let manifest_path =
            android::create_android_manifest(&apk_build_dir, android_manifest.clone()).unwrap();
        config
            .shell()
            .status_message("Generating", "unaligned APK file")?;
        let unaligned_apk_path = android::gen_unaligned_apk(
            &sdk,
            &apk_build_dir,
            &manifest_path,
            metadata.assets.clone(),
            metadata.resources,
            android_manifest.clone(),
        )
        .unwrap();
        config.shell().status("Adding libs into APK file")?;
        for (compiled_lib, build_target) in compiled_libs {
            android::add_libs_into_apk(
                &sdk,
                &ndk,
                &unaligned_apk_path,
                &compiled_lib,
                *build_target,
                profile,
                android_manifest.clone().uses_sdk.unwrap().min_sdk_version.unwrap().try_into().unwrap(),
                &apk_build_dir,
                &target_dir,
            )
            .unwrap();
        }
        config.shell().status("Aligning APK file")?;
        let aligned_apk_path = android::align_apk(
            &sdk,
            &unaligned_apk_path,
            &android_manifest.package,
            &apk_build_dir,
        )
        .unwrap();
        config
            .shell()
            .status_message("Generating", "debug signing key")?;
        let key = android::gen_debug_key().unwrap();
        config.shell().status("Signing APK file")?;
        android::sign_apk(&sdk, &aligned_apk_path, key).unwrap();
        config.shell().status("Build finished successfully")?;
        Ok((android_manifest.package, sdk, aligned_apk_path))
    }
}
