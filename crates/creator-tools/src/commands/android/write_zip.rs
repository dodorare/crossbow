use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip_extensions::write::ZipWriterExtensions;


pub fn write(source_path: &PathBuf, archive_file: &Path) -> zip::result::ZipResult<()> {
    let file = std::fs::File::create(archive_file)?;
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&source_path)?;
    Ok(())
}

pub fn dirs_to_write(source_path: &PathBuf) -> fs_extra::error::Result<()> {
    let path = source_path.join("AndroidManifest.xml");
    if path.exists() {
        let manifest_path = source_path.join("manifest");
        if !manifest_path.exists() {
            std::fs::create_dir_all(&manifest_path)?;
        }
        let options = fs_extra::file::CopyOptions::new();
        fs_extra::file::move_file(&path, &manifest_path.join("AndroidManifest.xml"), &options)?;
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
}
