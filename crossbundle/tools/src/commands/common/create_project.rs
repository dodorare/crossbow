use crate::error::*;
use std::path::Path;
use std::process::Command;

/// Creates a new project.
/// Runs `cargo generate ...` with given args.
pub fn create_project(
    current_dir: &Path,
    name: &str,
    git: &str,
    branch: &Option<String>,
) -> Result<()> {
    let mut cargo_generate = Command::new("cargo");
    cargo_generate
        .current_dir(current_dir)
        .arg("generate")
        .arg("--git")
        .arg(git)
        .arg("--name")
        .arg(name);
    if let Some(branch) = branch {
        cargo_generate.arg("--branch").arg(branch);
    };
    cargo_generate.output_err(true)?;
    Ok(())
}

/// Checks if `cargo-generate` is installed in the system.
pub fn check_cargo_generate() -> bool {
    Command::new("cargo")
        .arg("generate")
        .arg("-V")
        .output()
        .map(|s| s.status.success())
        .unwrap_or(false)
}
