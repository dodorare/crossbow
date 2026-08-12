use crate::error::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Allows to extract files from generated APK to use that to generate `.aab`
pub fn extract_archive(archive_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    let file = fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(output_dir)?;
    Ok(output_dir.to_owned())
}

#[cfg(test)]
mod tests {
    use super::extract_archive;
    use std::io::Write;

    #[test]
    fn extracts_deflated_android_archive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let archive_path = temp_dir.path().join("module.apk");
        let file = std::fs::File::create(&archive_path).unwrap();
        let mut archive = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        archive.start_file("AndroidManifest.xml", options).unwrap();
        archive.write_all(b"manifest").unwrap();
        archive.finish().unwrap();

        let output_dir = temp_dir.path().join("extracted");
        extract_archive(&archive_path, &output_dir).unwrap();

        assert_eq!(
            std::fs::read(output_dir.join("AndroidManifest.xml")).unwrap(),
            b"manifest"
        );
    }
}
