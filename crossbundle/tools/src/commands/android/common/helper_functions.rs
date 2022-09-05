use crate::error::*;

/// Helper function to delete files
pub fn remove(target: Vec<std::path::PathBuf>) -> Result<()> {
    target.iter().for_each(|content| {
        if content.exists() && content.is_file() {
            std::fs::remove_file(content).unwrap();
        }
        if content.exists() && content.is_dir() {
            std::fs::remove_dir_all(content).unwrap();
        }
    });
    Ok(())
}
