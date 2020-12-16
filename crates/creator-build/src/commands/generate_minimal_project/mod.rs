mod consts;

use super::Command;
use crate::error::StdResult;
use consts::*;

use std::path::PathBuf;
use std::{
    fs::{create_dir, File},
    io::Write,
};

pub struct GenerateMinimalProject {
    pub out_dir: PathBuf,
}

impl Command for GenerateMinimalProject {
    type Deps = ();
    type Output = ();

    fn run(&self, (): Self::Deps) -> StdResult<Self::Output> {
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::StdResult;

    #[test]
    fn test_command_run() -> StdResult<()> {
        let dir = tempfile::tempdir()?;
        let cmd = GenerateMinimalProject {
            out_dir: dir.path().to_owned(),
        };
        cmd.run(())?;
        Ok(())
    }
}
