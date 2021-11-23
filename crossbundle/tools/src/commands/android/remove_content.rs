use crate::error::*;
use std::path::PathBuf;

pub fn remove_content(android_build_dir: &PathBuf, target: Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(&android_build_dir)? {
        let path = entry?.path();
        target.iter().for_each(|content| {
            if path.is_file() && path.ends_with(content) {
                std::fs::remove_file(&path).unwrap();
            }
            if path.is_dir() && path.ends_with(content) {
                std::fs::remove_dir_all(&path).unwrap();
            }
        });
    }

    Ok(())
}
