use super::{BuildContext, SharedBuildCommand};
use crate::{error::*, types::CrossbowMetadata};
use android_manifest::AndroidManifest;
use android_tools::java_tools::Key;
use clap::{ArgAction, Parser};
use crossbundle_tools::{
    commands::{android::*, combine_folders},
    error::CommandExt,
    types::*,
};
use std::path::{Path, PathBuf};

/// Specifies flags and options needed to build application
#[derive(Parser, Clone, Debug, Default)]
pub struct AndroidBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build for the given android architecture.
    /// Supported targets are: `armv7-linux-androideabi`, `aarch64-linux-android`,
    /// `i686-linux-android`, `x86_64-linux-android`
    #[clap(long, short, action = ArgAction::Append)]
    pub target: Vec<AndroidTarget>,
    /// Build strategy specifies what and how to build Android application: with help of
    /// Gradle, or with our native approach.
    #[clap(long, short, default_value = "gradle-apk")]
    pub strategy: AndroidStrategy,
    /// Only compile rust code as a dynamic library. By default: "crossbow-android"
    #[clap(long, default_missing_value = "crossbow_android")]
    pub lib: Option<String>,
    /// Path to export Gradle project. By default exports to `target/android/` folder.
    #[clap(long)]
    pub export_path: Option<PathBuf>,
    /// Path to the signing key.
    #[clap(long, requires_all = &["sign_key_pass", "sign_key_alias"])]
    pub sign_key_path: Option<PathBuf>,
    /// Signing key password.
    #[clap(long)]
    pub sign_key_pass: Option<String>,
    /// Signing key alias.
    #[clap(long)]
    pub sign_key_alias: Option<String>,
    /// Print the immutable build plan without creating files or running commands.
    #[clap(long)]
    pub dry_run: bool,
    /// Emit the dry-run plan as stable JSON.
    #[clap(long, requires = "dry_run")]
    pub json: bool,
}

impl AndroidBuildCommand {
    // Checks options was specified in AndroidBuildCommand and then builds application.
    pub fn run(&self, config: &Config) -> Result<()> {
        if self.sign_key_path.is_some() && self.sign_key_pass.is_none() {
            config
                .shell()
                .warn("You provided a signing key but not password - set password please by providing `sign_key_pass` flag")?;
        }
        let context = BuildContext::new(config, &self.shared)?;
        let plan = self.create_plan(
            &context,
            crossbundle_tools::toolchain::PlanOperation::Build,
            false,
        );
        if self.dry_run {
            self.print_plan(&plan)?;
            return self.ensure_plan_valid(&plan);
        }
        self.ensure_plan_valid(&plan)?;
        let mut runner = AndroidBuildExecutor::new(self, config, &context, &plan)?;
        crossbundle_tools::toolchain::execute(&plan, &mut runner).map_err(plan_error)?;
        Ok(())
    }

    pub fn create_plan(
        &self,
        context: &BuildContext,
        operation: crossbundle_tools::toolchain::PlanOperation,
        attach_logger: bool,
    ) -> crossbundle_tools::toolchain::BuildPlan {
        let profile = self.shared.profile();
        let targets = Self::android_build_targets(context, profile, &self.target)
            .iter()
            .map(|target| target.rust_triple().to_owned())
            .collect();
        let strategy = match self.strategy {
            AndroidStrategy::GradleApk => crossbundle_tools::toolchain::PlanStrategy::GradleApk,
            AndroidStrategy::NativeApk => crossbundle_tools::toolchain::PlanStrategy::NativeApk,
            AndroidStrategy::NativeAab => crossbundle_tools::toolchain::PlanStrategy::NativeAab,
        };
        crossbundle_tools::toolchain::plan(
            &crossbundle_tools::toolchain::PlanRequest {
                operation,
                strategy,
                project_dir: context.project_path.clone(),
                targets,
                attach_logger,
                library_only: self.lib.is_some(),
                runtime: context.config.android.runtime,
            },
            &crossbundle_tools::toolchain::Environment::discover(),
        )
    }

    pub fn print_plan(&self, plan: &crossbundle_tools::toolchain::BuildPlan) -> Result<()> {
        if self.json {
            println!(
                "{}",
                serde_json::to_string_pretty(plan).map_err(Error::DoctorReport)?
            );
        } else {
            println!("Android {:?} plan ({:?})", plan.operation, plan.strategy);
            for (index, step) in plan.steps.iter().enumerate() {
                println!("{}. {}: {}", index + 1, step.id, step.action);
            }
        }
        Ok(())
    }

