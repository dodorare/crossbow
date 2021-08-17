use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use crate::{
    commands::{android, gen_minimal_project},
    tools::AndroidSdk,
    types::*,
};
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
    zip_path: &Path,
) -> Result<()> {
    Aapt2Compile::new(inputs_compile, o_compile).run()?;

    Aapt2Link::new(inputs_link, o_link, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .proto_format(true)
        .auto_add_overlay(true)
        .run()?;

    extract_apk::extract_apk(o_link, extracted_apk).unwrap();

    write_zip::dirs_to_write(&extracted_apk.to_owned())?;
    write_zip::write(&extracted_apk.to_owned(), zip_path).unwrap();

    BuildBundle::new(modules, save_aab).run()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let sdk = AndroidSdk::from_env().unwrap();
        gen_aab(
            &[Path::new("res\\mipmap\\Screenshot_2.png").to_owned()],
            Path::new("res\\mipmap\\"),
            &sdk,
            &[Path::new("res\\mipmap\\mipmap_Screenshot_2.png.flat").to_owned()],
            Path::new("res\\mipmap\\test.apk"),
            Path::new("res\\mipmap\\AndroidManifest.xml"),
            30,
            &[Path::new("res\\test\\base.zip").to_owned()],
            Path::new("res\\mipmap\\test.aab"),
            Path::new("res\\extracted_files\\"),
            Path::new("res\\test\\base.zip"),
        )
        .unwrap();
    }

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
        let apk_build_dir = target_dir.join(&profile).join("aab");
        let manifest_path = android::save_android_manifest(&apk_build_dir, &manifest).unwrap();
        assert!(manifest_path.exists());

        // Assign path to lib
        android::add_lib_aapt2(Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\lib\\"), 
        Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\extracted_files\\")).unwrap();

        android::add_libs_into_aapt2(
            &ndk,
            &compiled_lib,
            build_target,
            profile,
            30,
            &apk_build_dir,
            &target_dir,
        )
        .unwrap();

        // Gen unaligned aab
        android::gen_aab(
            &[Path::new("res\\mipmap\\Screenshot_2.png").to_owned()],
            Path::new("res\\mipmap\\"),
            &sdk,
            &[Path::new("res\\mipmap\\mipmap_Screenshot_2.png.flat").to_owned()],
            Path::new("res\\mipmap\\test.apk"),
            &manifest_path,
            30,
            &[Path::new("res\\base.zip").to_owned()],
            Path::new("res\\mipmap\\test.aab"),
            Path::new("res\\extracted_files\\"),
            Path::new("res\\base.zip"),
        )
        .unwrap();

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
