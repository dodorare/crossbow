use super::{BuildContext, SharedBuildCommand};
use android_manifest::AndroidManifest;
use android_tools::java_tools::{AabKey, JarSigner};
use clap::Parser;
use crossbundle_tools::{
    commands::android::{self, rust_compile},
    tools::*,
    types::*,
    utils::Config,
};
use std::path::{Path, PathBuf};

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
    /// Compile rust code as a dynamic library [default: crossbow-android]
    #[clap(long, default_missing_value = "crossbow_android")]
    pub lib: Option<String>,
    /// Compile rust code as a dynamic library, generate Gradle project and build generate apk/aab
    #[clap(long)]
    pub gradle: bool,
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
    pub fn run(&self, config: &Config) -> crate::error::Result<()> {
        if self.sign_key_path.is_some() && self.sign_key_pass.is_none() {
            config
                .shell()
                .warn("You provided a signing key but not password - set password please by providing `sign_key_pass` flag")?;
        }
        let context = BuildContext::new(config, self.shared.target_dir.clone())?;
        if self.aab {
            self.execute_aab(config, &context)?;
        } else if let Some(lib_name) = &self.lib {
            self.build_rust_lib(config, &context, lib_name)?;
        } else if self.gradle {
            self.build_gradle(config, &context)?;
        } else {
            self.execute_apk(config, &context)?;
        }
        Ok(())
    }

    /// Compile rust code as a dynamic library, generate Gradle project and build generate apk/aab
    pub fn build_gradle(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> crate::error::Result<PathBuf> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (_, target_dir, package_name) = Self::needed_project_dirs(example, context)?;

        config.status_message("Starting gradle build process", &package_name)?;
        let android_build_dir = target_dir.join("android").join(&package_name);

        config.status("Generating gradle project")?;
        let gradle_project_path = android::gen_gradle_project(
            &android_build_dir,
            context.android_res(),
            context.android_assets(),
        )?;

        // Get AndroidManifest.xml from file or generate from Cargo.toml
        let (sdk, _, _) = Self::android_toolchain(context)?;
        let (_android_manifest, _manifest_path) = Self::android_manifest(
            config,
            context,
            &sdk,
            &package_name,
            profile,
            &gradle_project_path,
            true,
        )?;

        let lib_name = "crossbow_android";
        self.build_rust_lib(config, context, &lib_name)?;

        config.status_message(
            "Gradle project generated",
            gradle_project_path.to_str().unwrap(),
        )?;
        Ok(gradle_project_path)
    }

    /// Compile rust code as a dynamic library
    pub fn build_rust_lib(
        &self,
        config: &Config,
        context: &BuildContext,
        lib_name: &str,
    ) -> crate::error::Result<()> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (project_path, target_dir, package_name) = Self::needed_project_dirs(example, context)?;
        config.status_message("Starting lib build process", &package_name)?;
        let (_sdk, ndk, target_sdk_version) = Self::android_toolchain(context)?;

        let android_build_dir = target_dir.join("android").join(&package_name);

        config.status_message("Compiling", "lib")?;
        let build_targets = context.android_build_targets(&self.target);
        let compiled_libs = self.build_target(
            build_targets,
            lib_name,
            &ndk,
            &project_path,
            profile,
            target_sdk_version,
            &target_dir,
            config,
        )?;

        for (compiled_lib, build_target) in compiled_libs {
            config.status_message(
                "Moving library to target/android/ directory",
                compiled_lib.file_name().unwrap().to_str().unwrap(),
            )?;
            let abi = build_target.android_abi();
            let out_dir = android_build_dir.join("libs").join(profile).join(abi);
            if !out_dir.exists() {
                std::fs::create_dir_all(&out_dir)?;
            }
            let file_name = compiled_lib.file_name().unwrap().to_owned();
            std::fs::copy(compiled_lib, &out_dir.join(&file_name))?;
        }
        Ok(())
    }

    /// Builds APK with aapt tool and signs it with apksigner
    pub fn execute_apk(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> crate::error::Result<(AndroidManifest, AndroidSdk, PathBuf)> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (project_path, target_dir, package_name) = Self::needed_project_dirs(example, context)?;
        config.status_message("Starting apk build process", &package_name)?;
        let (sdk, ndk, target_sdk_version) = Self::android_toolchain(context)?;

        let android_build_dir = target_dir.join("android").join(&package_name);
        let native_build_dir = android_build_dir.join("native");
        let outputs_build_dir = android_build_dir.join("outputs");
        if !outputs_build_dir.exists() {
            std::fs::create_dir_all(&outputs_build_dir)?;
        }

        // Get AndroidManifest.xml from file or generate from Cargo.toml
        let (android_manifest, manifest_path) = Self::android_manifest(
            config,
            context,
            &sdk,
            &package_name.to_string(),
            profile,
            &native_build_dir,
            false,
        )?;

        config.status_message("Compiling", "lib")?;
        let build_targets = context.android_build_targets(&self.target);
        let compiled_libs = self.build_target(
            build_targets,
            &package_name,
            &ndk,
            &project_path,
            profile,
            target_sdk_version,
            &target_dir,
            config,
        )?;

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
            &native_build_dir,
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
                build_target,
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
            &outputs_build_dir,
        )?;

        config.status_message("Generating", "debug signing key")?;
        let key = Self::find_keystore(
            self.sign_key_path.clone(),
            self.sign_key_pass.clone(),
            self.sign_key_alias.clone(),
        )?;

        config.status("Signing APK file")?;
        android::sign_apk(&sdk, &aligned_apk_path, key)?;
        config.status("Build finished successfully")?;
        Ok((android_manifest, sdk, aligned_apk_path))
    }

    /// Builds AAB with aapt2 tool and signs it with jarsigner
    pub fn execute_aab(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> crate::error::Result<(AndroidManifest, AndroidSdk, PathBuf, String, AabKey)> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (project_path, target_dir, package_name) = Self::needed_project_dirs(example, context)?;
        config.status_message("Starting aab build process", &package_name)?;
        let (sdk, ndk, target_sdk_version) = Self::android_toolchain(context)?;

        let android_build_dir = target_dir.join("android").join(&package_name);
        let native_build_dir = android_build_dir.join("native");
        let outputs_build_dir = android_build_dir.join("outputs");
        if !outputs_build_dir.exists() {
            std::fs::create_dir_all(&outputs_build_dir)?;
        }

        // Get AndroidManifest.xml from file or generate from Cargo.toml
        let (android_manifest, manifest_path) = Self::android_manifest(
            config,
            context,
            &sdk,
            &package_name,
            profile,
            &native_build_dir.clone(),
            false,
        )?;

        config.status_message("Compiling", "lib")?;
        let build_targets = context.android_build_targets(&self.target);
        let compiled_libs = self.build_target(
            build_targets,
            &package_name,
            &ndk,
            &project_path,
            profile,
            target_sdk_version,
            &target_dir,
            config,
        )?;

        config.status_message("Generating", "proto format APK file")?;
        let compiled_res_path = native_build_dir.join("compiled_res");
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

        let apk_path = native_build_dir.join(format!("{}_module.apk", package_name));
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
        let output_dir = native_build_dir.join("extracted_apk_files");
        let extracted_apk_path = android::extract_archive(&apk_path, &output_dir)?;

        config.status("Adding libs")?;
        for (compiled_lib, build_target) in compiled_libs {
            android::add_libs_into_aapt2(
                &ndk,
                &compiled_lib,
                build_target,
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
                &package_name,
            )?;
        }

        config.status("Generating ZIP module from extracted files")?;
        let gen_zip_modules =
            android::gen_zip_modules(&native_build_dir, &package_name, &extracted_apk_path)?;

        for entry in std::fs::read_dir(&native_build_dir)? {
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
            &outputs_build_dir,
        )?;

        config.status_message("Generating", "debug signing key")?;
        let key = Self::find_keystore(
            self.sign_key_path.clone(),
            self.sign_key_pass.clone(),
            self.sign_key_alias.clone(),
        )?;

        config.status_message("Signing", "debug signing key")?;
        JarSigner::new(&aab_path, &key.key_alias)
            .keystore(&key.key_path)
            .storepass(key.key_pass.to_string())
            .verbose(true)
            .sigalg("SHA256withRSA".to_string())
            .digestalg("SHA-256".to_string())
            .run()?;

        let signed_aab = android_build_dir.join(format!("{}_signed.aab", package_name));
        std::fs::rename(&aab_path, &signed_aab)?;
        let output_aab = signed_aab.file_name().unwrap().to_str().unwrap();
        println!("output_aab {:?}", output_aab);
        println!("outputs_build_dir {:?}", outputs_build_dir);
        let aab_output_path = outputs_build_dir.join(output_aab);
        let mut options = fs_extra::file::CopyOptions::new();
        options.overwrite = true;
        fs_extra::file::move_file(&signed_aab, &outputs_build_dir.join(output_aab), &options)?;
        config.status("Build finished successfully")?;
        Ok((android_manifest, sdk, aab_output_path, package_name, key))
    }

    /// Specifies project path and target directory needed to build application
    fn needed_project_dirs(
        example: Option<&String>,
        context: &BuildContext,
    ) -> crate::error::Result<(PathBuf, PathBuf, String)> {
        let project_path: PathBuf = context.project_path.clone();
        let target_dir: PathBuf = context.target_dir.clone();
        let (_target, package_name) = if let Some(example) = example {
            (Target::Example(example.clone()), example.clone())
        } else {
            (Target::Lib, context.package_name())
        };
        Ok((project_path, target_dir, package_name))
    }

    /// Specifies path to Android SDK and Android NDK
    fn android_toolchain(
        context: &BuildContext,
    ) -> crate::error::Result<(AndroidSdk, AndroidNdk, u32)> {
        let sdk = AndroidSdk::from_env()?;
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path()))?;
        let target_sdk_version = context.target_sdk_version(&sdk);
        Ok((sdk, ndk, target_sdk_version))
    }

    /// Generates or copies AndroidManifest.xml from specified path, then saves it to android folder
    fn android_manifest(
        config: &Config,
        context: &BuildContext,
        sdk: &AndroidSdk,
        package_name: &str,
        profile: Profile,
        android_build_dir: &Path,
        gradle: bool,
    ) -> crate::error::Result<(AndroidManifest, PathBuf)> {
        config.status_message("Generating", "AndroidManifest.xml")?;
        let android_manifest =
            context.gen_android_manifest(sdk, package_name, profile.is_debug(), gradle)?;
        let manifest_path = android::save_android_manifest(android_build_dir, &android_manifest)?;
        Ok((android_manifest, manifest_path))
    }

    /// Find keystore for signing application or create it
    fn find_keystore(
        sign_key_path: Option<PathBuf>,
        sign_key_pass: Option<String>,
        sign_key_alias: Option<String>,
    ) -> crate::error::Result<AabKey> {
        let key = if let Some(key_path) = sign_key_path {
            let aab_key = AabKey {
                key_path,
                key_pass: sign_key_pass.unwrap(),
                key_alias: sign_key_alias.unwrap(),
            };
            if aab_key.key_path.exists() {
                aab_key
            } else {
                android::gen_key(
                    Some(aab_key.key_path),
                    Some(aab_key.key_pass),
                    Some(aab_key.key_alias),
                )?
            }
        } else {
            let aab_key = AabKey::new_default()?;
            if aab_key.key_path.exists() {
                aab_key
            } else {
                android::gen_key(
                    Some(aab_key.key_path),
                    Some(aab_key.key_pass),
                    Some(aab_key.key_alias),
                )?
            }
        };
        Ok(key)
    }

    /// Compiling libs for architecture and write out it in vector
    fn build_target(
        &self,
        build_targets: Vec<AndroidTarget>,
        package_name: &str,
        ndk: &AndroidNdk,
        project_path: &Path,
        profile: Profile,
        target_sdk_version: u32,
        target_dir: &Path,
        config: &Config,
    ) -> crate::error::Result<Vec<(PathBuf, AndroidTarget)>> {
        let mut libs = Vec::new();
        for build_target in build_targets {
            let lib_name = format!("lib{}.so", package_name.replace('-', "_"));
            let rust_triple = build_target.rust_triple();

            config.status_message("Compiling for architecture", rust_triple)?;
            let app_wrapper = match self.shared.quad {
                true => ApplicationWrapper::Sokol,
                false => ApplicationWrapper::NdkGlue,
            };

            // Compile rust code for android depending on application wrapper
            rust_compile(
                ndk,
                build_target,
                project_path,
                profile,
                self.shared.features.clone(),
                self.shared.all_features,
                self.shared.no_default_features,
                target_sdk_version,
                &lib_name,
                app_wrapper,
            )?;

            let out_dir = target_dir.join(build_target.rust_triple()).join(&profile);
            let compiled_lib = out_dir.join(lib_name);
            libs.push((compiled_lib, build_target));
        }
        Ok(libs)
    }
}
