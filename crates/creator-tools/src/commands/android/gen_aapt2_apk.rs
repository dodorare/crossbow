use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use crate::{
    commands::{android, gen_minimal_project},
    tools::AndroidSdk,
    types::*,
};

use std::path::{Path, PathBuf};

pub fn gen_aapt2_apk(
    sdk: &AndroidSdk,
    res_path: &[PathBuf],
    compiled_res:PathBuf,
    res_dir: Option<PathBuf>,
    res_zip: Option<PathBuf>,
    legacy: bool,
    imputs: &[PathBuf],
    apk_path: PathBuf,
    manifest_path: &Path,
    assets_path: Option<PathBuf>,
    min_sdk_version: Option<u32>,
    target_sdk_version: u32,
    individual_flat: Option<PathBuf>,
    package_id: Option<String>,
    allow_reserved_package_id: bool,
    proguard_options: Option<PathBuf>,
    no_auto_version: bool,
    no_version_vectors: bool,
    no_version_transitions: bool,
    enable_sparse_encoding: bool,
) -> Result<()>{
    let mut aapt2_compile =  Aapt2Compile::new(&res_path, &compiled_res);
    if let Some(res_dir) = res_dir {
        aapt2_compile.res_dir(&res_dir);
    }
    if let Some(res_zip) = res_zip {
        aapt2_compile.res_zip(&res_zip);
    }
    if legacy {
        aapt2_compile.legacy(legacy);
    }
    aapt2_compile.run()?;

    let mut aapt2_link = Aapt2Link::new(&imputs, apk_path, manifest_path);
    aapt2_link
        .android_jar(sdk.android_jar(target_sdk_version)?)
        .version_code(1)
        .proto_format(true)
        .auto_add_overlay(true);
    if let Some(assets) = assets_path {
        aapt2_link.assets(assets);
    }
    if let Some(package_id) = package_id {
        aapt2_link.package_id(package_id);
    }
    if no_auto_version {
        aapt2_link.no_auto_version(no_auto_version);
    }
    if  no_version_vectors {
        aapt2_link.no_version_vectors(no_version_vectors);
    }
    if  no_version_transitions {
        aapt2_link.no_version_transitions(no_version_transitions);
    }
    if  enable_sparse_encoding {
        aapt2_link.enable_sparse_encoding(enable_sparse_encoding);
    }
    if let Some(individual_flat) = individual_flat {
        aapt2_link.individual_flat(individual_flat);
    }
    if let Some(proguard_options) = proguard_options {
        aapt2_link.proguard_options(proguard_options);
    }
    if allow_reserved_package_id {
        aapt2_link.allow_reserved_package_id(allow_reserved_package_id);
    }
    if let Some(min_sdk_version) = min_sdk_version {
        aapt2_link.min_sdk_version(min_sdk_version);
    }

    aapt2_link.run()?;
    Ok(())
}