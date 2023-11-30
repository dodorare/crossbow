use std::path::Path;
use zip::ZipWriter;
use zip::write::FileOptions;
use zip_extensions::write::ZipWriterExtensions;

/// Writing files into archive
pub fn zip_write(source_path: &Path, archive_file: &Path) -> zip::result::ZipResult<()> {
    let file = std::fs::File::create(archive_file)?;
    let mut zip = ZipWriter::new(file);
    zip.add_directory(&source_path.to_path_buf(),FileOptions::default())?;
    Ok(())
}

/// Moving AndroidManifest.xml file into directory to write files to archive
pub fn zip_dirs_to_write(source_path: &Path) -> fs_extra::error::Result<()> {
    let path = source_path.join("AndroidManifest.xml");
    if path.exists() {
        let manifest_path = source_path.join("manifest");
        if !manifest_path.exists() {
            std::fs::create_dir_all(&manifest_path)?;
        }
        let mut options = fs_extra::file::CopyOptions::new();
        options.overwrite = true;
        fs_extra::file::move_file(&path, &manifest_path.join("AndroidManifest.xml"), &options)?;
    }
    Ok(())
}
