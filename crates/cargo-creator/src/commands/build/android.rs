use super::{BuildContext, SharedBuildCommand};
use crate::error::*;
use android_manifest::AndroidManifest;
use clap::Clap;
use creator_tools::{
    commands::android,
    tools::{AndroidNdk, AndroidSdk},
    types::*,
    utils::Config,
};
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub struct AndroidBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build for the given android architecture.
    /// Supported targets are: `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`
    #[clap(long, default_value = "aarch64-linux-android")]
    pub target: Vec<AndroidTarget>,
}

impl AndroidBuildCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let context = BuildContext::new(config, self.shared.target_dir.clone())?;
        self.execute(config, &context)?;
        Ok(())
    }

    pub fn execute(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf)> {
        let project_path = context.project_path.clone();
        let target_dir = context.target_dir.clone();
        let profile = self.shared.profile();
        let (target, package_name) = if let Some(example) = &self.shared.example {
            (Target::Example(example.clone()), example.clone())
        } else {
            (Target::Lib, context.package_name())
        };
        config.status_message("Starting build process", &package_name)?;
        let sdk = AndroidSdk::from_env()?;
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path()))?;
        let build_targets = context.android_build_targets(&self.target);
        let target_sdk_version = context.target_sdk_version(&sdk);
        config.status_message("Generating", "AndroidManifest.xml")?;
        let android_manifest = context.gen_android_manifest(&sdk, &package_name)?;
        let apk_build_dir = target_dir.join("android").join(&profile);
        let manifest_path = android::save_android_manifest(&apk_build_dir, &android_manifest)?;
        let mut compiled_libs = Vec::new();
        for build_target in build_targets.iter() {
            let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
            let rust_triple = build_target.rust_triple();
            config.status_message("Compiling for architecture", rust_triple)?;
            android::compile_rust_for_android(
                &ndk,
                target.clone(),
                *build_target,
                &project_path,
                profile,
                self.shared.features.clone(),
                self.shared.all_features,
                self.shared.no_default_features,
                target_sdk_version,
            )?;
            let out_dir = target_dir.join(build_target.rust_triple()).join(&profile);
            let compiled_lib = out_dir.join(lib_name);
            compiled_libs.push((compiled_lib, build_target));
        }
        config.status_message("Generating", "unaligned APK file")?;
        let unaligned_apk_path = android::gen_unaligned_apk(
            &sdk,
            &project_path,
            &apk_build_dir,
            &manifest_path,
            context.android_assets(),
            context.android_res(),
            android_manifest
                .application
                .label
                .clone()
                .unwrap()
                .to_string(),
            target_sdk_version,
        )?;
        config.status("Adding libs into APK file")?;
        for (compiled_lib, build_target) in compiled_libs {
            android::add_libs_into_apk(
                &sdk,
                &ndk,
                &unaligned_apk_path,
                &compiled_lib,
                *build_target,
                profile,
                android_manifest
                    .uses_sdk
                    .as_ref()
                    .unwrap()
                    .min_sdk_version
                    .unwrap_or(9),
                &apk_build_dir,
                &target_dir,
            )?;
        }
        config.status("Aligning APK file")?;
        let aligned_apk_path = android::align_apk(
            &sdk,
            &unaligned_apk_path,
            &android_manifest.package,
            &apk_build_dir,
        )?;
        config.status_message("Generating", "debug signing key")?;
        let key = android::gen_debug_key().unwrap();
        config.status("Signing APK file")?;
        android::sign_apk(&sdk, &aligned_apk_path, key).unwrap();
        config.status("Build finished successfully")?;
        Ok((android_manifest, sdk, aligned_apk_path))
    }
}