    pub fn ensure_plan_valid(&self, plan: &crossbundle_tools::toolchain::BuildPlan) -> Result<()> {
        if plan.diagnostics.status == crossbundle_tools::toolchain::ReportStatus::Fail {
            let failures = plan
                .diagnostics
                .checks
                .iter()
                .filter(|check| check.status == crossbundle_tools::toolchain::CheckStatus::Fail)
                .map(|check| {
                    check.remediation.as_ref().map_or_else(
                        || check.summary.clone(),
                        |remediation| format!("{}: {remediation}", check.summary),
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");
            Err(anyhow::anyhow!("Android build plan is invalid: {failures}").into())
        } else {
            Ok(())
        }
    }

    /// Compile rust code as a dynamic library, generate Gradle project.
    pub fn build_gradle(
        &self,
        config: &Config,
        context: &BuildContext,
        export_path: &Option<PathBuf>,
        sdk: &AndroidSdk,
        ndk: &AndroidNdk,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf)> {
        let example = self.shared.example.as_ref();
        let (_, target_dir, package_name) = Self::needed_project_dirs(example, context)?;

        config.status_message("Starting gradle build process", &package_name)?;
        let android_build_dir = if let Some(export_path) = export_path {
            std::fs::create_dir_all(export_path)?;
            dunce::canonicalize(export_path)?
        } else {
            target_dir.join("android").join(&package_name)
        };

        config.status("Preparing resources and assets")?;
        let (assets, resources) =
            Self::prepare_assets_and_resources(&context.config, &android_build_dir)?;
        config.status_message("Reading", "AndroidManifest.xml")?;
        let manifest = Self::get_android_manifest(context, AndroidStrategy::GradleApk)?;
        let manifest_package = manifest
            .package
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("Android manifest package is missing"))?;
        let uses_sdk = manifest
            .uses_sdk
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Android manifest uses-sdk configuration is missing"))?;
        let min_sdk_version = uses_sdk.min_sdk_version.unwrap_or(DEFAULT_ANDROID_MIN_SDK);
        let target_sdk_version = uses_sdk
            .target_sdk_version
            .unwrap_or(DEFAULT_ANDROID_TARGET_SDK);

        config.status("Generating gradle project")?;
        let library_name = self.cargo_library_name(context)?;
        let gradle_project_path = gen_gradle_project(
            manifest_package,
            manifest.version_code.unwrap_or(1),
            &manifest
                .version_name
                .clone()
                .unwrap_or_else(|| "0.1".to_owned()),
            AndroidSdkVersions {
                min_sdk: min_sdk_version,
                target_sdk: target_sdk_version,
            },
            &android_build_dir,
            &assets,
            &resources,
            &context.config.android.plugins,
            context.config.android.runtime,
            &library_name,
            &context.project,
            context.config.android_uses_crossbow_bridge(),
        )?;

        config.status_message("Generating", "AndroidManifest.xml")?;
        let mut gradle_manifest = manifest.clone();
        gradle_manifest.uses_sdk = None;
        gradle_manifest.package = None;
        gradle_manifest.version_code = None;
        gradle_manifest.version_name = None;
        save_android_manifest(&gradle_project_path, &gradle_manifest)?;

        self.build_rust_lib(config, context, &library_name, Some(android_build_dir), ndk)?;

        config.status_message(
            "Gradle project generated",
            gradle_project_path.to_str().unwrap(),
        )?;
        Ok((manifest, sdk.clone(), gradle_project_path))
    }

    /// Compile rust code as a dynamic library.
    pub fn build_rust_lib(
        &self,
        config: &Config,
        context: &BuildContext,
        lib_name: &str,
        export_path: Option<PathBuf>,
        ndk: &AndroidNdk,
    ) -> Result<()> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (_, target_dir, package_name) = Self::needed_project_dirs(example, context)?;
        config.status_message("Starting lib build process", &package_name)?;

        let android_build_dir = if let Some(export_path) = export_path {
            export_path
        } else {
            target_dir.join("android").join(&package_name)
        };

        config.status_message("Reading", "AndroidManifest.xml")?;
        let manifest = Self::get_android_manifest(context, AndroidStrategy::NativeApk)?;

        config.status_message("Compiling", "lib")?;
        let min_sdk_version = Self::min_sdk_version(&manifest);
        let build_targets = Self::android_build_targets(context, profile, &self.target);
        let compiled_libs = self.build_target(
            context,
            build_targets,
            ndk,
            profile,
            min_sdk_version,
            &target_dir,
            config,
        )?;

        for (compiled_lib, build_target) in compiled_libs {
            let output_file_name = format!("lib{}.so", lib_name.replace('-', "_"));
            config.status_message(
                "Moving library to target/android/ directory",
                &output_file_name,
            )?;
            let abi = build_target.android_abi();
            let out_dir = android_build_dir.join("libs").join(profile).join(abi);
            if !out_dir.exists() {
                std::fs::create_dir_all(&out_dir)?;
            }
            std::fs::copy(compiled_lib, out_dir.join(output_file_name))?;
        }
        Ok(())
    }

    /// Builds APK with aapt tool and signs it with apksigner.
    pub fn execute_apk(
        &self,
        config: &Config,
        context: &BuildContext,
        sdk: &AndroidSdk,
        ndk: &AndroidNdk,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf)> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (project_path, target_dir, package_name) = Self::needed_project_dirs(example, context)?;
        config.status_message("Starting apk build process", &package_name)?;

        let android_build_dir = target_dir.join("android").join(&package_name);
        let native_build_dir = android_build_dir.join("native").join("apk");
        let outputs_build_dir = android_build_dir.join("outputs");
        if !outputs_build_dir.exists() {
            std::fs::create_dir_all(&outputs_build_dir)?;
        }

        config.status_message("Reading", "AndroidManifest.xml")?;
        let manifest = Self::get_android_manifest(context, AndroidStrategy::NativeApk)?;
        config.status_message("Generating", "AndroidManifest.xml")?;
        let manifest_path = save_android_manifest(&native_build_dir, &manifest)?;
        config.status("Preparing resources and assets")?;
        let (assets, resources) =
            Self::prepare_assets_and_resources(&context.config, &android_build_dir)?;

        config.status_message("Compiling", "lib")?;
        let target_sdk_version = Self::target_sdk_version(&manifest, sdk);
        let min_sdk_version = Self::min_sdk_version(&manifest);
        let build_targets = Self::android_build_targets(context, profile, &self.target);
        let compiled_libs = self.build_target(
            context,
            build_targets,
            ndk,
            profile,
            min_sdk_version,
            &target_dir,
            config,
        )?;

        config.status_message("Generating", "unaligned APK file")?;
        let unaligned_apk_path = gen_unaligned_apk(
            sdk,
            &project_path,
            &native_build_dir,
            &manifest_path,
            &assets,
            &resources,
            &package_name,
            target_sdk_version,
        )?;

        config.status("Adding libs into APK file")?;
        for (compiled_lib, build_target) in compiled_libs {
            add_libs_into_apk(
                sdk,
                ndk,
                &unaligned_apk_path,
                &compiled_lib,
                build_target,
                profile,
                Self::min_sdk_version(&manifest),
                &android_build_dir,
                &target_dir,
            )?;
        }

        config.status("Aligning APK file")?;
        let aligned_apk_path =
            align_apk(sdk, &unaligned_apk_path, &package_name, &outputs_build_dir)?;

        config.status_message("Generating", "debug signing key")?;
        let key = Self::find_keystore(
            self.sign_key_path.clone(),
            self.sign_key_pass.clone(),
            self.sign_key_alias.clone(),
        )?;

        config.status("Signing APK file")?;
        sign_apk(sdk, &aligned_apk_path, key)?;
        config.status("Build finished successfully")?;
        Ok((manifest, sdk.clone(), aligned_apk_path))
    }

