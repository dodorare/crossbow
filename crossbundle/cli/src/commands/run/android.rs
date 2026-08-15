use crate::commands::build::{
    BuildContext,
    android::{AndroidBuildArtifact, AndroidBuildCommand, AndroidBuildExecutor, plan_error},
};
use crate::error::*;
use clap::Parser;
use crossbundle_tools::{commands::android::*, error::CommandExt, types::Config};

#[derive(Parser, Clone, Debug)]
pub struct AndroidRunCommand {
    #[clap(flatten)]
    pub build_command: AndroidBuildCommand,
    /// Enable logging attach after run.
    #[clap(long)]
    pub log: bool,
}

impl AndroidRunCommand {
    /// Deployes and runs application in AAB or APK format on your device or emulator
    pub fn run(&self, config: &Config) -> Result<()> {
        if self.build_command.lib.is_some() {
            config.status("Can not run dynamic library")?;
            return Ok(());
        }
        let context = BuildContext::new(config, self.build_command.shared.target_dir.clone())?;
        let plan = self.build_command.create_plan(
            &context,
            crossbundle_tools::toolchain::PlanOperation::Run,
            self.log,
        );
        if self.build_command.dry_run {
            self.build_command.print_plan(&plan)?;
            return self.build_command.ensure_plan_valid(&plan);
        }
        self.build_command.ensure_plan_valid(&plan)?;
        let mut runner = AndroidRunPlanRunner {
            build: AndroidBuildExecutor::new(&self.build_command, config, &context, &plan)?,
        };
        crossbundle_tools::toolchain::execute(&plan, &mut runner).map_err(plan_error)?;
        config.status("Run finished successfully")?;
        Ok(())
    }
}

struct AndroidRunPlanRunner<'a> {
    build: AndroidBuildExecutor<'a>,
}

impl crossbundle_tools::toolchain::Runner for AndroidRunPlanRunner<'_> {
    type Error = Error;

    fn run_step(&mut self, step: &crossbundle_tools::toolchain::PlanStep) -> Result<()> {
        use crossbundle_tools::toolchain::PlanStepKind;
        if self.build.try_run_build_step(step.kind)? {
            return Ok(());
        }
        match step.kind {
            PlanStepKind::GenerateApksArchive => {
                let mut command = self.build.bundletool_command()?;
                let Some(AndroidBuildArtifact::NativeAab {
                    path,
                    package,
                    key,
                    apks,
                    ..
                }) = self.build.artifact.as_mut()
                else {
                    return Err(anyhow::anyhow!("AAB artifact was not built").into());
                };
                self.build.config.status("Generating apks")?;
                let output = path.parent().unwrap().join(format!("{package}.apks"));
                command
                    .arg("build-apks")
                    .arg("--bundle")
                    .arg(path)
                    .arg("--output")
                    .arg(&output)
                    .arg("--overwrite")
                    .arg("--ks")
                    .arg(&key.key_path)
                    .arg("--ks-pass")
                    .arg(format!("pass:{}", key.key_pass))
                    .arg("--ks-key-alias")
                    .arg(&key.key_alias);
                command.output_err(true)?;
                *apks = Some(output);
            }
            PlanStepKind::InstallArtifact => match self
                .build
                .artifact
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("installable artifact was not built"))?
            {
                AndroidBuildArtifact::NativeApk { sdk, path, .. } => {
                    self.build.config.status("Installing APK file")?;
                    install_apk(sdk, path)?;
                }
                AndroidBuildArtifact::NativeAab { apks, .. } => {
                    self.build.config.status("Installing APKs file")?;
                    let apks = apks
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("APKs archive was not generated"))?;
                    let mut command = self.build.bundletool_command()?;
                    command.arg("install-apks").arg("--apks").arg(apks);
                    command.output_err(true)?;
                }
                AndroidBuildArtifact::Gradle { project, .. } => {
                    self.build.config.status("Installing APK file on device")?;
                    let gradle = self.build.gradle_executable.ok_or_else(|| {
                        anyhow::anyhow!("Gradle executable is absent from build plan")
                    })?;
                    let mut gradle = std::process::Command::new(gradle);
                    gradle
                        .arg("installDebug")
                        .arg("-p")
                        .arg(dunce::simplified(project));
                    gradle.output_err(true)?;
                }
            },
            PlanStepKind::LaunchApplication => {
                self.build.config.status("Starting APK file")?;
                match self
                    .build
                    .artifact
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("launchable artifact was not built"))?
                {
                    AndroidBuildArtifact::NativeApk { manifest, sdk, .. }
                    | AndroidBuildArtifact::NativeAab { manifest, sdk, .. } => {
                        let package = manifest.package.as_deref().ok_or_else(|| {
                            anyhow::anyhow!("Android manifest package is missing")
                        })?;
                        start_app(sdk, package, "android.app.NativeActivity")?;
                    }
                    AndroidBuildArtifact::Gradle { sdk, .. } => {
                        start_app(sdk, "com.crossbow.game", ".CrossbowApp")?
                    }
                }
            }
            PlanStepKind::AttachLogger => {
                self.build.config.status("Attaching logger")?;
                std::thread::sleep(std::time::Duration::from_secs(2));
                let sdk = match self
                    .build
                    .artifact
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("artifact was not built"))?
                {
                    AndroidBuildArtifact::NativeApk { sdk, .. }
                    | AndroidBuildArtifact::NativeAab { sdk, .. }
                    | AndroidBuildArtifact::Gradle { sdk, .. } => sdk,
                };
                attach_logger_only_app(sdk)?;
            }
            _ => return Err(anyhow::anyhow!("unexpected {:?} step in run plan", step.kind).into()),
        }
        Ok(())
    }
}
