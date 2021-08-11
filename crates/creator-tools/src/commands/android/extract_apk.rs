use std::{fs, path::Path};

pub fn extract_apk(apk_path: &Path, extracted_apk: &Path) -> zip::result::ZipResult<()> {
    let filename = Path::new(apk_path);
    let file = fs::File::open(&filename)?;
    let mut apk = zip::ZipArchive::new(file)?;
    apk.extract(extracted_apk)?;
    Ok(())
}
