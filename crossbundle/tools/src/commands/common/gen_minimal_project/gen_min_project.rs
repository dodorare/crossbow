use super::*;
use crate::error::*;
use std::{
    fs::{create_dir, File},
    io::Write,
};

/// Generates a new minimal project in given path.
pub fn gen_minimal_project(out_dir: &std::path::Path, macroquad_project: bool) -> Result<String> {
    // Create Cargo.toml file
    let file_path = out_dir.join("Cargo.toml");
    let mut file = File::create(file_path)?;
    if macroquad_project {
        file.write_all(MQ_CARGO_TOML_VALUE.as_bytes())?;
    } else {
        file.write_all(BEVY_CARGO_TOML_VALUE.as_bytes())?;
    }
    // Create src folder
    let src_path = out_dir.join("src");
    create_dir(src_path.clone())?;
    // Create main.rs
    let main_rs_path = src_path.join("main.rs");
    let mut main_rs = File::create(main_rs_path)?;
    if macroquad_project {
        main_rs.write_all(MQ_MAIN_RS_VALUE.as_bytes())?;
    } else {
        main_rs.write_all(BEVY_MAIN_RS_VALUE.as_bytes())?;
    }
    create_res_folder(out_dir)?;
    Ok("example".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        gen_minimal_project(dir.path(), true).unwrap();
    }
}
