use crate::error::*;
use crate::types::{CargoTargetSelection, is_library_kind};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::Command,
};

/// Cargo's public description of the selected package and its resolved dependencies.
#[derive(Debug, Clone)]
pub struct CargoProject {
    pub workspace_manifest_path: PathBuf,
    pub target_directory: PathBuf,
    pub package: CargoPackage,
    packages: HashMap<String, CargoPackage>,
    dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CargoPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub default_run: Option<String>,
    pub targets: Vec<CargoTarget>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CargoTarget {
    pub name: String,
    pub kind: Vec<String>,
    pub crate_types: Vec<String>,
}

/// A selected Cargo package with its canonical manifest and project root.
pub struct LoadedProject {
    pub manifest_path: PathBuf,
    pub root: PathBuf,
    pub cargo: CargoProject,
}

impl LoadedProject {
    pub(crate) fn discover_manifest(path: &Path) -> Result<PathBuf> {
        let manifest_path = if path.file_name().is_some_and(|name| name == "Cargo.toml") {
            path.to_owned()
        } else {
            crate::commands::find_package_cargo_manifest_path(path)?
        };
        Ok(canonical(&manifest_path))
    }

    /// Discovers and loads a project using the build's Cargo feature selection.
    pub fn load_with_features(
        path: &Path,
        features: &[String],
        all_features: bool,
        no_default_features: bool,
    ) -> Result<Self> {
        Self::load(path, |manifest_path| {
            CargoProject::load_with_features(
                manifest_path,
                features,
                all_features,
                no_default_features,
            )
        })
    }

    /// Discovers and loads package metadata without resolving dependencies.
    pub fn load_package(path: &Path) -> Result<Self> {
        Self::load(path, CargoProject::load_package)
    }

    fn load(path: &Path, loader: impl FnOnce(&Path) -> Result<CargoProject>) -> Result<Self> {
        let manifest_path = Self::discover_manifest(path)?;
        let cargo = loader(&manifest_path)?;
        let manifest_path = canonical(&cargo.package.manifest_path);
        let root = manifest_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cargo manifest has no parent directory"))?
            .to_owned();
        Ok(Self {
            manifest_path,
            root,
            cargo,
        })
    }
}

impl CargoTarget {
    pub fn is_library(&self) -> bool {
        self.kind.iter().any(|kind| is_library_kind(kind))
    }

    pub fn is_cdylib(&self) -> bool {
        self.crate_types.iter().any(|kind| kind == "cdylib")
    }

    fn is_executable(&self, kind: &str) -> bool {
        self.kind.iter().any(|target_kind| target_kind == kind)
            && self
                .crate_types
                .iter()
                .any(|crate_type| crate_type == "bin")
    }
}

impl CargoProject {
    /// Load Cargo metadata and select the package identified by `manifest_path`.
    pub fn load(manifest_path: &Path) -> Result<Self> {
        Self::load_with_features(manifest_path, &[], false, false)
    }

    /// Load Cargo metadata using the same feature selection as the build.
    pub fn load_with_features(
        manifest_path: &Path,
        features: &[String],
        all_features: bool,
        no_default_features: bool,
    ) -> Result<Self> {
        Self::load_metadata(
            manifest_path,
            true,
            features,
            all_features,
            no_default_features,
        )
    }

    /// Load package metadata without resolving dependencies or writing a lockfile.
    pub fn load_package(manifest_path: &Path) -> Result<Self> {
        Self::load_metadata(manifest_path, false, &[], false, false)
    }

    fn load_metadata(
        manifest_path: &Path,
        dependencies: bool,
        features: &[String],
        all_features: bool,
        no_default_features: bool,
    ) -> Result<Self> {
        let mut command = Command::new("cargo");
        command
            .arg("metadata")
            .args(["--format-version", "1"])
            .arg("--manifest-path")
            .arg(manifest_path);
        if !dependencies {
            command.arg("--no-deps");
        }
        if !features.is_empty() {
            command.arg("--features").arg(features.join(","));
        }
        if all_features {
            command.arg("--all-features");
        }
        if no_default_features {
            command.arg("--no-default-features");
        }
        if let Some(project_dir) = manifest_path.parent() {
            command.current_dir(project_dir);
        }
        let output = command.output()?;
        if !output.status.success() {
            return Err(Error::CmdFailed(
                command,
                String::from_utf8_lossy(&output.stdout).into_owned(),
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ));
        }
        let metadata: Metadata = serde_json::from_slice(&output.stdout)
            .map_err(|error| anyhow::anyhow!("invalid output from `cargo metadata`: {error}"))?;
        Self::from_metadata(manifest_path, metadata)
    }

