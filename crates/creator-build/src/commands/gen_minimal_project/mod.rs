mod consts;

use super::Command;
use crate::error::Result;
use consts::*;

use std::path::PathBuf;
use std::{
    fs::{create_dir, File},
    io::Write,
};

#[derive(Debug, Clone)]
pub struct GenMinimalProject {
    pub out_dir: PathBuf,
}

impl GenMinimalProject {
    pub fn new(out_dir: PathBuf) -> Self {
        Self { out_dir }
    }
}

impl Command for GenMinimalProject {
    type Deps = ();
    type Output = String;

    fn run(&self) -> Result<Self::Output> {
        // Create Cargo.toml file
        let file_path = self.out_dir.join("Cargo.toml");
        let mut file = File::create(file_path)?;
        file.write_all(CARGO_TOML_VALUE.as_bytes())?;
        // Create src folder
        let src_path = self.out_dir.join("src/");
        create_dir(src_path.clone())?;
        // Create lib.rs
        let lib_rs_path = src_path.join("lib.rs");
        let mut lib_rs = File::create(lib_rs_path)?;
        lib_rs.write_all(LIB_RS_VALUE.as_bytes())?;
        // Create main.rs
        let main_rs_path = src_path.join("main.rs");
        let mut main_rs = File::create(main_rs_path)?;
        main_rs.write_all(MAIN_RS_VALUE.as_bytes())?;
        Ok("example".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        let cmd = GenMinimalProject::new(dir.path().to_owned());
        cmd.run().unwrap();
    }
}
