use super::{AndroidBuildCommand, BuildContext};
use crate::error::*;
use android_tools::java_tools::Key;
use crossbundle_tools::{
    commands::android::*,
    error::CommandExt,
    types::{
        AndroidNdk, AndroidSdk, AndroidStrategy, AndroidTarget, CliContext,
        android_manifest::AndroidManifest,
    },
};
use std::path::{Path, PathBuf};

struct PreparedNativeBuild {
    package_name: String,
    android_build_dir: PathBuf,
    native_build_dir: PathBuf,
    manifest: AndroidManifest,
    assets: Option<PathBuf>,
    resources: Option<PathBuf>,
    compiled_libs: Vec<(PathBuf, AndroidTarget)>,
}

impl AndroidBuildCommand {
    fn prepare_native_build(
        &self,
        config: &CliContext,
        context: &BuildContext,
        ndk: &AndroidNdk,
        kind: &str,
        strategy: AndroidStrategy,
    ) -> Result<PreparedNativeBuild> {
        let profile = self.shared.profile();
        let (_, target_dir, package_name) =
            Self::needed_project_dirs(self.shared.example.as_ref(), context)?;
        config.status_message(format!("Starting {kind} build process"), &package_name)?;

        let android_build_dir = target_dir.join("android").join(&package_name);
        let native_build_dir = android_build_dir.join("native").join(kind);
        std::fs::create_dir_all(android_build_dir.join("outputs"))?;

        config.status_message("Reading", "AndroidManifest.xml")?;
        let manifest = Self::get_android_manifest(context, strategy)?;
        config.status_message("Generating", "AndroidManifest.xml")?;
        save_android_manifest(&native_build_dir, &manifest)?;
        config.status("Preparing resources and assets")?;
        let (assets, resources) =
            Self::prepare_assets_and_resources(&context.project_config, &android_build_dir)?;

        config.status_message("Compiling", "lib")?;
        let targets = Self::android_build_targets(context, profile, &self.target);
        let compiled_libs = self.build_target(
            context,
            targets,
            ndk,
            profile,
            Self::min_sdk_version(&manifest),
            &target_dir,
            config,
        )?;

        Ok(PreparedNativeBuild {
            package_name,
            android_build_dir,
            native_build_dir,
            manifest,
            assets,
            resources,
            compiled_libs,
        })
    }

    /// Builds APK with aapt tool and signs it with apksigner.
    pub fn execute_apk(
        &self,
        config: &CliContext,
        context: &BuildContext,
        sdk: &AndroidSdk,
        ndk: &AndroidNdk,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf)> {
        let build =
            self.prepare_native_build(config, context, ndk, "apk", AndroidStrategy::NativeApk)?;

        config.status_message("Generating", "unaligned APK file")?;
        let unaligned_apk_path = gen_unaligned_apk(
            sdk,
            &context.project_path,
            &build.native_build_dir,
            &build.native_build_dir.join("AndroidManifest.xml"),
            &build.assets,
            &build.resources,
            &build.package_name,
            Self::target_sdk_version(&build.manifest, sdk)?,
        )?;

        config.status("Adding libs into APK file")?;
        for (compiled_lib, build_target) in build.compiled_libs {
            add_libs_into_apk(
                sdk,
                ndk,
                &unaligned_apk_path,
                &compiled_lib,
                build_target,
                self.shared.profile(),
                Self::min_sdk_version(&build.manifest),
                &build.android_build_dir,
                &context.target_dir,
            )?;
        }

        config.status("Aligning APK file")?;
        let aligned_apk_path = align_apk(
            sdk,
            &unaligned_apk_path,
            &build.package_name,
            &build.android_build_dir.join("outputs"),
        )?;

        config.status_message("Generating", "debug signing key")?;
        let key = Self::find_keystore(
            self.sign_key_path.clone(),
            self.sign_key_pass.clone(),
            self.sign_key_alias.clone(),
        )?;

        config.status("Signing APK file")?;
        sign_apk(sdk, &aligned_apk_path, key)?;
        config.status("Build finished successfully")?;
        Ok((build.manifest, sdk.clone(), aligned_apk_path))
    }

