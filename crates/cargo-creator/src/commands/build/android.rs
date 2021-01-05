use super::{BuildContext, SharedBuildCommand};
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub struct AndroidBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build for the given android architecture. Supported targets are: `armv7-linux-androideabi`,
    /// `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`
    #[clap(long)]
    pub target: Vec<AndroidTarget>,
}

impl AndroidBuildCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let build_context = BuildContext::init(&current_dir, self.shared.target_dir.clone())?;
        self.execute(&build_context)?;
        Ok(())
    }

    pub fn execute(&self, build_context: &BuildContext) -> Result<(String, AndroidSdk, PathBuf)> {
        log::info!("Starting build process");
        let package = build_context
            .manifest
            .package
            .as_ref()
            .ok_or(Error::InvalidManifest)?;
        let metadata = package
            .metadata
            .clone()
            .ok_or(Error::InvalidManifestMetadata)?
            .android;
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
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let target_sdk_version = metadata
            .manifest
            .target_sdk_version
            .unwrap_or_else(|| sdk.default_platform());
        let build_targets = if !self.target.is_empty() {
            self.target.clone()
        } else if metadata.build_targets.is_some()
            && !metadata.build_targets.as_ref().unwrap().is_empty()
        {
            metadata.build_targets.unwrap()
        } else {
            vec![AndroidTarget::Aarch64LinuxAndroid]
        };
        log::info!("Compiling rust lib");
        let mut compiled_libs = Vec::new();
        for build_target in build_targets.iter() {
            compile_rust_for_android(
                &ndk,
                target.clone(),
                *build_target,
                &project_path,
                profile,
                self.shared.features.clone(),
                self.shared.all_features,
                self.shared.no_default_features,
                target_sdk_version,
            )
            .unwrap();
            let out_dir = target_dir.join(build_target.rust_triple()).join(&profile);
            let compiled_lib = out_dir.join(format!("lib{}.so", package_name));
            compiled_libs.push((compiled_lib, build_target))
        }
        log::info!("Generating AndroidManifest.xml");
        let android_manifest = metadata.manifest.into_android_manifest(
            &target,
            profile,
            package_name,
            package.version.clone(),
            target_sdk_version,
        );
        let apk_build_dir = target_dir.join("android").join(&profile);
        let manifest_path = gen_android_manifest(&apk_build_dir, &android_manifest).unwrap();
        log::info!("Generating unaligned APK file");
        let unaligned_apk_path = gen_unaligned_apk(
            &sdk,
            &apk_build_dir,
            &manifest_path,
            metadata.assets.clone(),
            metadata.resources,
            &android_manifest,
        )
        .unwrap();
        log::info!("Adding all needed libs into unaligned APK file");
        for (compiled_lib, build_target) in compiled_libs {
            add_libs_into_apk(
                &sdk,
                &ndk,
                &unaligned_apk_path,
                &compiled_lib,
                *build_target,
                profile,
                android_manifest.min_sdk_version,
                &apk_build_dir,
                &target_dir,
            )
            .unwrap();
        }
        log::info!("Aligning APK file");
        let aligned_apk_path = align_apk(
            &sdk,
            &unaligned_apk_path,
            &android_manifest.package_label,
            &apk_build_dir,
        )
        .unwrap();
        log::info!("Generating debug key for signing APK file");
        let key = gen_debug_key().unwrap();
        log::info!("Signing APK file");
        sign_apk(&sdk, &aligned_apk_path, key).unwrap();
        Ok((android_manifest.package_name, sdk, aligned_apk_path))
    }
}
