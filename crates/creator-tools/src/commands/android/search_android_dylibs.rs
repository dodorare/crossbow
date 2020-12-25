use crate::commands::Command;
use crate::error::*;
use crate::types::*;
use std::path::PathBuf;

pub struct SearchAndroidDylibs {
    target_path: PathBuf,
    build_target: BuildTarget,
    profile: Profile,
}

impl SearchAndroidDylibs {
    pub fn new(target_path: PathBuf, build_target: BuildTarget, profile: Profile) -> Self {
        Self {
            target_path,
            build_target,
            profile,
        }
    }
}

impl Command for SearchAndroidDylibs {
    type Deps = ();
    type Output = Vec<PathBuf>;

    fn run(&self) -> Result<Self::Output> {
        let mut paths = Vec::new();
        let deps_dir = self
            .target_path
            .join(self.build_target.rust_triple())
            .join(self.profile.as_ref())
            .join("build");
        for dep_dir in deps_dir.read_dir()? {
            let output_file = dep_dir?.path().join("output");
            if output_file.is_file() {
                use std::{
                    fs::File,
                    io::{BufRead, BufReader},
                };
                for line in BufReader::new(File::open(output_file)?).lines() {
                    let line = line?;
                    if let Some(link_search) = line.strip_prefix("cargo:rustc-link-search=") {
                        let mut pie = link_search.split('=');
                        let (kind, path) = match (pie.next(), pie.next()) {
                            (Some(kind), Some(path)) => (kind, path),
                            (Some(path), None) => ("all", path),
                            _ => unreachable!(),
                        };
                        match kind {
                            // FIXME: which kinds of search path we interested in
                            "dependency" | "native" | "all" => paths.push(path.into()),
                            _ => (),
                        };
                    }
                }
            }
        }
        Ok(paths)
    }
}