    /// Builds AAB with aapt2 tool and signs it with jarsigner.
    pub fn execute_aab(
        &self,
        config: &CliContext,
        context: &BuildContext,
        sdk: &AndroidSdk,
        ndk: &AndroidNdk,
        java: &Path,
        jarsigner: &Path,
        bundletool: &Path,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf, String, Key)> {
        let build =
            self.prepare_native_build(config, context, ndk, "aab", AndroidStrategy::NativeAab)?;

        config.status_message("Generating", "proto format APK file")?;
        let compiled_res = if let Some(resources) = &build.resources {
            let output = build.native_build_dir.join("compiled_res");
            std::fs::create_dir_all(&output)?;
            Some(
                sdk.aapt2()?
                    .compile_incremental(dunce::simplified(resources), dunce::simplified(&output))
                    .run()?,
            )
        } else {
            None
        };

        let apk_path = build
            .native_build_dir
            .join(format!("{}_module.apk", build.package_name));
        let mut link = sdk.aapt2()?.link_compiled_res(
            compiled_res,
            &apk_path,
            &build.native_build_dir.join("AndroidManifest.xml"),
        );
        if let Some(assets) = &build.assets {
            link.assets(assets.clone());
        }
        link.android_jar(sdk.android_jar(Self::target_sdk_version(&build.manifest, sdk)?)?)
            .proto_format(true)
            .auto_add_overlay(true)
            .run()?;

        config.status("Extracting apk files")?;
        let extracted_apk_path = extract_archive(
            &apk_path,
            &build.native_build_dir.join("extracted_apk_files"),
        )?;

        config.status("Adding libs")?;
        for (compiled_lib, build_target) in build.compiled_libs {
            add_libs_into_aapt2(
                ndk,
                &compiled_lib,
                build_target,
                self.shared.profile(),
                Self::min_sdk_version(&build.manifest),
                &extracted_apk_path,
                &context.target_dir,
                &build.package_name,
            )?;
        }

        config.status("Generating ZIP module from extracted files")?;
        let module = gen_zip_modules(
            &build.native_build_dir,
            &build.package_name,
            &extracted_apk_path,
        )?;
        for entry in std::fs::read_dir(&build.native_build_dir)? {
            let path = entry?.path();
            if path.ends_with(format!("{}_unsigned.aab", build.package_name)) {
                std::fs::remove_file(path)?;
            }
        }

        config.status("Generating aab from modules")?;
        let aab_path = gen_aab_from_modules_with_toolchain(
            &build.package_name,
            &[module],
            &build.android_build_dir.join("outputs"),
            java,
            bundletool,
        )?;

        config.status_message("Generating", "debug signing key")?;
        let key = Self::find_keystore(
            self.sign_key_path.clone(),
            self.sign_key_pass.clone(),
            self.sign_key_alias.clone(),
        )?;
        config.status_message("Signing", "debug signing key")?;
        let mut command = std::process::Command::new(jarsigner);
        command
            .arg("-keystore")
            .arg(&key.key_path)
            .arg("-storepass")
            .arg(&key.key_pass)
            .arg("-verbose")
            .arg("-sigalg")
            .arg("SHA256withRSA")
            .arg("-digestalg")
            .arg("SHA-256")
            .arg(&aab_path)
            .arg(&key.key_alias);
        command.output_err(true)?;

        let output = build
            .android_build_dir
            .join("outputs")
            .join(format!("{}_signed.aab", build.package_name));
        std::fs::rename(aab_path, &output)?;
        config.status("Build finished successfully")?;
        Ok((build.manifest, sdk.clone(), output, build.package_name, key))
    }
}
