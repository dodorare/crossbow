mod artifact;
mod manifest;
mod profile;
mod utils;

use crate::Error;
pub use artifact::{Artifact, CrateType};
pub use profile::Profile;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct Subcommand {
    cmd: String,
    args: Vec<String>,
    package: String,
    manifest: PathBuf,
    target_dir: PathBuf,
    target: Option<String>,
    host_triple: String,
    profile: Profile,
    artifacts: Vec<Artifact>,
    quiet: bool,
}

impl Subcommand {
    pub fn new<F: FnMut(&str, Option<&str>) -> Result<bool, Error>>(
        subcommand: &'static str,
        mut parser: F,
    ) -> Result<Self, Error> {
        let mut args = std::env::args().peekable();
        args.next().ok_or(Error::InvalidArgs)?;
        let arg = args.next().ok_or(Error::InvalidArgs)?;
        if arg != subcommand {
            return Err(Error::InvalidArgs);
        }
        let cmd = args.next().unwrap_or_else(|| "--help".to_string());
        let mut cargo_args = Vec::new();
        let mut target = None;
        let mut profile = Profile::Dev;
        let mut artifacts = Vec::new();
        let mut target_dir = None;
        let mut manifest_path = None;
        let mut package = None;
        let mut examples = false;
        let mut bins = false;
        let mut quiet = false;
        while let Some(name) = args.next() {
            let value = if let Some(value) = args.peek() {
                if !value.starts_with('-') {
                    args.next()
                } else {
                    None
                }
            } else {
                None
            };
            let value_ref = value.as_deref();
            let mut matched = true;
            match (name.as_str(), value_ref) {
                ("--quiet", None) => quiet = true,
                ("--release", None) => profile = Profile::Release,
                ("--target", Some(value)) => target = Some(value.to_string()),
                ("--profile", Some("dev")) => profile = Profile::Dev,
                ("--profile", Some("release")) => profile = Profile::Release,
                ("--profile", Some(value)) => profile = Profile::Custom(value.to_string()),
                ("--example", Some(value)) => artifacts.push(Artifact::Example(value.to_string())),
                ("--examples", None) => examples = true,
                ("--bin", Some(value)) => artifacts.push(Artifact::Root(value.to_string())),
                ("--bins", None) => bins = true,
                ("--package", Some(value)) | ("-p", Some(value)) => {
                    package = Some(value.to_string())
                }
                ("--target-dir", Some(value)) => target_dir = Some(PathBuf::from(value)),
                ("--manifest-path", Some(value)) => manifest_path = Some(PathBuf::from(value)),
                _ => matched = false,
            }
            if matched || !parser(name.as_str(), value_ref)? {
                cargo_args.push(name);
                if let Some(value) = value {
                    cargo_args.push(value);
                }
            }
        }
        let (manifest, package, lib_name) = utils::find_package(
            &manifest_path.unwrap_or_else(|| std::env::current_dir().unwrap()),
            package.as_deref(),
        )?;
        let root_dir = manifest.parent().unwrap();
        let target_dir = target_dir.unwrap_or_else(|| {
            utils::find_workspace(&manifest, &package)
                .unwrap()
                .unwrap_or_else(|| manifest.clone())
                .parent()
                .unwrap()
                .join("target")
        });
        if examples {
            for file in utils::list_rust_files(&root_dir.join("examples"))? {
                artifacts.push(Artifact::Example(file));
            }
        }
        if bins {
            for file in utils::list_rust_files(&root_dir.join("src").join("bin"))? {
                artifacts.push(Artifact::Root(file));
            }
        }
        if artifacts.is_empty() {
            artifacts.push(Artifact::Root(lib_name.unwrap_or(package.clone())));
        }
        let host_triple = Command::new("rustc")
            .arg("-vV")
            .output()
            .map_err(|_| Error::RustcNotFound)?
            .stdout
            .lines()
            .map(|l| l.unwrap())
            .find(|l| l.starts_with("host: "))
            .map(|l| l[6..].to_string())
            .ok_or(Error::RustcNotFound)?;
        Ok(Self {
            cmd,
            args: cargo_args,
            package,
            manifest,
            target_dir,
            target,
            host_triple,
            profile,
            artifacts,
            quiet,
        })
    }

    pub fn cmd(&self) -> &str {
        &self.cmd
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn manifest(&self) -> &Path {
        &self.manifest
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }

    pub fn target_dir(&self) -> &Path {
        &self.target_dir
    }

    pub fn host_triple(&self) -> &str {
        &self.host_triple
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }
}
