use super::{BuildContext, SharedBuildCommand, validate_android_activity_runtime};
use crate::{error::*, types::ProjectConfig};
use android_manifest::AndroidManifest;
use android_tools::java_tools::Key;
use clap::{ArgAction, Parser};
use crossbundle_tools::{
    commands::{android::*, combine_folders},
    error::CommandExt,
    types::*,
};
use std::path::{Path, PathBuf};

mod native;

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
    /// Builds the application with the selected Android strategy.
    pub fn run(&self, config: &CliContext) -> Result<()> {
        let context = BuildContext::new(config, &self.shared)?;
        for target in Self::android_build_targets(&context, self.shared.profile(), &self.target) {
            validate_android_activity_runtime(
                &context.project,
                &context.project_config,
                &self.shared,
                target.rust_triple(),
            )?;
        }
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
                runtime: context.project_config.android.runtime,
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
        config: &CliContext,
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
            Self::prepare_assets_and_resources(&context.project_config, &android_build_dir)?;
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
            &context.project_config.android.plugins,
            context.project_config.android.runtime,
            &library_name,
            &context.project,
            context.project_config.android_uses_crossbow_bridge(),
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
        config: &CliContext,
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
        gen_key(sign_key_path, sign_key_pass, sign_key_alias).map_err(Into::into)
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
        config: &CliContext,
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
    pub fn target_sdk_version(android_manifest: &AndroidManifest, sdk: &AndroidSdk) -> Result<u32> {
        if let Some(target_sdk_version) = android_manifest
            .uses_sdk
            .as_ref()
            .and_then(|u| u.target_sdk_version)
        {
            return Ok(target_sdk_version);
        };
        Ok(sdk.default_platform()?)
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
        if profile == Profile::Debug
            && !context
                .project_config
                .android
                .debug_build_targets
                .is_empty()
        {
            return context.project_config.android.debug_build_targets.clone();
        };
        if profile == Profile::Release
            && !context
                .project_config
                .android
                .release_build_targets
                .is_empty()
        {
            return context.project_config.android.release_build_targets.clone();
        };
        vec![AndroidTarget::Aarch64]
    }

    /// Get android manifest from the path in cargo manifest or generate it with the given
    /// configuration
    pub fn get_android_manifest(
        context: &BuildContext,
        strategy: AndroidStrategy,
    ) -> Result<AndroidManifest> {
        let mut manifest =
            if let Some(manifest_path) = &context.project_config.android.manifest_path {
                read_android_manifest_with_variables(
                    manifest_path,
                    context.project_config.build_variables(),
                )?
            } else if let Some(manifest) = &context.project_config.android.manifest {
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
            context.project_config.app_name.clone(),
            library_name,
            strategy,
            context.project_config.android.runtime,
            context.project_config.android_uses_crossbow_bridge(),
        );
        context
            .project_config
            .permissions
            .iter()
            .for_each(|permission| {
                permission.update_manifest(&mut manifest);
            });
        if context.project_config.icon.is_some() {
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
    /// Project configuration.
    pub fn prepare_assets_and_resources(
        config: &ProjectConfig,
        out_dir: &Path,
    ) -> Result<(Option<PathBuf>, Option<PathBuf>)> {
        let res = config.android_resources();
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

        let assets = config.android_assets();
        let gen_assets = if !assets.is_empty() {
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
    pub(crate) config: &'a CliContext,
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
        config: &'a CliContext,
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
    use super::{AndroidBuildCommand, ProjectConfig, validate_cargo_library_target};

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

    #[test]
    fn prepares_assets_without_requiring_resources() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("assets");
        std::fs::create_dir(&source).unwrap();
        std::fs::write(source.join("data.bin"), b"asset").unwrap();
        let mut config = ProjectConfig::default();
        config.assets.push(source);

        let (assets, resources) =
            AndroidBuildCommand::prepare_assets_and_resources(&config, temp.path()).unwrap();

        assert!(assets.unwrap().join("data.bin").is_file());
        assert!(resources.is_none());
    }
}
