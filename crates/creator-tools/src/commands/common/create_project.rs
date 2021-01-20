use crate::error::*;
use std::path::PathBuf;
use std::process::Command;

pub fn create_project(
    current_dir: PathBuf,
    git: &str,
    project_name: String,
    template: Option<String>,
) -> Result<()> {
    let mut cargo_generate = Command::new("cargo");
    cargo_generate
        .current_dir(current_dir)
        .arg("generate")
        .arg("--git")
        .arg(git)
        .arg("--name")
        .arg(project_name);
    if let Some(template) = template {
        cargo_generate.arg("--branch").arg(template);
    };
    cargo_generate.output_err(true)?;
    Ok(())
}

pub fn check_cargo_generate() -> bool {
    Command::new("cargo")
        .arg("generate")
        .arg("-V")
        .output()
        .map(|s| s.status.success())
        .unwrap_or(false)
}
