use crate::error::*;
use crate::tools::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn aapt2_compile(
    sdk: &AndroidSdk,
    project_path: &Path,
    input: &Path,
    output_directory: &Path,
    build_dir: &Path,
    package_label: String,
) -> Result<PathBuf> {
    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
    }
    let apk_path = build_dir.join(format!("{}-unaligned.apk", package_label));
    let mut aapt2_compile = sdk.build_tool(bin!("aapt2"), Some(project_path))?;
    aapt2_compile
        .arg("compile")
        .arg(input)
        .arg("-o")
        .arg(output_directory);
    aapt2_compile.output_err(true)?;
    Ok(apk_path)
}

pub fn aapt2_link(
    sdk: &AndroidSdk,
    project_path: &Path,
    input: &Path,
    manifest_path: &Path,
    output_apk: &Path,
    flat_file: &Path,
    build_dir: &Path,
    project_src: &Path,
    assets: Option<PathBuf>,
    package_label: String,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let apk_path = build_dir.join(format!("{}-unaligned.apk", package_label));
    let mut aapt2_link = sdk.build_tool(bin!("aapt2"), Some(project_path))?;
    aapt2_link
        .arg("link")
        .arg(input)
        .arg("-o")
        .arg(output_apk)
        .arg("-I")
        .arg(sdk.android_jar(target_sdk_version)?)
        .arg("--manifest")
        .arg(manifest_path)
        .arg("-R")
        .arg(flat_file)
        .arg("--java")
        .arg(project_src)
        .arg("--auto-add-overlay");
        if let Some(assets) = &assets {
            aapt2_link.arg("--proto-format").arg(dunce::simplified(assets));
        }
        aapt2_link.output_err(true)?;
    Ok(apk_path)
}
