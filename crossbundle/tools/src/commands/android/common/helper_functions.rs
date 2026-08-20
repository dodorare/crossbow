use crate::error::*;

/// Helper function to delete files
pub fn remove(target: Vec<std::path::PathBuf>) -> Result<()> {
    for content in target {
        if content.is_file() {
            std::fs::remove_file(content)?;
        } else if content.is_dir() {
            std::fs::remove_dir_all(content)?;
        }
    }
    Ok(())
}
