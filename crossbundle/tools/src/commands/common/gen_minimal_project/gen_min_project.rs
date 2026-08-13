use super::*;
use crate::error::*;
use std::{
    fs::{File, create_dir},
    io::Write,
};

// TODO: Fix this file logic.

/// Generates a new minimal project in given path.
pub fn gen_minimal_project(out_dir: &std::path::Path, macroquad_project: bool) -> Result<String> {
    // Create Cargo.toml file
    let file_path = out_dir.join("Cargo.toml");
    let mut file = File::create(file_path)?;
    let cargo_toml = if macroquad_project {
        MINIMAL_MQ_CARGO_TOML_VALUE
    } else {
        MINIMAL_BEVY_CARGO_TOML_VALUE
    };
    let cargo_toml = use_test_workspace(cargo_toml)?;
    file.write_all(cargo_toml.as_bytes())?;
    // Create src folder
    let src_path = out_dir.join("src");
    create_dir(&src_path)?;
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

fn use_test_workspace(manifest: &str) -> Result<String> {
    let workspace_root = std::env::var_os("CROSSBOW_TEST_WORKSPACE");
    render_manifest(
        manifest,
        workspace_root.as_deref().map(std::path::Path::new),
    )
}

fn render_manifest(manifest: &str, workspace_root: Option<&std::path::Path>) -> Result<String> {
    let Some(workspace_root) = workspace_root else {
        return Ok(manifest.to_owned());
    };
    let workspace_root = dunce::canonicalize(workspace_root)?
        .to_string_lossy()
        .replace('\\', "/");
    Ok(manifest.replace(
        "crossbow = { git = \"https://github.com/dodorare/crossbow\" }",
        &format!("crossbow = {{ path = \"{}\" }}", workspace_root),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        gen_minimal_project(dir.path(), true).unwrap();
        assert!(dir.path().join("Cargo.toml").is_file());
    }

    #[test]
    fn default_manifest_uses_public_repository() {
        let manifest = render_manifest(MINIMAL_MQ_CARGO_TOML_VALUE, None).unwrap();
        assert!(manifest.contains("github.com/dodorare/crossbow"));
    }

    #[test]
    fn test_workspace_replaces_remote_dependency() {
        let workspace = tempfile::tempdir().unwrap();
        let manifest =
            render_manifest(MINIMAL_MQ_CARGO_TOML_VALUE, Some(workspace.path())).unwrap();
        assert!(manifest.contains("crossbow = { path ="));
        assert!(!manifest.contains("github.com/dodorare/crossbow"));
    }
}
