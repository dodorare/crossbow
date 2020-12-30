use crate::error::Result;

use std::path::Path;
use std::{
    fs::{create_dir, File},
    io::Write,
};

pub const CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
edition = "2018"
version = "0.1.0"

[lib]
crate-type = ["lib", "cdylib"]

[dependencies]
creator = { git = "https://github.com/creator-rs/creator" }
"#;

pub const LIB_RS_VALUE: &str = "#[creator::creator_main] pub fn main() {}";

pub const MAIN_RS_VALUE: &str = "fn main(){example::main();}";

pub fn gen_minimal_project(out_dir: &Path) -> Result<String> {
    // Create Cargo.toml file
    let file_path = out_dir.join("Cargo.toml");
    let mut file = File::create(file_path)?;
    file.write_all(CARGO_TOML_VALUE.as_bytes())?;
    // Create src folder
    let src_path = out_dir.join("src/");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        gen_minimal_project(dir.path()).unwrap();
    }
}
