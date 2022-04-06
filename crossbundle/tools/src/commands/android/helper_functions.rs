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

/// Helper function to parse the file to &[u8] from assets folder.
pub fn parse_the_file_to_slice_u8(
    file_name: &str,
    additional_dir: Option<String>,
    app_name: &str,
) -> Result<()> {
    let current_dir = std::env::current_dir()?
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let mut buf = std::path::PathBuf::from(current_dir);
    buf.push("examples");
    buf.push(app_name);
    buf.push("assets");
    if let Some(add) = additional_dir {
        buf.push(add);
    }
    buf.push(file_name);
    let bytes = std::fs::read(buf)?;
    bytes.as_slice();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_parse_the_file_to_slice_u8() {
        let file_name = "ferris.png";
        let app_name = "macroquad-3d";
        let additional_dir = Some(String::from("bob"));
        super::parse_the_file_to_slice_u8(file_name, additional_dir, app_name).unwrap();
    }
}
