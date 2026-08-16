use super::*;
#[cfg(feature = "apple")]
use crate::types::AndroidGradlePlugins;
use crate::{
    commands::*,
    types::{CrossbowMetadata, deserialize_crossbow_metadata},
};

pub(super) struct ProjectContext {
    pub(super) manifest_path: PathBuf,
    pub(super) state: ProjectState,
}

pub(super) enum ProjectState {
    Missing,
    Invalid,
    Valid(Box<Project>),
}

pub(super) struct Project {
    pub(super) package_name: String,
    pub(super) metadata_present: bool,
    pub(super) metadata: Result<CrossbowMetadata, ()>,
    #[cfg(feature = "apple")]
    pub(super) apple_metadata_present: bool,
    #[cfg(feature = "apple")]
    pub(super) android_plugins: Vec<String>,
}

impl ProjectContext {
    pub(super) fn load(path: &Path, platforms: &[DoctorPlatform]) -> Self {
        let requested_manifest_path = if path.file_name().is_some_and(|name| name == "Cargo.toml") {
            path.to_owned()
        } else {
            path.join("Cargo.toml")
        };
        if !path.is_dir() && !requested_manifest_path.is_file() {
            return Self {
                manifest_path: requested_manifest_path,
                state: ProjectState::Missing,
            };
        }
        let manifest_path = if path.file_name().is_some_and(|name| name == "Cargo.toml") {
            requested_manifest_path.clone()
        } else {
            match find_package_cargo_manifest_path(path) {
                Ok(path) => path,
                Err(_) => {
                    return Self {
                        manifest_path: requested_manifest_path.clone(),
                        state: if requested_manifest_path.is_file() {
                            ProjectState::Invalid
                        } else {
                            ProjectState::Missing
                        },
                    };
                }
            }
        };
        let manifest_path = dunce::canonicalize(&manifest_path).unwrap_or(manifest_path);
        let Ok(cargo_project) = CargoProject::load_package(&manifest_path) else {
            return Self {
                manifest_path,
                state: ProjectState::Invalid,
            };
        };
        let manifest = &cargo_project.package;
        let metadata_present = manifest
            .metadata
            .as_object()
            .is_some_and(|metadata| !metadata.is_empty());
        #[cfg(feature = "apple")]
        let apple_metadata_present = manifest.metadata.get("apple").is_some();
        let custom_metadata = &manifest.metadata;
        let metadata = typed_metadata(custom_metadata, platforms);
        #[cfg(feature = "apple")]
        let android_plugins = custom_metadata
            .get("android")
            .cloned()
            .and_then(|metadata| serde_json::from_value::<AndroidGradlePlugins>(metadata).ok())
            .map(|plugins| plugin_names(&plugins))
            .unwrap_or_default();
        Self {
            manifest_path,
            state: ProjectState::Valid(Box::new(Project {
                package_name: manifest.name.clone(),
                metadata_present,
                metadata,
                #[cfg(feature = "apple")]
                apple_metadata_present,
                #[cfg(feature = "apple")]
                android_plugins,
            })),
        }
    }

    pub(super) fn base_dir(&self) -> &Path {
        self.manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
    }

    pub(super) fn project(&self) -> Option<&Project> {
        match &self.state {
            ProjectState::Valid(project) => Some(project),
            ProjectState::Missing | ProjectState::Invalid => None,
        }
    }

    #[cfg(feature = "apple")]
    pub(super) fn metadata(&self) -> Option<&CrossbowMetadata> {
        self.project()?.metadata.as_ref().ok()
    }

    pub(super) fn common_checks(&self) -> Vec<DoctorCheck> {
        match &self.state {
            ProjectState::Missing => vec![check(
                "project.cargo.manifest",
                CheckStatus::Fail,
                "Project",
                "Cargo.toml was not found".into(),
                true,
                Some(ObservedValue {
                    version: None,
                    path: Some(self.manifest_path.clone()),
                }),
                None,
                Some("Pass an explicit project directory or Cargo.toml path".into()),
            )],
            ProjectState::Invalid => vec![check(
                "project.cargo.manifest",
                CheckStatus::Fail,
                "Project",
                "Cargo.toml is not a valid selected Cargo package manifest".into(),
                true,
                Some(ObservedValue {
                    version: None,
                    path: Some(self.manifest_path.clone()),
                }),
                None,
                Some("Fix the selected package manifest or pass its Cargo.toml".into()),
            )],
            ProjectState::Valid(project) => {
                let (metadata_status, metadata_summary) =
                    match (project.metadata.is_ok(), project.metadata_present) {
                        (false, _) => (CheckStatus::Fail, "Crossbow metadata is invalid"),
                        (true, true) => (CheckStatus::Pass, "Crossbow metadata is valid"),
                        (true, false) => (
                            CheckStatus::Warn,
                            "Crossbow metadata is absent; defaults will be used",
                        ),
                    };
                vec![
                    check(
                        "project.cargo.manifest",
                        CheckStatus::Pass,
                        "Project",
                        "Cargo.toml is a valid package manifest".into(),
                        true,
                        Some(ObservedValue {
                            version: None,
                            path: Some(self.manifest_path.clone()),
                        }),
                        None,
                        None,
                    ),
                    check(
                        "project.cargo.package",
                        CheckStatus::Pass,
                        "Project",
                        format!("Selected Cargo package {}", project.package_name),
                        true,
                        None,
                        None,
                        None,
                    ),
                    check(
                        "project.crossbow.metadata",
                        metadata_status,
                        "Project",
                        metadata_summary.into(),
                        false,
                        None,
                        None,
                        project
                            .metadata
                            .is_err()
                            .then(|| "Fix typed package.metadata fields".into()),
                    ),
                ]
            }
        }
    }
}

fn typed_metadata(
    metadata: &serde_json::Value,
    platforms: &[DoctorPlatform],
) -> Result<CrossbowMetadata, ()> {
    let mut metadata = metadata.clone();
    if let Some(table) = metadata.as_object_mut() {
        for platform in [DoctorPlatform::Android, DoctorPlatform::Apple] {
            if !platforms.contains(&platform) {
                table.remove(platform.canonical_name());
            }
        }
    }
    deserialize_crossbow_metadata(metadata).map_err(|_| ())
}

#[cfg(feature = "apple")]
fn plugin_names(plugins: &AndroidGradlePlugins) -> Vec<String> {
    let mut names = plugins
        .local
        .iter()
        .filter_map(|path| path.file_stem().and_then(|name| name.to_str()))
        .map(str::to_owned)
        .chain(plugins.remote.iter().map(|coordinate| {
            coordinate
                .split(':')
                .nth(1)
                .unwrap_or(coordinate)
                .to_owned()
        }))
        .chain(
            plugins
                .local_projects
                .iter()
                .map(|project| project.include().trim_matches(':').replace(':', "-")),
        )
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();
    names
}
