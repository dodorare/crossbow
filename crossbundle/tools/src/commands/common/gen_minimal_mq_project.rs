use crate::error::*;

use std::path::Path;
use std::{
    fs::{create_dir, create_dir_all, File},
    io::Write,
};

const CARGO_TOML_VALUE: &str = r#"
[package]
name = "macroquad-3d"
authors = ["DodoRare Team <support@dodorare.com>"]
version = "0.1.2"
edition = "2021"

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }
anyhow = "1.0"
macroquad = "0.3"
"#;

const MAIN_RS_VALUE: &str = r#"
#[macroquad::main("Macroquad 3D")]
async fn main() -> anyhow::Result<()> {Ok(())}
"#;

const STRINGS_XML_VALUE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="hello">Hello!</string>
</resources>
"#;

/// Generates a new minimal project in given path.
pub fn gen_minimal_mq_project(out_dir: &Path) -> Result<String> {
    // Create Cargo.toml file
    let file_path = out_dir.join("Cargo.toml");
    let mut file = File::create(file_path)?;
    file.write_all(CARGO_TOML_VALUE.as_bytes())?;
    // Create src folder
    let src_path = out_dir.join("src");
    create_dir(src_path.clone())?;
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
    strings_xml.write_all(STRINGS_XML_VALUE.as_bytes())?;
    Ok("example".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        gen_minimal_mq_project(dir.path()).unwrap();
    }
}
