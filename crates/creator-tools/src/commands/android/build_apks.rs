use crate::commands::android::gen_debug_key;
use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

pub fn build_apks(aab_path: &Path, output_apks: &Path, package_label: &str) -> Result<PathBuf> {
    let apks = output_apks.join(format!("{}.apks", package_label));
    if !output_apks.exists() {
        std::fs::create_dir_all(&output_apks)?;
    }
    let key = gen_debug_key().unwrap();
    let alias = "androiddebugkey".to_string();
    BuildApks::new(&aab_path, &apks)
        .ks(&key.path)
        .ks_key_alias(alias)
        .run()?;
    Ok(apks)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test() {
        build_apks(
            &Path::new("res\\mipmap\\test.aab"),
            &Path::new("res\\mipmap\\"),
            "example",
        )
        .unwrap();
    }
}
