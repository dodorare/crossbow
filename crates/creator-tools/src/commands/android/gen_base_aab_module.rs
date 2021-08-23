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
    res_path: &Path,
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
    if res_path.is_dir() {
        for entry in read_dir(res_path)? {
            let entry = entry?;
            let path = entry.path();
            Aapt2Compile::new(&[path], &compiled_res).run()?;
        }
    }
    let apk_path = build_dir.join(format!("{}_module.apk", package_label));
    if compiled_res.is_dir() {
        for entry in read_dir(compiled_res)? {
            let entry = entry?;
            let path = entry.path();
            Aapt2Link::new(&[path], apk_path.clone(), manifest_path)
                .i(sdk.android_jar(target_sdk_version)?)
                .version_code(1)
                .proto_format(true)
                .auto_add_overlay(true)
                .run()?;
        }
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
