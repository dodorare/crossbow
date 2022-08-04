use crate::{commands::android::*, error::*, types::*};
use android_manifest::AndroidManifest;
use std::path::{Path, PathBuf};

/// Generates minimal unsigned aab
pub fn gen_minimal_unsigned_aab(
    sdk: AndroidSdk,
    package_name: &str,
    target_sdk_version: u32,
    aab_build_dir: &Path,
) -> Result<PathBuf> {
    let mut manifest = AndroidManifest {
        package: "com.crossbow.minimal".to_owned(),
        ..Default::default()
    };
    update_android_manifest_with_default(
        &mut manifest,
        Some("Minimal".to_owned()),
        "minimal",
        AndroidStrategy::NativeAab,
    );

    let manifest_path = save_android_manifest(aab_build_dir, &manifest)?;
    let apk_path = aab_build_dir.join(format!("{}_module.apk", package_name));
    if !aab_build_dir.exists() {
        std::fs::create_dir_all(&aab_build_dir)?;
    }

    let mut aapt2_link = sdk
        .aapt2()?
        .link_compiled_res(None, &apk_path, &manifest_path);
    aapt2_link
        .android_jar(sdk.android_jar(target_sdk_version)?)
        .version_code(1)
        .proto_format(true)
        .auto_add_overlay(true);
    aapt2_link.run()?;

    let output_dir = aab_build_dir.join("extracted_apk_files");
    let extracted_apk_path = extract_archive(&apk_path, &output_dir)?;

    let gen_zip_modules = super::gen_zip_modules(aab_build_dir, package_name, &extracted_apk_path)?;

    let aab_path =
        super::gen_aab_from_modules(package_name, &[gen_zip_modules.clone()], aab_build_dir)?;

    remove(vec![gen_zip_modules, extracted_apk_path])?;
    Ok(aab_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_minimal_unsigned_aab() {
        // Creates a temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let aab_build_dir = tempdir.path();

        // Assigns configuration for aab generation
        let sdk = AndroidSdk::from_env().unwrap();
        let package_name = "minimal_unsigned_aab";
        let target_sdk_version = 30;

        // Generates minimal unsigned aab
        gen_minimal_unsigned_aab(sdk, package_name, target_sdk_version, aab_build_dir).unwrap();
    }
}