    /// Builds AAB with aapt2 tool and signs it with jarsigner.
    pub fn execute_aab(
        &self,
        config: &Config,
        context: &BuildContext,
        sdk: &AndroidSdk,
        ndk: &AndroidNdk,
        java: &Path,
        jarsigner: &Path,
        bundletool: &Path,
    ) -> Result<(AndroidManifest, AndroidSdk, PathBuf, String, Key)> {
        let profile = self.shared.profile();
        let example = self.shared.example.as_ref();
        let (_, target_dir, package_name) = Self::needed_project_dirs(example, context)?;
        config.status_message("Starting aab build process", &package_name)?;

        let android_build_dir = target_dir.join("android").join(&package_name);
        let native_build_dir = android_build_dir.join("native").join("aab");
        let outputs_build_dir = android_build_dir.join("outputs");
        if !outputs_build_dir.exists() {
            std::fs::create_dir_all(&outputs_build_dir)?;
        }

        config.status_message("Reading", "AndroidManifest.xml")?;
        let manifest = Self::get_android_manifest(context, AndroidStrategy::NativeAab)?;
        config.status_message("Generating", "AndroidManifest.xml")?;
        let manifest_path = save_android_manifest(&native_build_dir, &manifest)?;
        config.status("Preparing resources and assets")?;
        let (assets, resources) =
            Self::prepare_assets_and_resources(&context.config, &android_build_dir)?;

        config.status_message("Compiling", "lib")?;
        let target_sdk_version = Self::target_sdk_version(&manifest, sdk);
        let min_sdk_version = Self::min_sdk_version(&manifest);
        let build_targets = Self::android_build_targets(context, profile, &self.target);
        let compiled_libs = self.build_target(
            context,
            build_targets,
            ndk,
            profile,
            min_sdk_version,
            &target_dir,
            config,
        )?;

        config.status_message("Generating", "proto format APK file")?;

        let compiled_res = if let Some(res) = &resources {
            let compiled_res_path = native_build_dir.join("compiled_res");
            if !compiled_res_path.exists() {
                std::fs::create_dir_all(&compiled_res_path)?;
            }
            let aapt2_compile = sdk.aapt2()?.compile_incremental(
                dunce::simplified(res),
                dunce::simplified(&compiled_res_path),
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
        if let Some(assets) = &assets {
            aapt2_link.assets(assets.clone())
        } else {
            &mut aapt2_link
        }
        .android_jar(sdk.android_jar(target_sdk_version)?)
        .proto_format(true)
        .auto_add_overlay(true)
        .run()?;

        config.status("Extracting apk files")?;
        let output_dir = native_build_dir.join("extracted_apk_files");
        let extracted_apk_path = extract_archive(&apk_path, &output_dir)?;

        config.status("Adding libs")?;
        for (compiled_lib, build_target) in compiled_libs {
            add_libs_into_aapt2(
                ndk,
                &compiled_lib,
                build_target,
                profile,
                Self::min_sdk_version(&manifest),
                &extracted_apk_path,
                &target_dir,
                &package_name,
            )?;
        }

        config.status("Generating ZIP module from extracted files")?;
        let gen_zip_modules =
            gen_zip_modules(&native_build_dir, &package_name, &extracted_apk_path)?;

        for entry in std::fs::read_dir(&native_build_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with(format!("{}_unsigned.aab", package_name)) {
                std::fs::remove_file(&path)?;
            }
        }

        config.status("Generating aab from modules")?;
        let aab_path = gen_aab_from_modules_with_toolchain(
            &package_name,
            &[gen_zip_modules],
            &outputs_build_dir,
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

        let aab_output_path = outputs_build_dir.join(format!("{}_signed.aab", package_name));
        std::fs::rename(aab_path, &aab_output_path)?;
        config.status("Build finished successfully")?;
        Ok((manifest, sdk.clone(), aab_output_path, package_name, key))
    }

    /// Specifies project path and target directory needed to build application.
    pub fn needed_project_dirs(
        example: Option<&String>,
        context: &BuildContext,
    ) -> Result<(PathBuf, PathBuf, String)> {
        let project_path: PathBuf = context.project_path.clone();
        let target_dir: PathBuf = context.target_dir.clone();
        let package_name = if let Some(example) = example {
            example.clone()
        } else {
            context.project.package.name.clone()
        };
        Ok((project_path, target_dir, package_name))
    }

    /// Find keystore for signing application or create it.
    pub fn find_keystore(
        sign_key_path: Option<PathBuf>,
        sign_key_pass: Option<String>,
        sign_key_alias: Option<String>,
    ) -> Result<Key> {
        let key = if let Some(key_path) = sign_key_path {
            let aab_key = Key {
                key_path,
                key_pass: sign_key_pass.unwrap(),
                key_alias: sign_key_alias.unwrap(),
            };
            if aab_key.key_path.exists() {
                aab_key
            } else {
                gen_key(
                    Some(aab_key.key_path),
                    Some(aab_key.key_pass),
                    Some(aab_key.key_alias),
                )?
            }
        } else {
            let aab_key = Key::new_default()?;
            if aab_key.key_path.exists() {
                aab_key
            } else {
                gen_key(
                    Some(aab_key.key_path),
                    Some(aab_key.key_pass),
                    Some(aab_key.key_alias),
                )?
            }
        };
        Ok(key)
    }

    /// Compiling libs for architecture and write out it in vector.
    pub fn build_target(
        &self,
        context: &BuildContext,
        build_targets: Vec<AndroidTarget>,
        ndk: &AndroidNdk,
        profile: Profile,
        min_sdk_version: u32,
        target_dir: &Path,
        config: &Config,
    ) -> Result<Vec<(PathBuf, AndroidTarget)>> {
        let mut libs = Vec::new();
        let cargo_library_name = self.cargo_library_name(context)?;
        for build_target in build_targets {
            let rust_triple = build_target.rust_triple();

            config.status_message("Compiling for architecture", rust_triple)?;
            let compiled_lib = standard_cargo_compile(
                ndk,
                build_target,
                &context.project.package,
                &cargo_library_name,
                profile,
                &self.shared.features,
                self.shared.all_features,
                self.shared.no_default_features,
                min_sdk_version,
                target_dir,
            )?;
            libs.push((compiled_lib, build_target));
        }
        Ok(libs)
    }

    fn cargo_library_name(&self, context: &BuildContext) -> Result<String> {
        let library = context.project.library_target();
        validate_cargo_library_target(
            self.shared.example.as_deref(),
            library.map(|target| (target.name.as_str(), target.is_cdylib())),
        )
    }

    /// Get target sdk version from cargo manifest
    pub fn target_sdk_version(android_manifest: &AndroidManifest, sdk: &AndroidSdk) -> u32 {
        if let Some(target_sdk_version) = android_manifest
            .uses_sdk
            .as_ref()
            .and_then(|u| u.target_sdk_version)
        {
            return target_sdk_version;
        };
        sdk.default_platform()
    }

    /// Get min sdk version from cargo manifest
    pub fn min_sdk_version(android_manifest: &AndroidManifest) -> u32 {
        android_manifest
            .uses_sdk
            .as_ref()
            .and_then(|uses_sdk| uses_sdk.min_sdk_version)
            .unwrap()
    }

    /// Get android build targets from cargo manifest
    pub fn android_build_targets(
        context: &BuildContext,
        profile: Profile,
        build_targets: &[AndroidTarget],
    ) -> Vec<AndroidTarget> {
        if !build_targets.is_empty() {
            return build_targets.into();
        };
        if profile == Profile::Debug && !context.config.android.debug_build_targets.is_empty() {
            return context.config.android.debug_build_targets.clone();
        };
        if profile == Profile::Release && !context.config.android.release_build_targets.is_empty() {
            return context.config.android.release_build_targets.clone();
        };
        vec![AndroidTarget::Aarch64]
    }

    /// Get android manifest from the path in cargo manifest or generate it with the given
    /// configuration
    pub fn get_android_manifest(
        context: &BuildContext,
        strategy: AndroidStrategy,
    ) -> Result<AndroidManifest> {
        let mut manifest = if let Some(manifest_path) = &context.config.android.manifest_path {
            read_android_manifest_with_variables(manifest_path, context.config.build_variables())?
        } else if let Some(manifest) = &context.config.android.manifest {
            manifest.clone()
        } else {
            AndroidManifest::default()
        };
        let library_name = context
            .project
            .library_target()
            .map(|target| target.name.as_str())
            .unwrap_or(&context.project.package.name);
        update_android_manifest_with_default(
            &mut manifest,
            context.config.app_name.clone(),
            library_name,
            strategy,
            context.config.android.runtime,
            context.config.android_uses_crossbow_bridge(),
        );
        context.config.permissions.iter().for_each(|permission| {
            permission.update_manifest(&mut manifest);
        });
        if context.config.icon.is_some() {
            manifest.application.icon = Some(android_manifest::MipmapOrDrawableResource::mipmap(
                "ic_launcher",
                None,
            ));
        }
        Ok(manifest)
    }

    /// Prepare assets and resources for the application.
    ///
    /// Also, this function will generate mipmap icon resources if specified in the
    /// CrossbowMetadata config.
    pub fn prepare_assets_and_resources(
        config: &CrossbowMetadata,
        out_dir: &Path,
    ) -> Result<(Option<PathBuf>, Option<PathBuf>)> {
        let res = config.get_android_resources();
        let gen_resources = if res.is_empty() && config.icon.is_none() {
            None
        } else {
            let path = out_dir.join("gen_resources");
            std::fs::remove_dir_all(&path).ok();
            combine_folders(res, &path)?;

            if let Some(icon) = &config.icon {
                ImageGeneration {
                    icon_path: icon.to_owned(),
                    out_icon_name: "ic_launcher.png".to_owned(),
                    output_path: path.clone(),
                    force: true,
                }
                .gen_mipmap_res_from_icon()?;
            }
            Some(path)
        };

        let assets = config.get_android_assets();
        let gen_assets = if !res.is_empty() {
            let path = out_dir.join("gen_assets");
            std::fs::remove_dir_all(&path).ok();
            combine_folders(assets, &path)?;
            Some(path)
        } else {
            None
        };
        Ok((gen_assets, gen_resources))
    }
}

pub(crate) struct AndroidBuildExecutor<'a> {
    command: &'a AndroidBuildCommand,
    pub(crate) config: &'a Config,
    context: &'a BuildContext,
    sdk: AndroidSdk,
    ndk: AndroidNdk,
    pub(crate) gradle_executable: Option<&'a Path>,
    java: Option<&'a Path>,
    jarsigner: Option<&'a Path>,
    bundletool: Option<&'a Path>,
    pub(crate) artifact: Option<AndroidBuildArtifact>,
}

pub(crate) enum AndroidBuildArtifact {
    NativeApk {
        manifest: AndroidManifest,
        sdk: AndroidSdk,
        path: PathBuf,
    },
    NativeAab {
        manifest: AndroidManifest,
        sdk: AndroidSdk,
        path: PathBuf,
        package: String,
        key: Key,
        apks: Option<PathBuf>,
    },
    Gradle {
        manifest: AndroidManifest,
        sdk: AndroidSdk,
        project: PathBuf,
    },
}

impl<'a> AndroidBuildExecutor<'a> {
    pub(crate) fn new(
        command: &'a AndroidBuildCommand,
        config: &'a Config,
        context: &'a BuildContext,
        plan: &'a crossbundle_tools::toolchain::BuildPlan,
    ) -> Result<Self> {
        Ok(Self {
            command,
            config,
            context,
            sdk: AndroidSdk::from_resolved(
                required_path(plan.toolchain.sdk.as_deref(), "Android SDK")?.to_owned(),
                required_path(plan.toolchain.build_tools.as_deref(), "Android build-tools")?,
                required_path(plan.toolchain.platform.as_deref(), "Android platform")?,
            )?,
            ndk: AndroidNdk::from_path(
                required_path(plan.toolchain.ndk.as_deref(), "Android NDK")?.to_owned(),
            )?,
            gradle_executable: plan.toolchain.gradle.as_deref(),
            java: plan.toolchain.java.as_deref(),
            jarsigner: plan.toolchain.jarsigner.as_deref(),
            bundletool: plan.toolchain.bundletool.as_deref(),
            artifact: None,
        })
    }

    pub(crate) fn bundletool_command(&self) -> Result<std::process::Command> {
        let mut command = std::process::Command::new(required_path(self.java, "Java")?);
        command
            .arg("-jar")
            .arg(required_path(self.bundletool, "bundletool")?);
        Ok(command)
    }

    pub(crate) fn try_run_build_step(
        &mut self,
        kind: crossbundle_tools::toolchain::PlanStepKind,
    ) -> Result<bool> {
        use crossbundle_tools::toolchain::PlanStepKind;
        self.artifact = match kind {
            PlanStepKind::BuildRustLibrary => {
                let name = self.command.lib.as_deref().unwrap_or("crossbow_android");
                self.command
                    .build_rust_lib(self.config, self.context, name, None, &self.ndk)?;
                return Ok(true);
            }
            PlanStepKind::BuildNativeApk => {
                let (manifest, sdk, path) =
                    self.command
                        .execute_apk(self.config, self.context, &self.sdk, &self.ndk)?;
                Some(AndroidBuildArtifact::NativeApk {
                    manifest,
                    sdk,
                    path,
                })
            }
            PlanStepKind::BuildNativeAab => {
                let (manifest, sdk, path, package, key) = self.command.execute_aab(
                    self.config,
                    self.context,
                    &self.sdk,
                    &self.ndk,
                    required_path(self.java, "Java")?,
                    required_path(self.jarsigner, "jarsigner")?,
                    required_path(self.bundletool, "bundletool")?,
                )?;
                Some(AndroidBuildArtifact::NativeAab {
                    manifest,
                    sdk,
                    path,
                    package,
                    key,
                    apks: None,
                })
            }
            PlanStepKind::PrepareGradleProject => {
                let (manifest, sdk, project) = self.command.build_gradle(
                    self.config,
                    self.context,
                    &self.command.export_path,
                    &self.sdk,
                    &self.ndk,
                )?;
                Some(AndroidBuildArtifact::Gradle {
                    manifest,
                    sdk,
                    project,
                })
            }
            PlanStepKind::BuildGradleProject => {
                let Some(AndroidBuildArtifact::Gradle { sdk, project, .. }) =
                    self.artifact.as_ref()
                else {
                    return Err(anyhow::anyhow!("Gradle project was not prepared").into());
                };
                self.config.status("Building Gradle project")?;
                let mut gradle = std::process::Command::new(required_path(
                    self.gradle_executable,
                    "Gradle executable",
                )?);
                gradle
                    .env("ANDROID_SDK_ROOT", sdk.sdk_path())
                    .arg("build")
                    .arg("-p")
                    .arg(dunce::simplified(project));
                gradle.output_err(true)?;
                return Ok(true);
            }
            _ => return Ok(false),
        };
        Ok(true)
    }
}

impl crossbundle_tools::toolchain::Runner for AndroidBuildExecutor<'_> {
    type Error = Error;

    fn run_step(&mut self, step: &crossbundle_tools::toolchain::PlanStep) -> Result<()> {
        self.try_run_build_step(step.kind)?
            .then_some(())
            .ok_or_else(|| anyhow::anyhow!("unexpected {:?} step in build plan", step.kind).into())
    }
}

pub(crate) fn plan_error(error: crossbundle_tools::toolchain::ExecutionError<Error>) -> Error {
    Error::PlanStepFailed {
        step_id: error.step_id,
        source: Box::new(error.source),
    }
}

fn required_path<'a>(path: Option<&'a Path>, name: &str) -> Result<&'a Path> {
    path.ok_or_else(|| anyhow::anyhow!("{name} is absent from build plan").into())
}

