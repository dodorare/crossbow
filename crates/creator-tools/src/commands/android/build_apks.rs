use crate::commands::android::gen_debug_key;
use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::Key;

pub fn build_apks(
    aab_path: &Path,
    output_apks: &Path,
    package_label: &str,
    key: Key,
) -> Result<PathBuf> {
    let apks = output_apks.join(format!("{}.apks", package_label));
    if !output_apks.exists() {
        std::fs::create_dir_all(&output_apks)?;
    }
    let alias = "androiddebugkey".to_string();
    let mut build_apks = Command::new("java");
    build_apks.arg("-jar");
    if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
        build_apks.arg(bundletool_path);
    } else {
        return Err(AndroidError::BundletoolNotFound.into());
    }
    build_apks
        .arg("build-apks")
        .arg("--bundle")
        .arg(aab_path)
        .arg("--output")
        .arg(output_apks)
        .arg("--overwrite")
        .arg("--ks")
        .arg(&key.path)
        .arg("--ks-pass=pass:android")
        .arg("--ks-key-alias")
        .arg(alias);
    build_apks.output_err(true)?;
    // BuildApks::new(&aab_path, &apks)
    //     .ks(&key.path)
    //     .ks_pass_pass(key.password)
    //     .ks_key_alias(alias)
    //     .run()?;
    Ok(apks)
}

// #[cfg(test)]

// mod tests {
//     use super::Key;
//     use super::*;

//     #[test]

//     fn test() {
//         build_apks(
//             &Path::new("res\\mipmap\\test.aab"),
//             &Path::new("res\\mipmap\\"),
//             "example",
//             Key {},
//         )
//         .unwrap();
//     }
// }
