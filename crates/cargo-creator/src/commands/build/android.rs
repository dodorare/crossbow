use super::{BuildContext, SharedBuildCommand};
use crate::error::*;
use android_manifest::AndroidManifest;
use clap::Clap;
use creator_tools::{
    commands::android::{self, AabKey},
    tools::*,
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
    #[clap(long)]
    pub aab: bool,
    /// Path to the signing key
    #[clap(long)]
    pub sign_key_path: Option<PathBuf>,
    /// Signing key password
    #[clap(long)]
    pub sign_key_pass: Option<String>,
    /// Signing key alias
    #[clap(long)]
    pub sign_key_alias: Option<String>,
}

impl AndroidBuildCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        if self.sign_key_path.is_some() && self.sign_key_pass.is_none() {
            config
                .shell()
                .warn("You provided a signing key but not password - set password please by providing `sign_key_pass` flag")?;
        }

        let context = BuildContext::new(config, self.shared.target_dir.clone())?;
        if self.aab {
            self.execute_aab(config, &context)?;
        } else {
            self.execute(config, &context)?;
        }
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
        let android_manifest =
            context.gen_android_manifest(&sdk, &package_name, profile.is_debug())?;
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
            &android_manifest
                .application
                .label
                .clone()
                .unwrap()
                .to_string(),
            &apk_build_dir,
        )?;
        let key = if let Some(path) = self.sign_key_path.clone() {
            android::Key {
                path,
                password: self.sign_key_pass.clone().unwrap(),
            }
        } else {
            config.status_message("Generating", "debug signing key")?;
            android::gen_debug_key().unwrap()
        };
        config.status("Signing APK file")?;
        android::sign_apk(&sdk, &aligned_apk_path, key).unwrap();
        config.status("Build finished successfully")?;
        Ok((android_manifest, sdk, aligned_apk_path))
    }

    pub fn execute_aab(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(PathBuf, String, AabKey)> {
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
        let android_manifest =
            context.gen_android_manifest(&sdk, &package_name, profile.is_debug())?;
        let android_build_dir = target_dir.join("android").join(&profile);
        let manifest_path = android::save_android_manifest(&android_build_dir, &android_manifest)?;
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
        config.status_message("Generating", "proto format APK file")?;
        let compiled_res_path = android_build_dir.join("compiled_res");
        if !compiled_res_path.exists() {
            std::fs::create_dir_all(&compiled_res_path)?;
        }
        let compiled_res = if let Some(res) = context.android_res() {
            let aapt2_compile = Aapt2.compile_incremental(&res, &compiled_res_path);
            let compiled_res = aapt2_compile.run()?;
            Some(compiled_res)
        } else {
            None
        };
        let apk_path = android_build_dir.join(format!("{}_module.apk", package_name));
        let mut aapt2_link = Aapt2.link_compiled_res(compiled_res, &apk_path, &manifest_path);
        aapt2_link
            .android_jar(sdk.android_jar(target_sdk_version)?)
            .assets(context.android_assets().unwrap())
            .proto_format(true)
            .auto_add_overlay(true);
        aapt2_link.run()?;
        config.status("Extracting apk files")?;
        let output_dir = android_build_dir.join("extracted_apk_files");
        let extracted_apk_path = android::extract_apk(&apk_path, &output_dir)?;

        config.status("Adding libs")?;
        for (compiled_lib, build_target) in compiled_libs {
            // let android_abi = build_target.android_abi();
            // let android_compiled_lib = output_dir
            //     .join("lib")
            //     .join(android_abi)
            //     .join(format!("lib{}.so", package_name));
            // if !android_compiled_lib.exists() {
            //     std::fs::create_dir_all(&android_compiled_lib.parent().unwrap())?;
            //     let mut options = fs_extra::file::CopyOptions::new();
            //     options.overwrite = true;
            //     fs_extra::file::copy(&compiled_lib, &android_compiled_lib, &options).unwrap();
            // }
            android::add_libs_into_aapt2(
                &ndk,
                &compiled_lib,
                *build_target,
                profile,
                android_manifest
                    .uses_sdk
                    .as_ref()
                    .unwrap()
                    .min_sdk_version
                    .unwrap_or(9),
                &extracted_apk_path,
                &target_dir,
            )?;
        }
        config.status("Generating ZIP module from extracted files")?;
        let gen_zip_modules =
            android::gen_zip_modules(&android_build_dir, &package_name, &extracted_apk_path)?;
        config.status("Generating aab from modules")?;
        let aab_path =
            android::gen_aab_from_modules(&package_name, &[gen_zip_modules], &android_build_dir)?;
        for entry in std::fs::read_dir(&android_build_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with("extracted_apk_files") {
                std::fs::remove_dir_all(path.clone())?;
            }
            if path.ends_with("example_module.zip") {
                std::fs::remove_file(path)?;
            }
        }
        config.status_message("Generating", "debug signing key")?;
        let key = android::gen_aab_key(
            self.sign_key_path.clone(),
            self.sign_key_pass.clone(),
            self.sign_key_alias.clone(),
        )?;
        println!("{:?}", key);

        android::jarsigner(
            key.key_pass.clone(),
            key.key_path.clone(),
            &aab_path,
            key.key_alias.clone(),
        )
        .unwrap();
        config.status("Build finished successfully")?;
        Ok((aab_path, package_name, key))
    }
}
