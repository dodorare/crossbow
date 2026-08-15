use super::{CheckStatus, DoctorPlatform, DoctorReport, DoctorRequest, Environment, diagnose};
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

pub const BUILD_PLAN_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanOperation {
    Build,
    Run,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanStrategy {
    GradleApk,
    NativeApk,
    NativeAab,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanRequest {
    pub operation: PlanOperation,
    pub strategy: PlanStrategy,
    pub project_dir: PathBuf,
    pub target_dir: PathBuf,
    pub android_output_dir: PathBuf,
    pub targets: Vec<String>,
    pub release: bool,
    pub attach_logger: bool,
    pub library: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanStepKind {
    BuildRustLibrary,
    BuildNativeApk,
    BuildNativeAab,
    PrepareGradleProject,
    BuildGradleProject,
    GenerateApksArchive,
    InstallArtifact,
    LaunchApplication,
    AttachLogger,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanStep {
    pub id: String,
    pub kind: PlanStepKind,
    pub action: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildPlan {
    pub schema_version: u32,
    pub platform: String,
    pub operation: PlanOperation,
    pub strategy: PlanStrategy,
    pub toolchain: ResolvedAndroidToolchain,
    pub diagnostics: DoctorReport,
    pub steps: Vec<PlanStep>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResolvedAndroidToolchain {
    pub sdk: Option<PathBuf>,
    pub platform: Option<PathBuf>,
    pub build_tools: Option<PathBuf>,
    pub ndk: Option<PathBuf>,
    pub java: Option<PathBuf>,
    pub jarsigner: Option<PathBuf>,
    pub gradle: Option<PathBuf>,
    pub adb: Option<PathBuf>,
    pub bundletool: Option<PathBuf>,
}

pub trait Runner {
    type Error;
    fn run_step(&mut self, step: &PlanStep) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct ExecutionError<E> {
    pub step_id: String,
    pub source: E,
}

impl<E: fmt::Display> fmt::Display for ExecutionError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "plan step {} failed: {}",
            self.step_id, self.source
        )
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ExecutionError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

pub fn plan(request: &PlanRequest, environment: &Environment) -> BuildPlan {
    let mut diagnostics = diagnose(
        &DoctorRequest {
            project: Some(request.project_dir.clone()),
            strict: false,
            platforms: vec![DoctorPlatform::Android],
            targets: request.targets.clone(),
        },
        environment,
    );
    let mut required = Vec::new();
    if request.operation == PlanOperation::Run && request.library.is_none() {
        required.push("android.adb");
    }
    let mut steps = if request.library.is_some() {
        vec![step(
            PlanStepKind::BuildRustLibrary,
            "android.rust.library",
            "Compile the requested Rust library",
        )]
    } else {
        let (tools, kind, id, action) = match request.strategy {
            PlanStrategy::NativeApk => (
                &[][..],
                PlanStepKind::BuildNativeApk,
                "android.apk.build",
                "Compile, package, align, and sign the APK",
            ),
            PlanStrategy::NativeAab => (
                &["android.bundletool", "host.java.jarsigner"][..],
                PlanStepKind::BuildNativeAab,
                "android.aab.build",
                "Compile, package, and sign the Android App Bundle",
            ),
            PlanStrategy::GradleApk => (
                &["host.gradle"][..],
                PlanStepKind::PrepareGradleProject,
                "android.gradle.prepare",
                "Generate the Gradle project and compile Rust libraries",
            ),
        };
        required.extend(tools);
        vec![step(kind, id, action)]
    };
    for check in diagnostics
        .checks
        .iter_mut()
        .filter(|check| required.contains(&check.id.as_str()))
    {
        check.required = true;
        let missing = check
            .found
            .as_ref()
            .and_then(|found| found.path.as_ref())
            .is_none_or(|path| !path.exists());
        if check.status == CheckStatus::Skip || missing {
            check.status = CheckStatus::Fail;
            check.summary = "Tool is required by this plan but was not found".into();
        }
    }
    diagnostics.recompute();
    let toolchain = ResolvedAndroidToolchain {
        sdk: observed_path(&diagnostics, "android.sdk.root"),
        platform: observed_path(&diagnostics, "android.sdk.platform"),
        build_tools: observed_path(&diagnostics, "android.sdk.build_tools"),
        ndk: observed_path(&diagnostics, "android.ndk"),
        java: observed_path(&diagnostics, "host.java.runtime"),
        jarsigner: observed_path(&diagnostics, "host.java.jarsigner"),
        gradle: observed_path(&diagnostics, "host.gradle"),
        adb: observed_path(&diagnostics, "android.adb"),
        bundletool: observed_path(&diagnostics, "android.bundletool"),
    };
    if request.operation == PlanOperation::Build
        && request.strategy == PlanStrategy::GradleApk
        && request.library.is_none()
    {
        steps.push(step(
            PlanStepKind::BuildGradleProject,
            "android.gradle.build",
            "Build the generated Gradle project",
        ));
    }
    if request.operation == PlanOperation::Run && request.library.is_none() {
        if request.strategy == PlanStrategy::NativeAab && request.library.is_none() {
            steps.push(step(
                PlanStepKind::GenerateApksArchive,
                "android.apks.generate",
                "Generate an installable APK set from the bundle",
            ));
        }
        steps.push(step(
            PlanStepKind::InstallArtifact,
            "android.device.install",
            "Install application on selected device",
        ));
        steps.push(step(
            PlanStepKind::LaunchApplication,
            "android.device.launch",
            "Launch application",
        ));
        if request.attach_logger {
            steps.push(step(
                PlanStepKind::AttachLogger,
                "android.device.log",
                "Attach application logger",
            ));
        }
    }
    BuildPlan {
        schema_version: BUILD_PLAN_SCHEMA_VERSION,
        platform: "android".into(),
        operation: request.operation,
        strategy: request.strategy,
        toolchain,
        diagnostics,
        steps,
    }
}

fn observed_path(report: &DoctorReport, id: &str) -> Option<PathBuf> {
    report
        .checks
        .iter()
        .find(|check| check.id == id && check.status != CheckStatus::Fail)?
        .found
        .as_ref()?
        .path
        .clone()
}

pub fn execute<R: Runner>(
    plan: &BuildPlan,
    runner: &mut R,
) -> Result<(), ExecutionError<R::Error>> {
    for step in &plan.steps {
        runner.run_step(step).map_err(|source| ExecutionError {
            step_id: step.id.clone(),
            source,
        })?;
    }
    Ok(())
}

fn step(kind: PlanStepKind, id: &str, action: &str) -> PlanStep {
    PlanStep {
        id: id.into(),
        kind,
        action: action.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(operation: PlanOperation, strategy: PlanStrategy) -> PlanRequest {
        PlanRequest {
            operation,
            strategy,
            project_dir: "project".into(),
            target_dir: "target".into(),
            android_output_dir: "target/android/example".into(),
            targets: vec!["aarch64-linux-android".into()],
            release: false,
            attach_logger: false,
            library: None,
        }
    }

    fn ids(plan: &BuildPlan) -> Vec<&str> {
        plan.steps.iter().map(|step| step.id.as_str()).collect()
    }

    #[test]
    fn plan_shapes_cover_every_android_strategy_and_operation() {
        let environment = Environment::default();
        assert_eq!(
            ids(&plan(
                &request(PlanOperation::Build, PlanStrategy::NativeApk),
                &environment
            )),
            ["android.apk.build"]
        );
        assert_eq!(
            ids(&plan(
                &request(PlanOperation::Build, PlanStrategy::NativeAab),
                &environment
            )),
            ["android.aab.build"]
        );
        assert_eq!(
            ids(&plan(
                &request(PlanOperation::Build, PlanStrategy::GradleApk),
                &environment
            )),
            ["android.gradle.prepare", "android.gradle.build"]
        );
        assert_eq!(
            ids(&plan(
                &request(PlanOperation::Run, PlanStrategy::NativeAab),
                &environment
            )),
            [
                "android.aab.build",
                "android.apks.generate",
                "android.device.install",
                "android.device.launch",
            ]
        );
    }

    #[test]
    fn each_strategy_requires_the_tools_it_executes() {
        let environment = Environment::default();
        for (strategy, id) in [
            (PlanStrategy::NativeAab, "android.bundletool"),
            (PlanStrategy::GradleApk, "host.gradle"),
        ] {
            let plan = plan(&request(PlanOperation::Build, strategy), &environment);
            let check = plan.diagnostics.checks.iter().find(|c| c.id == id).unwrap();
            assert!(check.required);
            assert_eq!(check.status, CheckStatus::Fail);
        }
    }

    #[test]
    fn library_plan_does_not_require_unused_packaging_tools() {
        let mut request = request(PlanOperation::Build, PlanStrategy::NativeAab);
        request.library = Some("game".into());
        let plan = plan(&request, &Environment::default());
        assert_eq!(ids(&plan), ["android.rust.library"]);
        assert!(
            !plan
                .diagnostics
                .checks
                .iter()
                .find(|check| check.id == "android.bundletool")
                .unwrap()
                .required
        );
    }

    #[test]
    fn run_library_plan_does_not_add_device_steps_without_an_artifact() {
        let mut request = request(PlanOperation::Run, PlanStrategy::NativeAab);
        request.library = Some("game".into());
        let plan = plan(&request, &Environment::default());
        assert_eq!(ids(&plan), ["android.rust.library"]);
        assert!(
            !plan
                .diagnostics
                .checks
                .iter()
                .find(|check| check.id == "android.adb")
                .unwrap()
                .required
        );
    }

    #[test]
    fn run_plan_extends_the_build_plan_without_side_effects() {
        let temp = tempfile::tempdir().unwrap();
        let request = PlanRequest {
            operation: PlanOperation::Run,
            strategy: PlanStrategy::NativeApk,
            project_dir: temp.path().join("project"),
            target_dir: temp.path().join("target"),
            android_output_dir: temp.path().join("target/android/example"),
            targets: vec!["aarch64-linux-android".into()],
            release: false,
            attach_logger: true,
            library: None,
        };
        let result = plan(&request, &Environment::default());
        assert!(result.steps.iter().any(|s| s.id == "android.device.log"));
        assert!(!request.target_dir.exists());
    }

    #[test]
    fn execute_passes_the_exact_immutable_plan_to_the_runner() {
        struct RecordingRunner(Vec<String>);
        impl Runner for RecordingRunner {
            type Error = std::convert::Infallible;
            fn run_step(&mut self, step: &PlanStep) -> Result<(), Self::Error> {
                self.0.push(step.id.clone());
                Ok(())
            }
        }
        let request = PlanRequest {
            operation: PlanOperation::Build,
            strategy: PlanStrategy::GradleApk,
            project_dir: "project".into(),
            target_dir: "target".into(),
            android_output_dir: "target/android/example".into(),
            targets: vec![],
            release: false,
            attach_logger: false,
            library: None,
        };
        let plan = plan(&request, &Environment::default());
        let mut runner = RecordingRunner(Vec::new());
        execute(&plan, &mut runner).unwrap();
        assert_eq!(
            runner.0,
            plan.steps
                .iter()
                .map(|step| step.id.clone())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn execute_attributes_failure_to_the_exact_step() {
        struct FailingRunner;
        impl Runner for FailingRunner {
            type Error = &'static str;
            fn run_step(&mut self, step: &PlanStep) -> Result<(), Self::Error> {
                (step.kind != PlanStepKind::BuildGradleProject)
                    .then_some(())
                    .ok_or("boom")
            }
        }
        let request = PlanRequest {
            operation: PlanOperation::Build,
            strategy: PlanStrategy::GradleApk,
            project_dir: "project".into(),
            target_dir: "target".into(),
            android_output_dir: "target/android/example".into(),
            targets: vec![],
            release: false,
            attach_logger: false,
            library: None,
        };
        let plan = plan(&request, &Environment::default());
        let error = execute(&plan, &mut FailingRunner).unwrap_err();
        assert_eq!(error.step_id, "android.gradle.build");
    }
}
