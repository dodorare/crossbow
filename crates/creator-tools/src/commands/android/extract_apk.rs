use std::{fs, io, path::Path};

pub fn extract_apk(apk_path: &Path) {
    let filename = Path::new(apk_path);
    let file = fs::File::open(&filename).unwrap();

    let mut apk = zip::ZipArchive::new(file).unwrap();

    for i in 0..apk.len() {
        let mut file = apk.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let _extract_apk = extract_apk(&Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\mipmap\\test.apk"));
    }
}
