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
    project_path: &Path,
    build_dir: &Path,
    sdk: &AndroidSdk,
    package_label: &str,
    manifest_path: &Path,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let compiled_res = build_dir.join("compiled_res");
    Aapt2Compile::new(res_path, &compiled_res).run()?;

    let apk_path = build_dir
        .join(format!("{}_module.apk", package_label))
        .as_path();
    Aapt2Link::new(&[compiled_res], apk_path, manifest_path)
        .i(sdk.android_jar(target_sdk_version)?)
        .version_code(1)
        .proto_format(true)
        .auto_add_overlay(true)
        .run()?;
    if let Some(assets_path) = &assets_path {
        todo!()
    }

    let extracted_apk_files = build_dir.join("extracted_apk_files").as_path();
    extract_apk::extract_apk(apk_path, extracted_apk_files).unwrap();

    let zip_path = build_dir
        .join(format!("{}_module.zip", package_label))
        .as_path();
    write_zip::dirs_to_write(&extracted_apk_files.to_owned())?;
    write_zip::write(&extracted_apk_files.to_owned(), zip_path).unwrap();
    Ok(zip_path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aab() {
        // Create temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path();
        let package_name = gen_minimal_project(&dir).unwrap();

        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        let target_sdk_version = 30;
        let profile = Profile::Release;
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        android::compile_rust_for_android(
            &ndk,
            Target::Lib,
            build_target,
            &dir,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
        )
        .unwrap();
        let out_dir = dir
            .join("target")
            .join(build_target.rust_triple())
            .join(profile.as_ref());
        let compiled_lib = out_dir.join(format!("lib{}.so", package_name));
        assert!(compiled_lib.exists());

        // Generate manifest
        let target_dir = dir.join("target");
        let manifest = android::gen_minimal_android_manifest(
            &package_name,
            None,
            "0.0.1".to_string(),
            Some("1".to_string()),
            None,
            target_sdk_version,
            None,
            None,
        );
        let aab_build_dir = target_dir.join("android").join(&profile);
        let manifest_path = android::save_android_manifest(&aab_build_dir, &manifest).unwrap();
        assert!(manifest_path.exists());

        // Gen apks and prepare modules (zip, zip, zip)
        // let base_apk_path = android::gen_base_apk(
        //     &[Path::new("res\\mipmap\\Screenshot_2.png").to_owned()],
        //     Path::new("res\\mipmap\\"),
        //     &aab_build_dir,
        //     &sdk,
        //     &[Path::new("res\\mipmap\\mipmap_Screenshot_2.png.flat").to_owned()],
        //     Path::new("res\\mipmap\\test.apk"),
        //     &manifest_path,
        //     30,
        //     &[Path::new("res\\base.zip").to_owned()],
        //     Path::new("res\\mipmap\\test.aab"),
        //     Path::new("res\\extracted_files\\"),
        //     Path::new("res\\base.zip"),
        // )
        // .unwrap();

        // Assign path to lib
        // android::add_lib_aapt2(Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\lib\\"),
        // Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\extracted_files\\")).unwrap();

        // android::add_libs_into_aapt2(
        //     &ndk,
        //     &compiled_lib,
        //     build_target,
        //     profile,
        //     30,
        //     &aab_build_dir,
        //     &target_dir,
        // )
        // .unwrap();

        // let base_aab_module = android::gen_base_aab_module().unwrap();

        // Gen aab from given list of modules (zip, zip, zip)
        // let aab_path = android::gen_aab_from_modules(&[base_aab_module], &aab_build_dir).unwrap();

        // Create keystore with keytool command
        android::gen_debug_key_aab(Path::new("res\\mipmap\\"), "devtools".to_string()).unwrap();
        android::jarsigner(
            Path::new("res\\mipmap\\keystore"),
            Path::new("res\\mipmap\\test.aab"),
            "devtools".to_string(),
        )
        .unwrap();
        android::verify_aab(Path::new("res\\mipmap\\test.aab")).unwrap();
    }
}
