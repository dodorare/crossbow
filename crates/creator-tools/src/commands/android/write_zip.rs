use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip_extensions::write::ZipWriterExtensions;

pub fn write(source_path: &PathBuf, archive_file: &Path) -> zip::result::ZipResult<()> {
    let file = std::fs::File::create(archive_file)?;
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&source_path)?;
    Ok(())
}

pub fn dirs_to_write(source_path: &PathBuf) -> std::io::Result<()> {
    for entry in std::fs::read_dir(source_path)? {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename.ends_with(".xml") {
                let manifest_path = source_path.join("manifest");
                let options = fs_extra::dir::CopyOptions::new();
                let mut from_paths = Vec::new();
                let file_path = entry.path();
                from_paths.push(&file_path);
                if !manifest_path.exists() {
                    std::fs::create_dir_all(&manifest_path)?;
                }
                fs_extra::move_items(&from_paths, &manifest_path.as_path(), &options).unwrap();
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let _write_zip = write(
            &Path::new("res\\extracted_files\\").to_owned(),
            Path::new("res\\mipmap\\doit.zip"),
        );
    }

    #[test]
    fn test_two() {
        let _get_libs_in_dir = dirs_to_write(
            &Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\extracted_files\\").to_owned(),
             
        );
    }

    #[test]
    fn test_three() -> std::io::Result<()> {
        let options = fs_extra::dir::CopyOptions::new();
        let mut from_paths = Vec::new();
        from_paths.push("res\\mipmap\\AndroidManifest.xml");
        fs_extra::move_items(&from_paths, "res\\mipmap\\manifest\\", &options).unwrap();
        Ok(())
    }
}