fn validate_cargo_library_target(
    example: Option<&str>,
    library: Option<(&str, bool)>,
) -> Result<String> {
    if let Some(example) = example {
        return Err(anyhow::anyhow!(
            "standard Cargo Android builds require a library target, but `--example {example}` \
             selects a binary. Move the mobile entry point into `[lib]` with \
             `crate-type = [\"cdylib\", \"rlib\"]`."
        )
        .into());
    }
    let (name, is_cdylib) = library.ok_or_else(|| {
        anyhow::anyhow!(
            "standard Cargo Android builds require a library target. Add `[lib]` and \
             `crate-type = [\"cdylib\", \"rlib\"]` to Cargo.toml."
        )
    })?;
    if !is_cdylib {
        return Err(anyhow::anyhow!(
            "Cargo library target `{name}` does not produce a cdylib. Add \
             `crate-type = [\"cdylib\", \"rlib\"]` under `[lib]` in Cargo.toml."
        )
        .into());
    }
    Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::validate_cargo_library_target;

    #[test]
    fn accepts_a_renamed_cdylib_target() {
        assert_eq!(
            validate_cargo_library_target(None, Some(("mobile_game", true))).unwrap(),
            "mobile_game"
        );
    }

    #[test]
    fn rejects_binary_examples_on_the_standard_path() {
        let error = validate_cargo_library_target(Some("demo"), Some(("game", true)))
            .unwrap_err()
            .to_string();
        assert!(error.contains("--example demo"));
        assert!(error.contains("library target"));
    }

    #[test]
    fn explains_missing_cdylib_configuration() {
        let error = validate_cargo_library_target(None, Some(("game", false)))
            .unwrap_err()
            .to_string();
        assert!(error.contains("crate-type"));
        assert!(error.contains("cdylib"));
    }
}
