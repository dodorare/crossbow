use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip_extensions::write::ZipWriterExtensions;

pub fn write(source_path: &PathBuf, archive_file: &Path) -> zip::result::ZipResult<()> {
    let file = std::fs::File::create(archive_file)?;
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&source_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let _write_zip = write(
            &Path::new("res\\mipmap\\").to_owned(),
            Path::new("res\\mipmap\\doit.zip"),
        );
    }
}
