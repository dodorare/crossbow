use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

pub fn gen_aab(
    inputs_compile: &[PathBuf],
    o_compile: &Path,
    sdk: &AndroidSdk,
    inputs_link: &[PathBuf],
    o_link: &Path,
    manifest: &Path,
    target_sdk_version: u32,
    modules: &[PathBuf],
    save_aab: &Path,
    extracted_apk: &Path,
) -> Result<()> {
    Aapt2Compile::new(inputs_compile, o_compile).run()?;

    Aapt2Link::new(inputs_link, o_link, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .proto_format(true)
        .auto_add_overlay(true)
        .run()?;

    extract_apk::extract_apk(o_link, extracted_apk).unwrap();

    // write_zip::write(
    //     &Path::new("res\\mipmap\\").to_owned(),
    //     Path::new("res\\mipmap\\base.zip"),
    // )
    // .unwrap();

    BuildBundle::new(modules, save_aab);
    Ok(())
}