    fn from_metadata(manifest_path: &Path, metadata: Metadata) -> Result<Self> {
        let selected_path = canonical(manifest_path);
        let package = metadata
            .packages
            .iter()
            .find(|package| canonical(&package.manifest_path) == selected_path)
            .cloned()
            .ok_or_else(|| Error::FailedToFindManifest(manifest_path.to_owned()))?;
        let packages = metadata
            .packages
            .into_iter()
            .map(|package| (package.id.clone(), package))
            .collect();
        let dependencies = metadata
            .resolve
            .map(|resolve| {
                resolve
                    .nodes
                    .into_iter()
                    .map(|node| (node.id, node.dependencies))
                    .collect()
            })
            .unwrap_or_default();
        Ok(Self {
            workspace_manifest_path: metadata.workspace_root.join("Cargo.toml"),
            target_directory: metadata.target_directory,
            package,
            packages,
            dependencies,
        })
    }

    pub fn library_target(&self) -> Option<&CargoTarget> {
        self.package
            .targets
            .iter()
            .find(|target| target.is_library())
    }

    /// Select the executable Cargo target that should become an application bundle.
    pub fn executable_target(
        &self,
        bin: Option<&str>,
        example: Option<&str>,
    ) -> Result<CargoTargetSelection> {
        if let Some(name) = example {
            return self.named_executable(CargoTargetSelection::Example(name.to_owned()));
        }
        if let Some(name) = bin.or(self.package.default_run.as_deref()) {
            return self.named_executable(CargoTargetSelection::Bin(name.to_owned()));
        }

        let binaries = self
            .package
            .targets
            .iter()
            .filter(|target| target.is_executable("bin"))
            .collect::<Vec<_>>();
        match binaries.as_slice() {
            [target] => Ok(CargoTargetSelection::Bin(target.name.clone())),
            [] => Err(anyhow::anyhow!(
                "Cargo package `{}` has no executable binary target. Add `src/main.rs`, or select an executable example with `--example`.",
                self.package.name
            )
            .into()),
            targets => Err(anyhow::anyhow!(
                "Cargo package `{}` has multiple binary targets: {}. Select one with `--bin`, or set `package.default-run` in Cargo.toml.",
                self.package.name,
                targets
                    .iter()
                    .map(|target| target.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into()),
        }
    }

    fn named_executable(&self, selection: CargoTargetSelection) -> Result<CargoTargetSelection> {
        let name = selection.name();
        let kind = selection.kind();
        if self
            .package
            .targets
            .iter()
            .any(|target| target.name == name && target.is_executable(kind))
        {
            return Ok(selection);
        }
        let available = self
            .package
            .targets
            .iter()
            .filter(|target| target.is_executable(kind))
            .map(|target| target.name.as_str())
            .collect::<Vec<_>>();
        let available = if available.is_empty() {
            "none".to_owned()
        } else {
            available.join(", ")
        };
        Err(anyhow::anyhow!(
            "Cargo package `{}` has no executable {kind} target `{name}`. Available {kind} targets: {available}.",
            self.package.name
        )
        .into())
    }

    /// Find one named package in the selected package's resolved dependency closure.
    pub fn dependency(&self, name: &str) -> Result<&CargoPackage> {
        let mut pending = vec![self.package.id.as_str()];
        let mut visited = HashSet::new();
        let mut matches = Vec::new();
        while let Some(id) = pending.pop() {
            if !visited.insert(id) {
                continue;
            }
            if id != self.package.id
                && let Some(package) = self.packages.get(id)
                && package.name == name
            {
                matches.push(package);
            }
            pending.extend(
                self.dependencies
                    .get(id)
                    .into_iter()
                    .flatten()
                    .map(String::as_str),
            );
        }
        matches.sort_by(|left, right| left.version.cmp(&right.version));
        match matches.as_slice() {
            [package] => Ok(package),
            [] => Err(anyhow::anyhow!(
                "Cargo package `{}` does not depend on `{name}`",
                self.package.name
            )
            .into()),
            packages => Err(anyhow::anyhow!(
                "Cargo package `{}` resolves multiple `{name}` versions: {}",
                self.package.name,
                packages
                    .iter()
                    .map(|package| package.version.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into()),
        }
    }
}

fn canonical(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap_or_else(|_| path.to_owned())
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<CargoPackage>,
    workspace_root: PathBuf,
    target_directory: PathBuf,
    resolve: Option<Resolve>,
}

#[derive(Deserialize)]
struct Resolve {
    nodes: Vec<Node>,
}

#[derive(Deserialize)]
struct Node {
    id: String,
    dependencies: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(name: &str, kind: &str) -> CargoTarget {
        CargoTarget {
            name: name.into(),
            kind: vec![kind.into()],
            crate_types: vec!["bin".into()],
        }
    }

    fn project(targets: Vec<CargoTarget>, default_run: Option<&str>) -> CargoProject {
        let package = CargoPackage {
            id: "app 0.1.0 (path+file:///app)".into(),
            name: "app".into(),
            version: "0.1.0".into(),
            manifest_path: "/app/Cargo.toml".into(),
            metadata: serde_json::Value::Null,
            default_run: default_run.map(str::to_owned),
            targets,
        };
        CargoProject {
            workspace_manifest_path: "/Cargo.toml".into(),
            target_directory: "/target".into(),
            packages: HashMap::from([(package.id.clone(), package.clone())]),
            package,
            dependencies: HashMap::new(),
        }
    }

    #[test]
    fn selects_cargo_executables_without_guessing() {
        let single = project(vec![target("renamed", "bin")], None);
        assert_eq!(
            single.executable_target(None, None).unwrap(),
            CargoTargetSelection::Bin("renamed".into())
        );

        let multiple = project(
            vec![target("client", "bin"), target("server", "bin")],
            Some("client"),
        );
        assert_eq!(
            multiple.executable_target(None, None).unwrap(),
            CargoTargetSelection::Bin("client".into())
        );
        let error = multiple
            .executable_target(None, Some("demo"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("no executable example target `demo`"));
        assert!(error.contains("Available example targets: none"));
    }

    #[test]
    fn rejects_ambiguous_default_binary() {
        let project = project(vec![target("client", "bin"), target("server", "bin")], None);
        let error = project
            .executable_target(None, None)
            .unwrap_err()
            .to_string();
        assert!(error.contains("multiple binary targets: client, server"));
        assert!(error.contains("--bin"));
    }

    #[test]
    fn selects_the_manifest_and_resolves_its_dependency_closure() {
        let root = tempfile::tempdir().unwrap();
        let app = root.path().join("app");
        let dependency = root.path().join("miniquad");
        std::fs::create_dir_all(app.join("src")).unwrap();
        std::fs::create_dir_all(dependency.join("src")).unwrap();
        std::fs::write(
            root.path().join("Cargo.toml"),
            "[workspace]\nresolver = \"3\"\nmembers = [\"app\", \"miniquad\"]\n",
        )
        .unwrap();
        std::fs::write(
            app.join("Cargo.toml"),
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\
             [lib]\nname = \"mobile_app\"\ncrate-type = [\"cdylib\", \"rlib\"]\n\
             [features]\nmobile = [\"dep:miniquad\"]\n\
             [dependencies]\nminiquad = { path = \"../miniquad\", optional = true }\n",
        )
        .unwrap();
        std::fs::write(app.join("src/lib.rs"), "").unwrap();
        std::fs::write(
            dependency.join("Cargo.toml"),
            "[package]\nname = \"miniquad\"\nversion = \"1.2.3\"\nedition = \"2024\"\n",
        )
        .unwrap();
        std::fs::write(dependency.join("src/lib.rs"), "").unwrap();

        let loaded =
            LoadedProject::load_with_features(&app, &["mobile".into()], false, false).unwrap();
        assert_eq!(loaded.root, dunce::canonicalize(&app).unwrap());
        assert_eq!(loaded.manifest_path, loaded.root.join("Cargo.toml"));
        let project = loaded.cargo;
        assert_eq!(project.package.name, "app");
        assert_eq!(project.library_target().unwrap().name, "mobile_app");
        assert!(project.library_target().unwrap().is_cdylib());
        assert_eq!(project.dependency("miniquad").unwrap().version, "1.2.3");
    }

    #[test]
    fn rejects_ambiguous_dependency_versions() {
        let root = tempfile::tempdir().unwrap();
        let app = root.path().join("app");
        std::fs::create_dir_all(app.join("src")).unwrap();
        std::fs::write(
            root.path().join("Cargo.toml"),
            "[workspace]\nresolver = \"3\"\nmembers = [\"app\"]\n\
             exclude = [\"miniquad-1\", \"miniquad-2\"]\n",
        )
        .unwrap();
        std::fs::write(
            app.join("Cargo.toml"),
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\
             [dependencies]\nminiquad-1 = { package = \"miniquad\", path = \"../miniquad-1\" }\n\
             miniquad-2 = { package = \"miniquad\", path = \"../miniquad-2\" }\n",
        )
        .unwrap();
        std::fs::write(app.join("src/lib.rs"), "").unwrap();
        for (directory, version) in [("miniquad-1", "1.0.0"), ("miniquad-2", "2.0.0")] {
            let package = root.path().join(directory);
            std::fs::create_dir_all(package.join("src")).unwrap();
            std::fs::write(
                package.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"miniquad\"\nversion = \"{version}\"\nedition = \"2024\"\n"
                ),
            )
            .unwrap();
            std::fs::write(package.join("src/lib.rs"), "").unwrap();
        }

        let error = CargoProject::load(&app.join("Cargo.toml"))
            .unwrap()
            .dependency("miniquad")
            .unwrap_err()
            .to_string();
        assert!(error.contains("multiple `miniquad` versions"));
        assert!(error.contains("1.0.0, 2.0.0"));
    }
}
