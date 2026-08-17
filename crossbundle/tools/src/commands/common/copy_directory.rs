use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy)]
pub(crate) enum ExistingFile {
    Overwrite,
    #[cfg_attr(not(feature = "apple"), allow(dead_code))]
    Skip,
}

pub(crate) fn copy_directory_contents(
    source: &Path,
    destination: &Path,
    existing_file: ExistingFile,
) -> io::Result<()> {
    fn copy(
        source: &Path,
        destination: &Path,
        existing_file: ExistingFile,
        ancestors: &mut Vec<PathBuf>,
    ) -> io::Result<()> {
        let canonical_source = fs::canonicalize(source)?;
        if ancestors.contains(&canonical_source) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("directory symlink cycle at {}", source.display()),
            ));
        }
        let entries = fs::read_dir(source)?;
        ancestors.push(canonical_source);

        let result = (|| {
            fs::create_dir_all(destination)?;
            for entry in entries {
                let entry = entry?;
                let source = entry.path();
                let destination = destination.join(entry.file_name());
                let metadata = fs::metadata(&source)?;

                if metadata.is_dir() {
                    copy(&source, &destination, existing_file, ancestors)?;
                } else if !metadata.is_file() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("source is not a file or directory: {}", source.display()),
                    ));
                } else if !destination.exists() || matches!(existing_file, ExistingFile::Overwrite)
                {
                    fs::copy(source, destination)?;
                }
            }
            Ok(())
        })();

        ancestors.pop();
        result
    }

    copy(source, destination, existing_file, &mut Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_nested_contents_and_empty_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let destination = temp_dir.path().join("destination");
        fs::create_dir_all(source.join("nested/empty")).unwrap();
        fs::write(source.join("nested/value.txt"), "new").unwrap();
        fs::create_dir_all(destination.join("nested")).unwrap();
        fs::write(destination.join("nested/value.txt"), "old").unwrap();

        copy_directory_contents(&source, &destination, ExistingFile::Overwrite).unwrap();

        assert_eq!(
            fs::read_to_string(destination.join("nested/value.txt")).unwrap(),
            "new"
        );
        assert!(destination.join("nested/empty").is_dir());
    }

    #[test]
    fn can_skip_existing_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let destination = temp_dir.path().join("destination");
        fs::create_dir_all(&source).unwrap();
        fs::create_dir_all(&destination).unwrap();
        fs::write(source.join("existing.txt"), "new").unwrap();
        fs::write(source.join("additional.txt"), "additional").unwrap();
        fs::write(destination.join("existing.txt"), "old").unwrap();

        copy_directory_contents(&source, &destination, ExistingFile::Skip).unwrap();

        assert_eq!(
            fs::read_to_string(destination.join("existing.txt")).unwrap(),
            "old"
        );
        assert_eq!(
            fs::read_to_string(destination.join("additional.txt")).unwrap(),
            "additional"
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_directory_symlink_cycles() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let destination = temp_dir.path().join("destination");
        fs::create_dir_all(source.join("nested")).unwrap();
        std::os::unix::fs::symlink(&source, source.join("nested/back")).unwrap();

        let error =
            copy_directory_contents(&source, &destination, ExistingFile::Overwrite).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains("symlink cycle"));
    }
}
