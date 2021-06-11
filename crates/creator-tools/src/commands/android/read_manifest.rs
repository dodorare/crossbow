use crate::error::{AndroidError, Result};
use android_manifest::AndroidManifest;
use std::{fs::File, io::BufReader, path::Path};

/// Read file and deserializes `AndroidManifest.xml` into [`AndroidManifest`](android_manifest::AndroidManifest).
pub fn read_android_manifest(path: &Path) -> Result<AndroidManifest> {
    let file = File::open(&path).map_err(|_| {
        AndroidError::FailedToFindAndroidManifest(path.to_string_lossy().to_string())
    })?;
    let reader = BufReader::new(&file);
    Ok(android_manifest::from_reader(reader).map_err(AndroidError::from)?)
}
