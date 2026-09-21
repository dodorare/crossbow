use super::SharedBuildCommand;
use crate::{error::*, types::*};
#[cfg(feature = "android")]
use crossbundle_tools::types::AndroidRuntime;
use crossbundle_tools::{
    commands::*,
    types::{CliContext, parse_project_config},
};
use std::path::PathBuf;

pub struct BuildContext {
    // Paths
    pub project_path: PathBuf,
    pub target_dir: PathBuf,
    // Configurations
    pub project: CargoProject,
    pub project_config: ProjectConfig,
}

impl BuildContext {
    /// Create new instance of build context
    pub fn new(context: &CliContext, command: &SharedBuildCommand) -> Result<Self> {
        info!("Reading Cargo metadata");
        let loaded = LoadedProject::load_with_features(
            context.current_dir(),
            &command.features,
            command.all_features,
            command.no_default_features,
        )?;
        let project_path = loaded.root;
        let project = loaded.cargo;
        let target_dir = match &command.target_dir {
            Some(path) if path.is_relative() => {
                std::path::absolute(context.current_dir().join(path))?
            }
            Some(path) => path.clone(),
            None => project.target_directory.clone(),
        };
        let mut project_config = parse_project_config(project.package.metadata.clone())
            .and_then(|metadata| metadata.resolve())
            .map_err(Error::InvalidMetadata)?;
        project_config.resolve_paths(&project_path);
        Ok(Self {
            project_path,
            target_dir,
            project_config,
            project,
        })
    }
}

#[cfg(feature = "android")]
pub(super) fn validate_android_activity_runtime(
    project: &CargoProject,
    config: &ProjectConfig,
    command: &SharedBuildCommand,
    target: &str,
) -> Result<()> {
    let runtime = config.android.runtime;
    if runtime == AndroidRuntime::Miniquad {
        return Ok(());
    }
    let Some(features) = project.target_dependency_features(
        "android-activity",
        target,
        &command.features,
        command.all_features,
        command.no_default_features,
    )?
    else {
        // Custom runtimes may implement the native Activity ABI without android-activity.
        return Ok(());
    };
    validate_android_activity_features(runtime, &features)
}

#[cfg(feature = "android")]
fn validate_android_activity_features(runtime: AndroidRuntime, features: &[String]) -> Result<()> {
    let expected = match runtime {
        AndroidRuntime::NativeActivity => "native-activity",
        AndroidRuntime::GameActivity => "game-activity",
        AndroidRuntime::Miniquad => return Ok(()),
    };
    if features.iter().any(|feature| feature == expected)
        && !features.iter().any(|feature| {
            matches!(feature.as_str(), "native-activity" | "game-activity") && feature != expected
        })
    {
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "Android runtime `{}` requires the resolved `android-activity` feature `{expected}`, but its activated features are [{}]. Align `package.metadata.android.runtime` with the Bevy or android-activity Android feature.",
        runtime.as_str(),
        features.join(", ")
    )
    .into())
}

#[cfg(all(test, feature = "android"))]
mod tests {
    use super::*;

    fn features(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn accepts_matching_android_activity_runtime_features() {
        validate_android_activity_features(
            AndroidRuntime::NativeActivity,
            &features(&["native-activity"]),
        )
        .unwrap();
        validate_android_activity_features(
            AndroidRuntime::GameActivity,
            &features(&["game-activity"]),
        )
        .unwrap();
        validate_android_activity_features(
            AndroidRuntime::Miniquad,
            &features(&["native-activity", "game-activity"]),
        )
        .unwrap();
    }

    #[test]
    fn rejects_mismatched_or_ambiguous_android_activity_runtime_features() {
        let mismatch = validate_android_activity_features(
            AndroidRuntime::GameActivity,
            &features(&["native-activity"]),
        )
        .unwrap_err()
        .to_string();
        assert!(
            mismatch.contains("requires the resolved `android-activity` feature `game-activity`")
        );

        let ambiguous = validate_android_activity_features(
            AndroidRuntime::GameActivity,
            &features(&["native-activity", "game-activity"]),
        )
        .unwrap_err()
        .to_string();
        assert!(ambiguous.contains("native-activity, game-activity"));
    }
}
