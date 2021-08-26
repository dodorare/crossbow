use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use crate::{
    commands::{android, gen_minimal_project},
    tools::AndroidSdk,
    types::*,
};

use std::fs::read_dir;
use std::path::{Path, PathBuf};

/// Compile resources, link resources and extract apk
pub fn gen_base_aab_module(
    res_path: Option<PathBuf>,
    assets_path: Option<PathBuf>,
    build_dir: &Path,
    sdk: &AndroidSdk,
    package_label: &str,
    manifest_path: &Path,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let compiled_res = build_dir.join("compiled_res");
    if !compiled_res.exists() {
        std::fs::create_dir_all(&compiled_res)?;
    }
    if let Some(res) = res_path {
        let resources = res.as_path();
        // TODO: handle errors, return err if path not found
        let paths = read_dir(resources)?
            .map(|e| e.map(|x| x.path()))
            .flatten()
            .collect::<Vec<_>>();
        Aapt2Compile::new(&paths, &compiled_res).run()?;
    }

    let apk_path = build_dir.join(format!("{}_module.apk", package_label));
    if compiled_res.is_dir() {
        // TODO: handle errors, return err if path not found
        let paths = read_dir(compiled_res)?
            .map(|e| e.map(|x| x.path()))
            .flatten()
            .collect::<Vec<_>>();
        let mut aapt2_link = Aapt2Link::new(&paths, apk_path.clone(), manifest_path);
        aapt2_link
            .i(sdk.android_jar(target_sdk_version)?)
            .version_code(1)
            .proto_format(true)
            .auto_add_overlay(true);
        if let Some(assets) = assets_path {
            aapt2_link.assets(assets);
        }
        aapt2_link.run()?;
    }
    let extracted_apk_files = build_dir.join("extracted_apk_files");
    extract_apk::extract_apk(&apk_path, &extracted_apk_files).unwrap();
    Ok(extracted_apk_files)
}

pub fn gen_zip_modules(
    build_dir: &Path,
    package_label: &str,
    extracted_apk_files: &PathBuf,
) -> Result<PathBuf> {
    let zip_path = build_dir.join(format!("{}_module.zip", package_label));
    write_zip::dirs_to_write(&extracted_apk_files.to_owned())?;
    write_zip::write(&extracted_apk_files.to_owned(), &zip_path).unwrap();
    Ok(zip_path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let sdk = AndroidSdk::from_env().unwrap();
        gen_base_aab_module(
            Some(Path::new("res\\mipmap").to_owned()),
            None,
            Path::new("res\\"),
            &sdk,
            "example",
            Path::new("manifest\\AndroidManifest.xml"),
            30,
        )
        .unwrap();
    }
}
