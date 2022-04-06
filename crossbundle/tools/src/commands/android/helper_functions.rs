use crate::error::*;

/// Helper function to delete files
pub fn remove(target: Vec<std::path::PathBuf>) -> Result<()> {
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

/// Helper function to parse the file to slice u8 from assets folder.
pub fn parse_the_file_to_slice_u8(
    file_name: &str,
    app_name: &str,
    additional_dir: Option<String>,
) -> Result<Vec<u8>> {
    let current_dir = std::env::current_dir()?;
    let mut buf = current_dir;
    buf.push("examples");
    buf.push(app_name);
    buf.push("assets");
    if let Some(add) = additional_dir {
        buf.push(add);
    }
    buf.push(file_name);
    let bytes = std::fs::read(buf)?;
    Ok(bytes)
}
