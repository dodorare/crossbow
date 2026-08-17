use std::{
    fs::File,
    io,
    path::Path,
};
use zip::{ZipWriter, write::SimpleFileOptions};

/// Writing files into archive
pub fn zip_write(source_path: &Path, archive_file: &Path) -> zip::result::ZipResult<()> {
    let file = File::create(archive_file)?;
    let mut zip = ZipWriter::new(file);
    let mut directories = vec![source_path.to_path_buf()];

    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(source_path).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("archive entry is outside the source directory: {error}"),
                )
            })?;
            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                zip.add_directory_from_path(relative_path, SimpleFileOptions::default())?;
                directories.push(path);
            } else if metadata.is_file() {
                zip.start_file_from_path(relative_path, SimpleFileOptions::default())?;
                let mut source = File::open(path)?;
                io::copy(&mut source, &mut zip)?;
            }
        }
    }

    zip.finish()?;
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
        fs_extra::file::move_file(&path, manifest_path.join("AndroidManifest.xml"), &options)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn zip_write_archives_directory_contents() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source_dir = temp_dir.path().join("module");
        let manifest_dir = source_dir.join("manifest");
        let library_dir = source_dir.join("lib").join("arm64-v8a");
        let empty_dir = source_dir.join("assets").join("empty");
        std::fs::create_dir_all(&manifest_dir).unwrap();
        std::fs::create_dir_all(&library_dir).unwrap();
        std::fs::create_dir_all(&empty_dir).unwrap();
        std::fs::write(
            manifest_dir.join("AndroidManifest.xml"),
            b"<manifest package=\"com.crossbow.test\" />",
        )
        .unwrap();
        std::fs::write(library_dir.join("libcrossbow.so"), b"native-library").unwrap();

        let archive_path = temp_dir.path().join("module.zip");
        zip_write(&source_dir, &archive_path).unwrap();

        let archive_file = std::fs::File::open(archive_path).unwrap();
        let mut archive = zip::ZipArchive::new(archive_file).unwrap();

        let mut manifest = String::new();
        archive
            .by_name("manifest/AndroidManifest.xml")
            .unwrap()
            .read_to_string(&mut manifest)
            .unwrap();
        assert_eq!(manifest, "<manifest package=\"com.crossbow.test\" />");

        let mut library = Vec::new();
        archive
            .by_name("lib/arm64-v8a/libcrossbow.so")
            .unwrap()
            .read_to_end(&mut library)
            .unwrap();
        assert_eq!(library, b"native-library");

        assert!(archive.by_name("assets/empty/").unwrap().is_dir());
    }
}
