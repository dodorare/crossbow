use crate::error::{AndroidError, Result};
use std::process::Command;

/// Find gradle executable file in and initialize it
pub fn gradle_init() -> Result<Command> {
    if let Ok(gradle) = which::which(bat!("gradle")) {
        return Ok(Command::new(gradle));
    }
    let gradle = std::env::var("GRADLE_HOME").ok();
    let gradle_path = std::path::PathBuf::from(gradle.ok_or(AndroidError::GradleNotFound)?);
    let gradle_executable = gradle_path.join("bin").join(bat!("gradle"));
    Ok(Command::new(gradle_executable))
}

#[cfg(test)]
mod tests {

    use android_tools::error::CommandExt;

    use super::gradle_init;
    #[test]
    fn test_gradle_exe() {
        let mut gradle = gradle_init().unwrap();
        gradle.arg("-h");
        gradle.output_err(true).unwrap();
    }
}
