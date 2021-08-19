use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use crate::{
    commands::{android, gen_minimal_project},
    tools::AndroidSdk,
    types::*,
};
use std::path::{Path, PathBuf};

pub fn gen_base_aab_module(
    res_path: &[PathBuf],
    assets_path: Option<PathBuf>,
    build_dir: &Path,
    sdk: &AndroidSdk,
    package_label: &str,
    manifest_path: &Path,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let compiled_res = build_dir.join("compiled_res");
    Aapt2Compile::new(res_path, &compiled_res).run()?;

    let apk_path = build_dir.join(format!("{}_module.apk", package_label));
    let link = Aapt2Link::new(&[compiled_res], &apk_path, manifest_path)
        .i(sdk.android_jar(target_sdk_version)?)
        .version_code(1)
        .proto_format(true)
        .auto_add_overlay(true)
        .run()?;
    if let Some(assets_path) = assets_path {
        todo!()
    }

    let extracted_apk_files = build_dir.join("extracted_apk_files");
    extract_apk::extract_apk(&apk_path, &extracted_apk_files).unwrap();

    let zip_path = build_dir.join(format!("{}_module.zip", package_label));
    write_zip::dirs_to_write(&extracted_apk_files.to_owned())?;
    write_zip::write(&extracted_apk_files.to_owned(), &zip_path).unwrap();
    Ok(zip_path.to_path_buf())
}
