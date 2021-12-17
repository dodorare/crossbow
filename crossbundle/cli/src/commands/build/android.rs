use super::{BuildContext, SharedBuildCommand};
use crate::error::*;
use android_manifest::AndroidManifest;
use android_tools::java_tools::{gen_key, AabKey, Jarsigner};
use clap::Parser;
use crossbundle_tools::{
    commands::android::{self, remove},
    tools::*,
    types::*,
    utils::Config,
};
use std::path::PathBuf;

const MIN_SDK_VERSION: u32 = 9;

/// Specifies flags and options needed to build application
#[derive(Parser, Clone, Debug)]
pub struct AndroidBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build for the given android architecture.
    /// Supported targets are: `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`
    #[clap(long, default_value = "aarch64-linux-android")]
    pub target: Vec<AndroidTarget>,
    /// Generating aab. By default crossbow generating apk
    #[clap(long)]
    pub aab: bool,
    /// Path to the signing key
    #[clap(long, requires_all = &["sign-key-pass", "sign-key-alias"])]
    pub sign_key_path: Option<PathBuf>,
    /// Signing key password
    #[clap(long)]
    pub sign_key_pass: Option<String>,
    /// Signing key alias
    #[clap(long)]
    pub sign_key_alias: Option<String>,
}

impl AndroidBuildCommand {
    // Checks options was specified in AndroidBuildCommand and then builds application
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
            self.execute_apk(config, &context)?;
        }
        Ok(())
    }

    /// Builds APK with aapt tool and signs it with apksigner
    pub fn execute_apk(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf)> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (project_path, target_dir, target, package_name) =
            Self::needed_project_dirs(example, context)?;
        config.status_message("Starting build process", &package_name)?;
        let (sdk, ndk, target_sdk_version) = Self::android_toolchain(context)?;
        config.status_message("Generating", "AndroidManifest.xml")?;
        let android_build_dir = target_dir.join("android").join(&profile);
        let (android_manifest, manifest_path) = Self::android_manifest(
            context,
            &sdk,
            package_name.to_string(),
            profile,
            &android_build_dir.clone(),
        )?;

        let build_targets = context.android_build_targets(&self.target);
        let mut compiled_libs = Vec::new();
        for build_target in build_targets.iter() {
            let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
            let rust_triple = build_target.rust_triple();
            config.status_message("Compiling for architecture", rust_triple)?;

            // We need a different compilation process for macroquad projects
            // because of the sokol lib dependency
            if self.shared.quad {
                android::compile_macroquad_rust_for_android(
                    &ndk,
                    *build_target,
                    &project_path,
                    profile,
                    self.shared.features.clone(),
                    self.shared.all_features,
                    self.shared.no_default_features,
                    target_sdk_version,
                    &lib_name,
                )?;
            } else {
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
            }
            let out_dir = target_dir.join(build_target.rust_triple()).join(&profile);
            let compiled_lib = out_dir.join(lib_name);
            compiled_libs.push((compiled_lib, build_target));
        }

        config.status_message("Generating", "unaligned APK file")?;
        let package_label = android_manifest
            .application
            .label
            .clone()
            .unwrap()
            .to_string();

        let unaligned_apk_path = android::gen_unaligned_apk(
            &sdk,
            &project_path,
            &android_build_dir,
            &manifest_path,
            context.android_assets(),
            context.android_res(),
            &package_label,
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
                    .unwrap_or(MIN_SDK_VERSION),
                &android_build_dir,
                &target_dir,
            )?;
        }

        config.status("Aligning APK file")?;
        let aligned_apk_path = android::align_apk(
            &sdk,
            &unaligned_apk_path,
            &package_label,
            &android_build_dir,
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

    /// Builds AAB with aapt2 tool and signs it with jarsigner
    pub fn execute_aab(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf, String, AabKey)> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (project_path, target_dir, target, package_name) =
            Self::needed_project_dirs(example, context)?;
        config.status_message("Starting build process", &package_name)?;
        let (sdk, ndk, target_sdk_version) = Self::android_toolchain(context)?;
        config.status_message("Generating", "AndroidManifest.xml")?;
        let android_build_dir = target_dir.join("android").join(&profile);
        let (android_manifest, manifest_path) = Self::android_manifest(
            context,
            &sdk,
            package_name.to_string(),
            profile,
            &android_build_dir.clone(),
        )?;

        let build_targets = context.android_build_targets(&self.target);
        let mut compiled_libs = Vec::new();
        for build_target in build_targets.iter() {
            let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
            let rust_triple = build_target.rust_triple();
            config.status_message("Compiling for architecture", rust_triple)?;

            // We need a different compilation process for macroquad projects
            // because of the sokol lib dependency
            if self.shared.quad {
                android::compile_macroquad_rust_for_android(
                    &ndk,
                    *build_target,
                    &project_path,
                    profile,
                    self.shared.features.clone(),
                    self.shared.all_features,
                    self.shared.no_default_features,
                    target_sdk_version,
                    &lib_name,
                )?;
            } else {
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
            }
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
            let aapt2_compile = sdk.aapt2()?.compile_incremental(
                dunce::simplified(&res),
                &dunce::simplified(&compiled_res_path).to_owned(),
            );
            let compiled_res = aapt2_compile.run()?;
            Some(compiled_res)
        } else {
            None
        };

        let apk_path = android_build_dir.join(format!("{}_module.apk", package_name));
        let mut aapt2_link =
            sdk.aapt2()?
                .link_compiled_res(compiled_res, &apk_path, &manifest_path);
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
            android::add_libs_into_aapt2(
                &ndk,
                &compiled_lib,
                *build_target,
                profile,
                android_manifest
                    .clone()
                    .uses_sdk
                    .as_ref()
                    .unwrap()
                    .min_sdk_version
                    .unwrap_or(MIN_SDK_VERSION),
                &extracted_apk_path,
                &target_dir,
            )?;
        }

        config.status("Generating ZIP module from extracted files")?;
        let gen_zip_modules =
            android::gen_zip_modules(&android_build_dir, &package_name, &extracted_apk_path)?;

        for entry in std::fs::read_dir(&android_build_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with(format!("{}_unsigned.aab", package_name)) {
                std::fs::remove_file(&path)?;
            }
        }

        config.status("Generating aab from modules")?;
        let aab_path = android::gen_aab_from_modules(
            &package_name,
            &[gen_zip_modules.clone()],
            &android_build_dir,
        )?;

        remove(vec![gen_zip_modules, extracted_apk_path])?;

        config.status_message("Generating", "debug signing key")?;
        let key = if let Some(key_path) = self.sign_key_path.clone() {
            let aab_key = AabKey {
                key_path,
                key_pass: self.sign_key_pass.clone().unwrap(),
                key_alias: self.sign_key_alias.clone().unwrap(),
            };
            if aab_key.key_path.exists() {
                aab_key
            } else {
                gen_key(aab_key)?
            }
        } else {
            let aab_key: AabKey = Default::default();
            if aab_key.key_path.exists() {
                aab_key
            } else {
                gen_key(aab_key)?
            }
        };

        config.status_message("Signing", "debug signing key")?;
        Jarsigner::new(&aab_path, &key.key_alias)
            .keystore(&key.key_path)
            .storepass(key.key_pass.to_string())
            .verbose(true)
            .sigalg("SHA256withRSA".to_string())
            .digestalg("SHA-256".to_string())
            .run()?;

        let signed_aab = android_build_dir.join(format!("{}_signed.aab", package_name));
        std::fs::rename(&aab_path, &signed_aab)?;
        config.status("Build finished successfully")?;

        Ok((android_manifest, sdk, signed_aab, package_name, key))
    }

    /// Specifies project path and target directory needed to build application
    fn needed_project_dirs(
        example: Option<&String>,
        context: &BuildContext,
    ) -> Result<(PathBuf, PathBuf, Target, String)> {
        let project_path: PathBuf = context.project_path.clone();
        let target_dir: PathBuf = context.target_dir.clone();
        let (target, package_name) = if let Some(example) = example {
            (Target::Example(example.clone()), example.clone())
        } else {
            (Target::Lib, context.package_name())
        };
        Ok((project_path, target_dir, target, package_name))
    }

    /// Specifies path to Android SDK and Android NDK
    fn android_toolchain(context: &BuildContext) -> Result<(AndroidSdk, AndroidNdk, u32)> {
        let sdk = AndroidSdk::from_env()?;
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path()))?;
        let target_sdk_version = context.target_sdk_version(&sdk);
        Ok((sdk, ndk, target_sdk_version))
    }

    /// Generates and saves AndroidManifest.xml
    fn android_manifest(
        context: &BuildContext,
        sdk: &AndroidSdk,
        package_name: String,
        profile: Profile,
        android_build_dir: &PathBuf,
    ) -> Result<(AndroidManifest, PathBuf)> {
        let android_manifest =
            context.gen_android_manifest(&sdk, &package_name, profile.is_debug())?;
        let manifest_path = android::save_android_manifest(&android_build_dir, &android_manifest)?;
        Ok((android_manifest, manifest_path))
    }
}
