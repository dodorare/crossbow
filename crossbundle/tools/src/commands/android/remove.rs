use crate::error::*;
use std::path::PathBuf;

/// Function to delete files
pub fn remove(target: Vec<PathBuf>) -> Result<()> {
    target.iter().for_each(|content| {
        if content.is_file() {
            std::fs::remove_file(&content).unwrap();
        }
        if content.is_dir() {
            std::fs::remove_dir_all(&content).unwrap();
        }
    });
    Ok(())
}
