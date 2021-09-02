use crate::error::Result;

use std::path::Path;
use std::{
    fs::{create_dir, create_dir_all, File},
    io::Write,
};

const CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
edition = "2018"
version = "0.1.0"

[lib]
name = "example"
crate-type = ["lib", "cdylib"]
path = "src/lib.rs"

[dependencies]
creator = { git = "https://github.com/creator-rs/creator" }
"#;

const LIB_RS_VALUE: &str = r#"#[creator::creator_main] pub fn main() {println!("hello")}"#;

const MAIN_RS_VALUE: &str = "fn main(){example::main();}";

const STRINGS_XNL_VALUE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="hello">Hello!</string>
</resources>
"#;

/// Generates a new minimal project in given path.
pub fn gen_minimal_project(out_dir: &Path) -> Result<String> {
    // Create Cargo.toml file
    let file_path = out_dir.join("Cargo.toml");
    let mut file = File::create(file_path)?;
    file.write_all(CARGO_TOML_VALUE.as_bytes())?;
    // Create src folder
    let src_path = out_dir.join("src");
    create_dir(src_path.clone())?;
    // Create lib.rs
    let lib_rs_path = src_path.join("lib.rs");
    let mut lib_rs = File::create(lib_rs_path)?;
    lib_rs.write_all(LIB_RS_VALUE.as_bytes())?;
    // Create main.rs
    let main_rs_path = src_path.join("main.rs");
    let mut main_rs = File::create(main_rs_path)?;
    main_rs.write_all(MAIN_RS_VALUE.as_bytes())?;
    // Create res/values folder
    let res_path = out_dir.join("res").join("values");
    create_dir_all(res_path.clone())?;
    // Create strings.xml
    let strings_xml_path = res_path.join("strings.xml");
    let mut strings_xml = File::create(strings_xml_path)?;
    strings_xml.write_all(STRINGS_XNL_VALUE.as_bytes())?;
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
