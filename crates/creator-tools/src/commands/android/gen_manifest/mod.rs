mod manifest;

pub use manifest::*;

use crate::commands::Command;
use crate::error::*;
use std::{fs::File, io::Write, path::PathBuf};

pub struct GenAndroidManifest {
    pub out_dir: PathBuf,
    pub manifest: AndroidManifest,
}

impl GenAndroidManifest {
    pub fn new(out_dir: PathBuf, manifest: AndroidManifest) -> Self {
        Self { out_dir, manifest }
    }
}

impl Command for GenAndroidManifest {
    type Deps = ();
    type Output = ();

    fn run(&self) -> Result<Self::Output> {
        let mut file = File::create(self.out_dir.join("AndroidManifest.xml"))?;
        writeln!(file, "{}", self.manifest.to_string())?;
        Ok(())
    }
}
