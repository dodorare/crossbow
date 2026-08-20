use super::CargoPackage;
use crate::{error::*, types::*};
use anyhow::Context as _;
use serde::Deserialize;
use std::{
    io::{BufRead as _, Write as _},
    path::PathBuf,
    process::{Command, Stdio},
};

/// A Cargo build described entirely through Cargo's public command-line interface.
pub struct CargoBuild<'a> {
    pub package: &'a CargoPackage,
    pub target: &'a CargoTargetSelection,
    pub target_triple: &'a str,
    pub target_dir: &'a std::path::Path,
    pub profile: Profile,
    pub features: &'a [String],
    pub all_features: bool,
    pub no_default_features: bool,
}

#[derive(Debug)]
pub struct CargoArtifact {
    pub crate_types: Vec<String>,
    pub filenames: Vec<PathBuf>,
    pub executable: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct CargoArtifactTarget {
    name: String,
    kind: Vec<String>,
    crate_types: Vec<String>,
}

#[derive(Deserialize)]
struct CargoMessage {
    reason: String,
    package_id: Option<String>,
    target: Option<CargoArtifactTarget>,
    #[serde(default)]
    filenames: Vec<PathBuf>,
    executable: Option<PathBuf>,
    message: Option<CargoDiagnostic>,
}

#[derive(Deserialize)]
struct CargoDiagnostic {
    rendered: Option<String>,
}

impl CargoBuild<'_> {
    pub fn command(&self) -> Command {
        let mut command = Command::new("cargo");
        command
            .arg("build")
            .arg("--manifest-path")
            .arg(&self.package.manifest_path)
            .arg("--package")
            .arg(&self.package.name)
            .arg("--target")
            .arg(self.target_triple)
            .arg("--target-dir")
            .arg(self.target_dir)
            .arg("--profile")
            .arg(self.profile.cargo_name())
            .arg("--message-format=json-render-diagnostics");
        self.target.append_to(&mut command);
        if !self.features.is_empty() {
            command.arg("--features").arg(self.features.join(","));
        }
        if self.all_features {
            command.arg("--all-features");
        }
        if self.no_default_features {
            command.arg("--no-default-features");
        }
        if let Some(project_dir) = self.package.manifest_path.parent() {
            command.current_dir(project_dir);
        }
        command
    }

    pub fn run(self, configure: impl FnOnce(&mut Command)) -> Result<CargoArtifact> {
        let mut command = self.command();
        configure(&mut command);
        command.stdout(Stdio::piped()).stderr(Stdio::inherit());

        let mut child = command.spawn().context("failed to start Cargo")?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("failed to capture Cargo stdout"))?;
        let mut artifact = None;
        let mut read_error = None;

        for line in std::io::BufReader::new(stdout).lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    read_error = Some(error);
                    break;
                }
            };
            let Ok(message) = serde_json::from_str::<CargoMessage>(&line) else {
                eprintln!("{line}");
                continue;
            };
            if let Some(rendered) = message
                .message
                .as_ref()
                .and_then(|diagnostic| diagnostic.rendered.as_deref())
            {
                eprint!("{rendered}");
                std::io::stderr().flush().ok();
            }
            if let Some(reported) = message.artifact_for(self.package, self.target) {
                artifact = Some(reported);
            }
        }

        if read_error.is_some() {
            child.kill().ok();
        }
        let status = child.wait().context("failed to wait for Cargo")?;
        if let Some(error) = read_error {
            return Err(anyhow::Error::new(error)
                .context("failed to read Cargo output")
                .into());
        }
        if !status.success() {
            return Err(Error::CmdFailed(
                command,
                String::new(),
                format!("Cargo exited with {status}"),
            ));
        }
        artifact.ok_or_else(|| {
            anyhow::anyhow!(
                "Cargo did not report artifact `{}` for package `{}`",
                self.target.name(),
                self.package.name
            )
            .into()
        })
    }
}

impl CargoMessage {
    fn artifact_for(
        self,
        package: &CargoPackage,
        selection: &CargoTargetSelection,
    ) -> Option<CargoArtifact> {
        if self.reason != "compiler-artifact"
            || self.package_id.as_deref() != Some(package.id.as_str())
        {
            return None;
        }
        self.target
            .filter(|target| {
                target.name == selection.name() && selection.matches_kind(&target.kind)
            })
            .map(|target| CargoArtifact {
                crate_types: target.crate_types,
                filenames: self.filenames,
                executable: self.executable,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package() -> CargoPackage {
        CargoPackage {
            id: "path+file:///game#my-game@0.1.0".into(),
            name: "my-game".into(),
            version: "0.1.0".into(),
            manifest_path: PathBuf::from("/game/Cargo.toml"),
            metadata: serde_json::Value::Null,
            default_run: None,
            targets: Vec::new(),
        }
    }

    #[test]
    fn constructs_an_explicit_cargo_build() {
        let package = package();
        let target = CargoTargetSelection::Bin("mobile".into());
        let build = CargoBuild {
            package: &package,
            target: &target,
            target_triple: "aarch64-apple-ios",
            target_dir: std::path::Path::new("/tmp/output"),
            profile: Profile::Release,
            features: &["mobile".into(), "bevy/png".into()],
            all_features: false,
            no_default_features: true,
        };
        let args = build
            .command()
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            args,
            [
                "build",
                "--manifest-path",
                "/game/Cargo.toml",
                "--package",
                "my-game",
                "--target",
                "aarch64-apple-ios",
                "--target-dir",
                "/tmp/output",
                "--profile",
                "release",
                "--message-format=json-render-diagnostics",
                "--bin",
                "mobile",
                "--features",
                "mobile,bevy/png",
                "--no-default-features",
            ]
        );
    }

    #[test]
    fn matches_the_exact_package_and_target() {
        let package = package();
        let target = CargoTargetSelection::Bin("mobile".into());
        let message = |package_id: &str| {
            serde_json::from_value::<CargoMessage>(serde_json::json!({
                "reason": "compiler-artifact",
                "package_id": package_id,
                "target": {
                    "name": "mobile",
                    "kind": ["bin"],
                    "crate_types": ["bin"]
                },
                "filenames": ["/tmp/mobile"],
                "executable": "/tmp/mobile"
            }))
            .unwrap()
        };
        assert!(
            message(&package.id)
                .artifact_for(&package, &target)
                .is_some()
        );
        assert!(
            message("path+file:///dependency#my-game@0.1.0")
                .artifact_for(&package, &target)
                .is_none()
        );
    }
}
